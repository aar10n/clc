use crate::value::{Value, Width};
use crate::Opts;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::Write;
use std::str::FromStr;

lazy_static! {
  static ref RE_TYPED: Regex = Regex::new(r"^([iu](?:8|16|32|64)|f64) (-?(?:\d+|\d*\.\d+))$").unwrap();
  static ref RE_INT: Regex = Regex::new(r"^-?\d+$").unwrap();
  static ref RE_FLOAT: Regex = Regex::new(r"^-?\d*\.\d+$").unwrap();
}

#[derive(Debug)]
pub struct Buffer {
  filename: Option<String>,
  contents: Vec<Value>,
  max_size: usize,
  size: usize,
}

impl Buffer {
  pub fn create(opts: &Opts) -> Result<Buffer, std::io::Error> {
    let max_size = opts.buffer_size as usize;

    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(opts.buffer_file.clone().unwrap());

    let mut raw = String::new();
    match file.unwrap().read_to_string(&mut raw) {
      Ok(_) => (),
      Err(_) => {
        eprintln!("failed to read buffer file");
        return Ok(Buffer {
          filename: Some(opts.buffer_file.clone().unwrap()),
          contents: vec![],
          max_size,
          size: 0,
        });
      }
    }

    let mut count: usize = 0;
    let mut entries = vec![];
    for line in raw.split("\n") {
      if count >= max_size {
        break;
      }

      let value = Buffer::parse_line(line);
      match value {
        Some(val) => {
          entries.push(val);
          count += 1;
        }
        None => continue,
      }
    }

    Ok(Buffer {
      filename: Some(opts.buffer_file.clone().unwrap()),
      contents: entries,
      max_size,
      size: count,
    })
  }

  pub fn get(&self, i: usize) -> Value {
    if i > self.size {
      return Value::Integer(0, Width::U64);
    }
    self.contents[i]
  }

  pub fn add(&mut self, value: Value) {
    if self.size < self.max_size {
      self.contents.push(Value::Integer(0, Width::U64));
      self.contents.rotate_right(1);
      self.contents[0] = value;
      self.size += 1;
    } else {
      self.contents.rotate_right(1);
      self.contents[0] = value;
    }
  }

  pub fn save(&mut self) {
    let filename = self.filename.as_ref().unwrap();
    let file = File::create(filename);
    if file.is_err() {
      eprintln!("failed to save buffer");
      return;
    }

    let mut f = file.unwrap();
    f.set_len(0).unwrap();
    for value in self.contents.iter() {
      write!(f, "{}\n", value.as_typed_string()).unwrap();
    }
    f.flush().unwrap();
  }

  //

  fn parse_line(line: &str) -> Option<Value> {
    if line.is_empty() {
      return None;
    }

    let mut mat = RE_TYPED.captures(line);
    if mat.is_some() {
      // line is a typed value
      let groups = mat.unwrap();
      let type_str = groups.get(1)?;
      let value_str = groups.get(2)?;

      #[rustfmt::skip]
      return match type_str.as_str() {
        "u64" => Some(Value::Integer(u64::from_str(value_str.as_str()).ok()?, Width::U64)),
        "u32" => Some(Value::Integer(u32::from_str(value_str.as_str()).ok()? as u64, Width::U32)),
        "u16" => Some(Value::Integer(u16::from_str(value_str.as_str()).ok()? as u64, Width::U16)),
        "u8" => Some(Value::Integer(u8::from_str(value_str.as_str()).ok()? as u64, Width::U8)),

        "i64" => Some(Value::Integer(i64::from_str(value_str.as_str()).ok()? as u64, Width::I64)),
        "i32" => Some(Value::Integer(i32::from_str(value_str.as_str()).ok()? as u64, Width::I32)),
        "i16" => Some(Value::Integer(i16::from_str(value_str.as_str()).ok()? as u64, Width::I16)),
        "i8" => Some(Value::Integer(i8::from_str(value_str.as_str()).ok()? as u64, Width::I8)),

        "f64" => Some(Value::Float(f64::from_str(value_str.as_str()).ok()?)),

        _ => return None,
      };
    }

    mat = RE_INT.captures(line);
    if mat.is_some() {
      let groups = mat.unwrap();
      let value_str = groups.get(1)?;
      return Some(Value::Integer(u64::from_str(value_str.as_str()).ok()?, Width::U64));
    }

    mat = RE_FLOAT.captures(line);
    if mat.is_some() {
      let groups = mat.unwrap();
      let value_str = groups.get(1)?;
      return Some(Value::Float(f64::from_str(value_str.as_str()).ok()?));
    }

    return None;
  }
}
