mod buffer;
mod lexer;
mod names;
mod operators;
mod parser;
mod value;

use crate::buffer::Buffer;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::value::Format;
use clap::Clap;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process;

pub const CONFIG_FILE: &str = ".clcrc";
pub const BUFFER_FILE: &str = ".clc_history";
pub const BUFFER_SIZE: u8 = 32;

#[derive(Clap, Debug)]
#[clap(name = "clc", version = "1.0")]
pub struct Opts {
  /// Set the max buffer size.
  #[clap(short, long, value_name = "SIZE", default_value = "32")]
  buffer_size: u8,

  /// Specify an alternate buffer file.
  #[clap(short = 'B', long, value_name = "FILE")]
  buffer_file: Option<String>,

  /// Read program from file.
  #[clap(short, long)]
  file: Option<String>,

  /// Expression to evaluate
  #[clap(short, long = "expr", conflicts_with = "file")]
  expression: Option<String>,

  /// Output format [all|bin|hex|oct|alfred]
  #[clap(short = 'o')]
  format: Option<String>,
}

fn read_program(opts: &Opts) -> String {
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
  } else if opts.expression.is_some() {
    program = opts.expression.clone().unwrap();
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
  let mut opts = Opts::parse();
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

  let home = env::var("HOME").map_or(String::from(""), |p| p);
  if opts.buffer_file.is_none() {
    opts.buffer_file = Some(String::from(Path::new(&home).join(BUFFER_FILE).to_str().unwrap()));
  }

  let mut buffer = match Buffer::create(&opts) {
    Ok(b) => b,
    Err(err) => {
      output_err(format!("failed to read buffer: {}", err), format);
      process::exit(1);
    }
  };

  let program = read_program(&opts);
  // println!("{:?}", opts);
  // println!("{}", program);
  let tokens = match tokenize(program.as_bytes()) {
    Ok(tokens) => tokens,
    Err(err) => {
      output_err(err, format);
      process::exit(1);
    }
  };

  let result = match parse(&tokens, &mut buffer) {
    Ok(value) => value,
    Err(err) => {
      buffer.save();
      output_err(err, format);
      process::exit(1);
    }
  };

  buffer.save();
  println!("{}", result.as_format_string(format));
}
