mod lexer;
mod parser;
mod symbols;
mod value;

use crate::lexer::tokenize;
use crate::parser::parse;
use crate::value::Format;
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

  /// Output format [all|bin|hex|oct|alfred]
  #[arg(short = 'o')]
  format: Option<String>,
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

fn output_err(err: String, format: Format) {
  match format {
    Format::Alfred => {
      println!(
        "\
<?xml version=\"1.0\"?>
<items>
  <item arg=\"{0}\" valid=\"NO\" autocomplete=\"{0}\" type=\"default\">
    <title>{0}</title>
    <subtitle><![CDATA[{1}]]></subtitle>
  </item>
</items>",
        "...", err
      )
    }
    _ => eprintln!("{}", err),
  }
}

fn main() {
  let mut format = Format::Default;
  let opts = Opts::parse();
  if opts.format.is_some() {
    let fmt = opts.format.clone().unwrap();
    format = match fmt.as_str() {
      "all" => Format::All,
      "bin" => Format::Binary,
      "hex" => Format::Hex,
      "oct" => Format::Octal,
      "alfred" => Format::Alfred,
      _ => {
        eprintln!("bad output format: {}", fmt);
        process::exit(1);
      }
    }
  }

  let program = read_input(&opts);
  let tokens = match tokenize(&program) {
    Ok(tokens) => tokens,
    Err(err) => {
      output_err(err, format);
      process::exit(1);
    }
  };

  let result = match parse(tokens) {
    Ok(value) => value,
    Err(err) => {
      output_err(err, format);
      process::exit(1);
    }
  };

  println!("{}", result.as_format_string(format));
}
