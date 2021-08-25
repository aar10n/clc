use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Stdin;
use std::io::prelude::*;
use crate::{Opts, BUFFER_FILE, CONFIG_FILE};

pub enum InputSource {
  File(File),
  Stdin(Stdin),
  String(String),
  None(),
}

pub struct Config {
  /// config file
  pub config_file: Option<File>,
  /// buffer source
  pub buffer_src: InputSource,
  /// buffer size
  pub buffer_size: u8,
  /// program source
  pub program_src: InputSource
}


// pub fn load_config(opts: Opts) -> Result<Config, std::io::Error> {
//   let config_file = OpenOptions::new()
//       .read(true)
//       .create_new(opts.config == CONFIG_FILE)
//       .open(opts.config)?;
//
//
// }


