// [build] cargo build
// [build] cd ../../komodoDEX && ../log-loc-rs/target/debug/log-loc -vm kmd
// # [build] cargo install --path .. --force
// # [build] cd ../../komodoDEX && log-loc -m kmd

#![feature(non_ascii_idents)]

#[macro_use] extern crate fomat_macros;
#[macro_use] extern crate gstuff;

use glob::glob;
use gstuff::{now_ms, status_line, status_line_lm, slurp, with_status_line, ISATTY};
use structopt::StructOpt;
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
  let files: Vec<_> = try_s! (glob ("**/*.dart")) .collect();
  let filesⁿ = files.len();
  for (path, idx) in files.into_iter().zip (1..) {
    let path = try_s! (path);
    verbose! ((path.display()));
    if let Some (name) = path.file_name() {
      if let Some (name) = name.to_str() {
        status! ((idx) '/' (filesⁿ) ' ' (name) '…')}}
    let bytes = slurp (&path);
    if bytes.is_empty() {continue}
    
  }
  ERR! ("TBD")}

fn main() {
  let opt = Opt::from_args();
  match opt.mode {
    Mode::KomodoFlutter => komodo_flutter (opt) .unwrap()}}
