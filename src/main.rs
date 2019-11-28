#![feature(non_ascii_idents)]
#![feature(hash_raw_entry)]

#[macro_use] extern crate fomat_macros;
#[macro_use] extern crate gstuff;

mod kmd;

use glob::glob;
use gstuff::{now_ms, status_line, status_line_lm, status_line_lm0, slurp, with_status_line, ISATTY};
use structopt::StructOpt;
use std::borrow::Cow;
use std::collections::hash_map::{HashMap, RawEntryMut};
use std::fs;
use std::io::Write;
use std::str::FromStr;

#[derive(Debug, StructOpt)]
enum Mode {
  /// `Log.println('$loc', '...')`
  KomodoFlutter}

impl FromStr for Mode {
  type Err = String;
  fn from_str (s: &str) -> Result<Self, Self::Err> {
    match s {
      "KomodoFlutter" | "komodo-flutter" | "kmd" => Ok (Mode::KomodoFlutter),
      _ => ERR! ("Unknown mode: {}", s)}}}

#[derive(Debug, StructOpt)]
struct Opt {
  /// "komodo-flutter" or "kmd" .. patch `Log.println('$loc', '...')`
  #[structopt(short = "m", long, env = "LOG-LOC-MODE", hide_env_values = true)]
  mode: Mode,

  /// Discard the changes after finding and patching the logging statements.
  #[structopt(short = "d", long)]
  dry_run: bool,

  /// Show progress and patching highlights.
  #[structopt(short = "v", long)]
  verbose: bool,

  /// On terminals display a status line to track the search and patching progress.
  #[structopt(short = "s", long)]
  status: bool}

/// Patch `Log.println('$loc', '...')` in Flutter files (*.dart).
fn komodo_flutter (opt: Opt) -> Result<(), String> {
  macro_rules! status {($($args: tt)+) => {if opt.status && *ISATTY {
    if now_ms() - status_line_lm() > 333 {
      status_line ("log-loc", line!(), fomat! ($($args)+))}}}}
  macro_rules! verbose {($($args: tt)+) => {
    if opt.verbose {
      if opt.status && *ISATTY {
        with_status_line (&|| {pintln! ($($args)+)})
      } else {pintln! ($($args)+)}}}}

  status! ("Looking for dart files…");
  let mut files = Vec::new();
  let mut stem_cnt = HashMap::new();
  for path in try_s! (glob ("**/*.dart")) {
    let path = try_s! (path);
    if !path.is_file() {continue}
    let stem = match path.file_stem() {Some (n) => n, None => continue};
    let stem = match stem.to_str() {Some (n) => n, None => continue};
    match stem_cnt.raw_entry_mut().from_key (stem) {
      RawEntryMut::Occupied (mut oe) => *oe.get_mut() += 1,
      RawEntryMut::Vacant (ve) => {ve.insert (stem.to_string(), 1);}};
    let stem = stem.to_string();
    files.push ((path, stem))}

  let mut modified = 0;
  for ((path, stem), idx) in files.iter().zip (1..) {
    verbose! ((path.display()));
    status! ((idx) '/' (files.len()) ", " (modified) " modified, " (stem) '…');

    let bytes = slurp (&path);
    if bytes.is_empty() {continue}

    let els = try_s! (kmd::find_tags (&bytes));
    if els.len() == 1 {continue}  // A single `Source` chunk.

    // Create a new version of the file, replacing the tags.
    let mut buf = Vec::with_capacity (bytes.len() + 77);
    let mut line = 1;
    for el in els {
      match el {
        kmd::El::Source (bytes) => {
          line += bytes.iter().filter (|&&ch| ch == b'\n') .count();
          buf.extend_from_slice (bytes)},
        kmd::El::Tag (tag) => {
          buf.extend_from_slice (tag.head());

          // If the file name is unique then we use a short version of it
          // and if not then we use an unambiguous path.
          let name: Cow<str> =
            if let Some (1) = stem_cnt.get (stem) {stem.into()}
            else {fomat! ((path.display())) .into()};

          let _ = wite! (&mut buf, (name) ":" (line));
          buf.extend_from_slice (tag.tail());
          line += tag.head().iter().filter (|&&ch| ch == b'\n') .count();
          line += tag.tail().iter().filter (|&&ch| ch == b'\n') .count()}}}

    if buf != bytes {
      let tmpᵖ = path.with_extension ("dart.tmp");
      verbose! ("Writing to " (tmpᵖ.display()) '…');
      if !opt.dry_run {
        let mut tmp = try_s! (fs::File::create (&tmpᵖ));
        try_s! (tmp.write_all (&buf));
        drop (tmp);
        try_s! (fs::rename (tmpᵖ, path))}
      modified += 1}}

  status_line_lm0();
  status! ((files.len()) '/' (files.len()) ", " (modified) " modified.");
  Ok(())}

fn main() {
  let opt = Opt::from_args();
  match opt.mode {
    Mode::KomodoFlutter => komodo_flutter (opt) .unwrap()}}
