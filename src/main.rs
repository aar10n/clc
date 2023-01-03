mod alfred;
mod functions;
mod lexer;
mod number;
mod parser;
mod unit;
mod value;

use crate::alfred::{alfred_error, alfred_result};
use crate::lexer::tokenize;
use crate::parser::parse;
use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use std::process;

#[derive(Parser, Debug)]
#[command(name = "clc", version = "1.1")]
pub struct Opts {
  /// Read expression from file.
  #[arg(short, long)]
  file: Option<String>,

  /// Expression to evaluate
  #[arg(short, long, conflicts_with = "file")]
  expr: Option<String>,

  /// Enables alfred JSON output
  #[arg(long)]
  alfred: bool,
}

fn read_input(opts: &Opts) -> String {
  let mut program = String::new();
  if opts.file.is_some() {
    let file = File::open(opts.file.clone().unwrap());
    match file {
      Ok(mut f) => {
        f.read_to_string(&mut program).unwrap();
      }
      Err(err) => {
        eprint!("{}", err);
        process::exit(1);
      }
    }
  } else if opts.expr.is_some() {
    program = opts.expr.clone().unwrap();
  } else {
    let stdin = io::stdin();
    match stdin.read_line(&mut program) {
      Ok(_) => (),
      Err(err) => {
        eprint!("{}", err);
        process::exit(1);
      }
    }
  }
  return program;
}

fn output_err(err: String, opts: &Opts) {
  if opts.alfred {
    println!("{}", alfred_error(err));
  } else {
    eprintln!("{}", err);
  }
}

fn main() {
  let opts = Opts::parse();
  let program = read_input(&opts);
  let tokens = match tokenize(&program) {
    Ok(tokens) => tokens,
    Err(err) => {
      output_err(err, &opts);
      process::exit(1);
    }
  };

  let result = match parse(tokens) {
    Ok(value) => value,
    Err(err) => {
      output_err(err, &opts);
      process::exit(1);
    }
  };

  if opts.alfred {
    println!("{}", alfred_result(result));
  } else {
    println!("{}", result);
  }
}
