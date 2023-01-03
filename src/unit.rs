use crate::number::Number;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
  Raw, // no unit
  // digital size
  Byte,
  Kilobyte,
  Megabyte,
  Gigabyte,
  Terabyte,
  Petabyte,
  // temperature
  Celsius,
  Fahrenheit,
  Kelvin,
}

impl Unit {
  pub fn is_raw(&self) -> bool {
    matches!(self, Unit::Raw)
  }

  pub fn is_size(&self) -> bool {
    matches!(
      self,
      Unit::Byte | Unit::Kilobyte | Unit::Megabyte | Unit::Gigabyte | Unit::Terabyte | Unit::Petabyte
    )
  }

  pub fn group(&self) -> &'static str {
    match self {
      Unit::Raw => "raw",
      Unit::Byte | Unit::Kilobyte | Unit::Megabyte | Unit::Gigabyte | Unit::Terabyte | Unit::Petabyte => "size",
      Unit::Celsius | Unit::Fahrenheit | Unit::Kelvin => "temperature",
    }
  }

  /// Normalizes a number to the base unit of the given unit (e.g. 1 kilobyte -> 1024 bytes).
  /// Not all units are normalized to bytes, such is the case when the unit is in a mixed
  /// unit system category (e.g. temperature).
  pub fn normalize(number: Number, from: Unit) -> Number {
    match from {
      // size (base unit is bytes)
      Unit::Byte => number.to_unsigned(),
      Unit::Kilobyte => (number * Number::from(1024u64)).to_unsigned(),
      Unit::Megabyte => (number * Number::from(1024u64.pow(2))).to_unsigned(),
      Unit::Gigabyte => (number * Number::from(1024u64.pow(3))).to_unsigned(),
      Unit::Terabyte => (number * Number::from(1024u64.pow(4))).to_unsigned(),
      Unit::Petabyte => (number * Number::from(1024u64.pow(5))).to_unsigned(),
      Unit::Celsius | Unit::Fahrenheit | Unit::Kelvin => number.to_float(),
      _ => number,
    }
  }

  /// Specializes a number to the given unit (e.g. 1024 bytes -> 1 kilobyte).
  /// This makes the number suitable for display.
  pub fn specialize(number: Number, to: Unit) -> Number {
    match to {
      // size (base unit is bytes)
      Unit::Byte => number.to_unsigned(),
      Unit::Kilobyte => number.to_float() / Number::from(1024u64),
      Unit::Megabyte => number.to_float() / Number::from(1024u64.pow(2)),
      Unit::Gigabyte => number.to_float() / Number::from(1024u64.pow(3)),
      Unit::Terabyte => number.to_float() / Number::from(1024u64.pow(4)),
      Unit::Petabyte => number.to_float() / Number::from(1024u64.pow(5)),
      _ => number,
    }
  }

  pub fn convert(value: Number, from: Unit, to: Unit) -> Option<Number> {
    // if from != to {
    //   println!("converting from {:#?} to {:#?}", from, to);
    // }

    match (from, to) {
      (Unit::Raw, Unit::Raw) => Some(value),
      (Unit::Raw, _) => Some(Self::normalize(value, to)),
      (_, Unit::Raw) => Some(Self::normalize(value, from)),
      (a, b) if a == b => Some(value),
      // size (all stored as bytes)
      (a, b) if a.is_size() && b.is_size() => Some(value),
      // temperature
      (Unit::Celsius, Unit::Fahrenheit) => Some(value.to_float() * Number::from(9f64 / 5f64) + Number::from(32f64)),
      (Unit::Celsius, Unit::Kelvin) => Some(value.to_float() + Number::from(273.15f64)),
      (Unit::Fahrenheit, Unit::Celsius) => Some((value.to_float() - Number::from(32f64)) * Number::from(5f64 / 9f64)),
      (Unit::Fahrenheit, Unit::Kelvin) => {
        Some((value.to_float() - Number::from(32f64)) * Number::from(5f64 / 9f64) + Number::from(273.15f64))
      }
      (Unit::Kelvin, Unit::Celsius) => Some(value.to_float() - Number::from(273.15f64)),
      (Unit::Kelvin, Unit::Fahrenheit) => {
        Some((value.to_float() - Number::from(273.15f64)) * Number::from(9f64 / 5f64) + Number::from(32f64))
      }
      _ => None,
    }
  }

  pub fn from_str(s: &str) -> Option<Unit> {
    match s {
      // size
      "B" => Some(Unit::Byte),
      "K" => Some(Unit::Kilobyte),
      "M" => Some(Unit::Megabyte),
      "G" => Some(Unit::Gigabyte),
      "T" => Some(Unit::Terabyte),
      "P" => Some(Unit::Petabyte),
      // temperature
      "°" | "°C" => Some(Unit::Celsius),
      "°F" => Some(Unit::Fahrenheit),
      "°K" => Some(Unit::Kelvin),
      _ => None,
    }
  }

  pub fn for_group(group: &str) -> Vec<Unit> {
    match group {
      "raw" => vec![Unit::Raw],
      "size" => vec![
        Unit::Byte,
        Unit::Kilobyte,
        Unit::Megabyte,
        Unit::Gigabyte,
        Unit::Terabyte,
        Unit::Petabyte,
      ],
      "temperature" => vec![Unit::Celsius, Unit::Fahrenheit, Unit::Kelvin],
      _ => vec![],
    }
  }
}

impl std::fmt::Display for Unit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Unit::Raw => write!(f, ""),
      // size
      Unit::Byte => write!(f, "B"),
      Unit::Kilobyte => write!(f, "K"),
      Unit::Megabyte => write!(f, "M"),
      Unit::Gigabyte => write!(f, "G"),
      Unit::Terabyte => write!(f, "T"),
      Unit::Petabyte => write!(f, "P"),
      // temperature
      Unit::Celsius => write!(f, "°C"),
      Unit::Fahrenheit => write!(f, "°F"),
      Unit::Kelvin => write!(f, "°K"),
    }
  }
}
