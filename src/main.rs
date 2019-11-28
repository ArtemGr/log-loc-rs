// [build] cargo build
// [build] cd ../../komodoDEX && ../log-loc-rs/target/debug/log-loc -m kmd
// # [build] cargo install --path .. --force
// # [build] cd ../../komodoDEX && log-loc -m kmd

#[macro_use] extern crate gstuff;

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
  verbose: bool}

fn komodo_flutter() -> Result<(), String> {
  ERR! ("TBD")}

fn main() {
  let opt = Opt::from_args();
  match opt.mode {
    Mode::KomodoFlutter => komodo_flutter().unwrap()}}
