mod buffer;
mod config;
mod parser;
mod lexer;
mod value;
mod names;

use std::env;
use std::fs::File;
use std::fs;
use std::io::prelude::*;

use clap::{AppSettings, Clap};
use crate::lexer::tokenize;


pub const CONFIG_FILE: &str = "~/.clcrc";
pub const BUFFER_FILE: &str = "~/.clc_history";
pub const BUFFER_SIZE: u8 = 32;

#[derive(Clap, Debug)]
#[clap(name = "clc", version = "1.0")]
struct Opts {
  /// Set the max buffer size.
  #[clap(short, long, value_name = "SIZE", default_value = "32")]
  buffer_size: u8,

  /// Specify an alternate buffer file.
  #[clap(short = 'B', long, value_name = "FILE", default_value = CONFIG_FILE)]
  buffer_file: String,

  /// Specify an alternate rc file.
  #[clap(short, long, value_name = "FILE", default_value = BUFFER_FILE)]
  config: String,

  /// Read program from file.
  #[clap(short, long)]
  file: Option<String>,
}

fn main() {
  // let opts = Opts::parse();

  let program = "-$1 + 2 - sin(0.5)";
  let tokens = match tokenize(program.as_bytes()) {
    Ok(tokens) => tokens,
    Err(failure) => panic!(failure)
  };
  println!("{}", program);
  println!("{:?}", tokens);
}
