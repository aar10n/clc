use crate::value::{Number, Unit, Value};
use phf::phf_map;

#[derive(Copy, Clone, Debug)]
pub enum Function {
  Unary(fn(Value) -> Result<Value, String>),
  Binary(fn(Value, Value) -> Result<Value, String>),
}

/// A macro to define constant values.
macro_rules! constant {
  ($value:expr) => {
    || Value::new_raw(Number::from($value))
  };
}

/// A macro to define pure unary functions.
///
/// The macro wraps a given closure in `Function::Unary` and handles the conversion from `Value`
/// to the parameter type and back again. The closure must have the parameter explicitly typed
/// and should be `Number` or a primitive type convertible from `Number`. It should return a
/// value convertible to `Number`.
///
/// ## Examples
///
/// ```
/// unary!(|v: f64| v.sin())
/// unary!(|v: Number| f64::from(v).sin())
/// ```
macro_rules! unary {
  (|$param:ident: Value| $($rest:tt)*) => { unary!(_ Value |$param: Value| $($rest)*) };
  (|$param:ident: $type:ty| $($rest:tt)*) => { unary!(_ $type |$param: $type| $($rest)*) };
  (|$param:ident| $($rest:tt)*) => { unary!(_ Value |$param: Value| $($rest)*) };
  // internally invoked by the above
  (_ Value $callable:expr) => {
    Function::Unary(|v: Value| {
      Ok(Value::from($callable(v)))
    })
  };
  (_ $ty:tt $callable:expr) => {
    Function::Unary(|v: Value| {
      Ok(Value::from((Number::from($callable(<$ty>::from(v.number))), v.unit)))
    })
  };
}

/// A macro to define binary functions.
///
/// The macro wraps a given closure in `Function::Binary` and handles the conversion from `Value`
/// to the parameter type and back again. The closure must have the parameters explicitly typed
/// and both should be `Number` or a primitive type convertible from `Number`. It should return
/// a value convertible to `Number`.
///
/// ## Examples
///
/// ```
/// binary!(|v1: f64, v2: f64| v1 + v2)
/// binary!(|v1: Number, v2: Number| v1 + v2)
/// ```
macro_rules! binary {
  (|$p1:ident: Value, $p2:ident: Value| $($rest:tt)*) => { binary!(_ Value |$p1: Value, $p2: Value| $($rest)*) };
  (|$p1:ident: $t1:ty, $p2:ident: $t2:ty| $($rest:tt)*) => { binary!(_ $t1 $t2 |$p1: $t1, $p2: $t2| $($rest)*) };
  (|$p1:ident, $p2:ident| $($rest:tt)*) => { binary!(_ Value |$p1: Value, $p2: Value| $($rest)*) };
  // internally invoked by the above
  (_ Value $callable:expr) => {
    Function::Binary(|a: Value, b: Value| {
      Ok(Value::from($callable(a, b)))
    })
  };
  (_ $t1:tt $t2:tt $callable:expr) => {
    Function::Binary(|a: Value, b: Value| {
      let unit = a.unit;
      let a = <$t1>::from(a.number);
      let b = <$t2>::from(b.number);
      Ok(Value::from((Number::from($callable(a, b)), unit)))
    })
  };
}

/// A macro to define casting functions.
macro_rules! cast {
  ($type:ty) => {
    Function::Unary(|v: Value| Ok(Value::from((Number::from(<$type>::from(v.number)), Unit::Raw))))
  };
}

/// A macro to define conversion functions.
macro_rules! convert {
  // convert to a specific unit using any available conversion
  ($unit:expr) => {
    Function::Unary(|v: Value| {
      v.convert($unit)
        .ok_or(format!("Invalid conversion from {} to {}", v.unit, $unit))
    })
  };
  // convert to a specific unit from another given unit (or raw)
  ($from:expr => $to:expr) => {
    Function::Unary(|v: Value| {
      if v.unit == Unit::Raw {
        Ok(Unit::normalize(v.number, $to))
      } else if v.unit == $from {
        v.convert($unit)
          .ok_or(format!("Invalid conversion from {} to {}", v.unit, $unit))
      } else {
        Err(format!("Conversion from {} to {} is not supported", v.unit, $unit))
      }
    })
  };
}

//

const CONST_TABLE: phf::Map<&'static str, fn() -> Value> = phf_map! {
  "PI" => constant!(std::f64::consts::PI),
  "E" => constant!(std::f64::consts::E),
  "NAN" => constant!(f64::NAN),
  "INF" => constant!(f64::INFINITY),
  "NEG_INF" => constant!(f64::NEG_INFINITY),

  "F64_MIN" => constant!(f64::MIN),
  "F64_MAX" => constant!(f64::MAX),
  "U64_MIN" => constant!(u64::MIN),
  "U64_MAX" => constant!(u64::MAX),
  "U32_MIN" => constant!(u32::MIN),
  "U32_MAX" => constant!(u32::MAX),
  "U16_MIN" => constant!(u16::MIN),
  "U16_MAX" => constant!(u16::MAX),
  "U8_MIN" => constant!(u8::MIN),
  "U8_MAX" => constant!(u8::MAX),
  "I64_MIN" => constant!(i64::MIN),
  "I64_MAX" => constant!(i64::MAX),
  "I32_MIN" => constant!(i32::MIN),
  "I32_MAX" => constant!(i32::MAX),
  "I16_MIN" => constant!(i16::MIN),
  "I16_MAX" => constant!(i16::MAX),
  "I8_MIN" => constant!(i8::MIN),
  "I8_MAX" => constant!(i8::MAX),
};

const FUNC_TABLE: phf::Map<&'static str, Function> = phf_map! {
  // operators
  "+u" => unary!(|v: Number| v),
  "-u" => unary!(|v: Number| -v),
  "!u" => unary!(|v: bool| !v),
  "~u" => unary!(|v: Number| !v),

  "+" => binary!(|a: Number, b: Number| a + b),
  "-" => binary!(|a: Number, b: Number| a - b),
  "*" => binary!(|a: Number, b: Number| a * b),
  "/" => binary!(|a: Number, b: Number| a / b),
  "%" => binary!(|a: Number, b: Number| a % b),

  "&" => binary!(|a: Number, b: Number| a & b),
  "|" => binary!(|a: Number, b: Number| a | b),
  "^" => binary!(|a: Number, b: Number| a ^ b),

  "<<" => binary!(|a: Number, b: Number| a << b),
  ">>" => binary!(|a: Number, b: Number| a >> b),

  "<" => binary!(|a: Number, b: Number| a < b),
  ">" => binary!(|a: Number, b: Number| a > b),
  ">=" => binary!(|a: Number, b: Number| a >= b),
  "<=" => binary!(|a: Number, b: Number| a <= b),
  "==" => binary!(|a: Number, b: Number| a == b),
  "!=" => binary!(|a: Number, b: Number| a != b),

  "&&" => binary!(|a: bool, b: bool| a && b),
  "||" => binary!(|a: bool, b: bool| a || b),

  // casting
  "u64" => cast!(u64),
  "u32" => cast!(u32),
  "u16" => cast!(u16),
  "u8" => cast!(u8),
  "i64" => cast!(i64),
  "i32" => cast!(i32),
  "i16" => cast!(i16),
  "i8" => cast!(i8),
  "f64" => cast!(f64),

  // unit conversion
  "bytes" => convert!(Unit::Byte),
  "kilobyte" => convert!(Unit::Kilobyte),
  "megabyte" => convert!(Unit::Megabyte),
  "gigabyte" => convert!(Unit::Gigabyte),
  "terabyte" => convert!(Unit::Terabyte),
  "petabyte" => convert!(Unit::Petabyte),

  "celsius" => convert!(Unit::Celsius),
  "fahrenheit" => convert!(Unit::Fahrenheit),
  "kelvin" => convert!(Unit::Kelvin),

  // functions
  "abs" => unary!(|v: Number| v.abs()),
  "sin" => unary!(|v: f64| v.sin()),
  "cos" => unary!(|v: f64| v.cos()),
  "tan" => unary!(|v: f64| v.tan()),
  "asin" => unary!(|v: f64| v.asin()),
  "acos" => unary!(|v: f64| v.asin()),
  "atan" => unary!(|v: f64| v.asin()),
  "floor" => unary!(|v: f64| v.floor()),
  "ceil" => unary!(|v: f64| v.ceil()),
  "round" => unary!(|v: f64| v.round()),
  "sqrt" => unary!(|v: f64| v.sqrt()),
  "exp" => unary!(|v: f64| v.exp()),
  "ln" => unary!(|v: f64| v.ln()),
  "log2" => unary!(|v: f64| v.log2()),
  "log10" => unary!(|v: f64| v.log10()),
  "deg" => unary!(|v: f64| v / (std::f64::consts::FRAC_1_PI * 180.0)),
  "rad" => unary!(|v: f64| v * (std::f64::consts::FRAC_1_PI * 180.0)),
};

const ALIAS_TABLE: phf::Map<&'static str, &'static str> = phf_map! {
  "KiB" => "kilobyte",
  "MiB" => "megabyte",
  "GiB" => "gigabyte",
  "TiB" => "terabyte",
  "PiB" => "petabyte",

  "tempC" => "celsius",
  "tempF" => "fahrenheit",
  "tempK" => "kelvin",
};

pub fn get_constant(name: &str) -> Option<Value> {
  CONST_TABLE.get(name).map(|f| f())
}

pub fn get_function(name: &str) -> Option<Function> {
  FUNC_TABLE.get(name).map(|f| f.clone()).or_else(|| {
    ALIAS_TABLE
      .get(name)
      .and_then(|alias| FUNC_TABLE.get(alias))
      .map(|f| f.clone())
  })
}
