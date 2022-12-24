use float_cmp::approx_eq;
use std::cmp::Ordering;
use std::{fmt, fmt::Display};

macro_rules! do_cast {
  ($v: expr, $w: expr, $t: tt) => {
    match $w {
      Width::U64 => ($v as u64) as $t,
      Width::U32 => ($v as u32) as $t,
      Width::U16 => ($v as u16) as $t,
      Width::U8 => ($v as u8) as $t,
      Width::I64 => ($v as i64) as $t,
      Width::I32 => ($v as i32) as $t,
      Width::I16 => ($v as i16) as $t,
      Width::I8 => ($v as i8) as $t,
    }
  };
}

#[rustfmt::skip]
macro_rules! cmp {
  ($v1: expr, $v2: expr, $w: expr) => {
    match $w {
      Width::U64 => (($v1 as u64).cmp(&($v2 as u64))), 
      Width::U32 => (($v1 as u32).cmp(&($v2 as u32))), 
      Width::U16 => (($v1 as u16).cmp(&($v2 as u16))), 
      Width::U8 => (($v1 as u8).cmp(&($v2 as u8))),
      Width::I64 => (($v1 as i64).cmp(&($v2 as i64))), 
      Width::I32 => (($v1 as i32).cmp(&($v2 as i32))),
      Width::I16 => (($v1 as i16).cmp(&($v2 as i16))),
      Width::I8 => (($v1 as i8).cmp(&($v2 as i8))),
    }
  };
}

#[rustfmt::skip]
macro_rules! value_fmt {
  ($v: expr, $w: expr, $f: expr) => {
    match $w {
      Width::U64 => format!($f, $v as u64), 
      Width::U32 => format!($f, $v as u32), 
      Width::U16 => format!($f, $v as u16),
      Width::U8 => format!($f, $v as u8),
      Width::I64 => format!($f, $v as i64), 
      Width::I32 => format!($f, $v as i32),
      Width::I16 => format!($f, $v as i16), 
      Width::I8 => format!($f, $v as i8),
    }
  };
}

macro_rules! impl_arithmetic_op {
  ($ops: tt, $func: tt, $op: tt) => {
    impl std::ops::$ops<Value> for Value {
      type Output = Value;
      fn $func(self, rhs: Value) -> Value {
        return match self {
          Value::Integer(v1, w) => match rhs {
            Value::Integer(v2, _) => Value::Integer((v1 $op v2) & w.mask(), w),
            Value::Float(v2) => Value::Integer((v1 $op (v2 as u64)), w),
          },
          Value::Float(v1) => match rhs {
            Value::Integer(v2, _) => Value::Float(v1 $op (v2 as f64)),
            Value::Float(v2) => Value::Float(v1 $op v2),
          },
        };
      }
    }
  };
}

macro_rules! impl_bitwise_op {
  ($ops: tt, $func: tt, $op: tt) => {
    impl std::ops::$ops<Value> for Value {
      type Output = Value;
      fn $func(self, rhs: Value) -> Value {
        return match self {
          Value::Integer(v1, w) => match rhs {
            Value::Integer(v2, _) => Value::Integer((v1 $op v2) & w.mask(), w),
            Value::Float(v2) => Value::Integer((v1 $op (v2 as u64)) & w.mask(), w),
          },
          Value::Float(_) => match rhs {
            Value::Integer(_, _) => Value::Float(f64::NAN),
            Value::Float(_) => Value::Float(f64::NAN),
          },
        };
      }
    }
  };
}

macro_rules! impl_value_from {
  ($t: tt) => {
    impl From<$t> for Value {
      fn from(v: $t) -> Self {
        return Value::Float(v as f64);
      }
    }
  };
  ($t: tt, $w: expr) => {
    impl From<$t> for Value {
      fn from(v: $t) -> Self {
        return Value::Integer(v as u64, $w);
      }
    }
  };
}

macro_rules! impl_from_value {
  ($t: tt) => {
    impl From<Value> for $t {
      fn from(src: Value) -> $t {
        return match src {
          Value::Integer(v, w) => do_cast!(v, w, $t),
          Value::Float(v) => v as $t,
        };
      }
    }
  };
}

//

#[derive(Debug, Copy, Clone)]
pub enum Value {
  Integer(u64, Width),
  Float(f64),
}

// `From` traits
impl_value_from!(u64, Width::U64);
impl_value_from!(u32, Width::U32);
impl_value_from!(u16, Width::U16);
impl_value_from!(u8, Width::U8);

impl_value_from!(i64, Width::I64);
impl_value_from!(i32, Width::I32);
impl_value_from!(i16, Width::I16);
impl_value_from!(i8, Width::I8);

impl_value_from!(bool, Width::U8);
impl_value_from!(f64);

impl_from_value!(u64);
impl_from_value!(u32);
impl_from_value!(u16);
impl_from_value!(u8);

impl_from_value!(i64);
impl_from_value!(i32);
impl_from_value!(i16);
impl_from_value!(i8);

impl_from_value!(f64);

impl From<Value> for bool {
  fn from(src: Value) -> bool {
    return match src {
      Value::Integer(v, _) => v != 0,
      Value::Float(v) => v != 0f64,
    };
  }
}

impl_arithmetic_op!(Add, add, +);
impl_arithmetic_op!(Sub, sub, -);
impl_arithmetic_op!(Mul, mul, *);
impl_arithmetic_op!(Div, div, /);
impl_arithmetic_op!(Rem, rem, %);

impl_bitwise_op!(BitAnd, bitand, &);
impl_bitwise_op!(BitOr, bitor, |);
impl_bitwise_op!(BitXor, bitxor, ^);
impl_bitwise_op!(Shl, shl, <<);
impl_bitwise_op!(Shr, shr, >>);

impl std::ops::Neg for Value {
  type Output = Value;
  fn neg(self) -> Value {
    return match self {
      Value::Integer(v, w) => Value::Integer(v.wrapping_neg() & w.mask(), w),
      Value::Float(v) => Value::Float(-v),
    };
  }
}
impl std::ops::Not for Value {
  type Output = Value;
  fn not(self) -> Value {
    return match self {
      Value::Integer(v, w) => Value::Integer(!v & w.mask(), w),
      Value::Float(v) => Value::Integer((v != 0f64) as u64, Width::U8),
    };
  }
}

// Comparison traits
impl Eq for Value {}

impl Ord for Value {
  fn cmp(&self, other: &Self) -> Ordering {
    return match self {
      Value::Integer(v1, w) => match other {
        Value::Integer(v2, _) => cmp!(*v1, *v2, w),
        Value::Float(v2) => cmp!(*v1, *v2, w),
      },
      Value::Float(v1) => match other {
        Value::Integer(v2, w) => {
          let vf = do_cast!(*v2, w, f64);
          if approx_eq!(f64, *v1, vf) {
            Ordering::Equal
          } else if v1 < &vf {
            Ordering::Less
          } else {
            Ordering::Greater
          }
        }
        Value::Float(v2) => {
          if approx_eq!(f64, *v1, *v2) {
            Ordering::Equal
          } else if v1 < &v2 {
            Ordering::Less
          } else {
            Ordering::Greater
          }
        }
      },
    };
  }
}

impl PartialEq<Value> for Value {
  fn eq(&self, other: &Value) -> bool {
    self.cmp(other) == Ordering::Equal
  }
}

impl PartialOrd<Value> for Value {
  fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Value {
  pub fn as_string(&self) -> String {
    match self {
      Value::Integer(v, w) => value_fmt!(*v, w, "{}"),
      Value::Float(v) => format!("{}", v),
    }
  }

  pub fn as_format_string(&self, format: Format) -> String {
    match self {
      Value::Integer(v, w) => match format {
        Format::Default => value_fmt!(*v, w, "{}"),
        Format::Alfred => generate_alfred_output(*self),
        Format::Binary => value_fmt!(*v, w, "{:#b}"),
        Format::Hex => value_fmt!(*v, w, "{:#x}"),
        Format::Octal => value_fmt!(*v, w, "{:#o}"),
      },
      Value::Float(v) => match format {
        Format::Alfred => generate_alfred_output(*self),
        _ => format!("{}", v),
      },
    }
  }

  pub fn as_typed_string(&self) -> String {
    match self {
      Value::Integer(_, w) => format!("{} {}", w.as_string(), self.as_string()),
      Value::Float(v) => format!("f64 {}", v),
    }
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

//

impl Value {
  pub fn is_signed(&self) -> bool {
    use Width::*;
    match self {
      Value::Integer(_, w) => match w {
        U64 | U32 | U16 | U8 => false,
        _ => true,
      },
      Value::Float(_) => true,
    }
  }

  pub fn to_signed(&self) -> Value {
    use Width::*;
    match self {
      Value::Integer(v, w) => {
        let new_width = match w {
          U64 => I64,
          U32 => I64,
          U16 => I16,
          U8 => I8,
          _ => *w,
        };

        Value::Integer(*v, new_width)
      }
      Value::Float(v) => Value::Float(*v),
    }
  }
}

#[derive(Clone, Copy)]
pub enum Format {
  Default,
  Alfred,
  Binary,
  Hex,
  Octal,
}

#[derive(Debug, Clone, Copy)]
pub enum Width {
  U64,
  U32,
  U16,
  U8,

  I64,
  I32,
  I16,
  I8,
}

impl Width {
  pub fn mask(&self) -> u64 {
    use Width::*;
    match self {
      U64 => 0xFFFFFFFFFFFFFFFF,
      U32 => 0xFFFFFFFF,
      U16 => 0xFFFF,
      U8 => 0xFF,

      I64 => 0xFFFFFFFFFFFFFFFF,
      I32 => 0xFFFFFFFF,
      I16 => 0xFFFF,
      I8 => 0xFF,
    }
  }

  pub fn as_string(&self) -> &str {
    use Width::*;
    match self {
      U64 => "u64",
      U32 => "u32",
      U16 => "u16",
      U8 => "u8",

      I64 => "i64",
      I32 => "i32",
      I16 => "i16",
      I8 => "i8",
    }
  }
}

impl Display for Width {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

// Alfred workflow xml output
fn generate_alfred_output(value: Value) -> String {
  match value {
    Value::Integer(_, _) => {
      let expr = value.as_format_string(Format::Default);
      let expr_hex = value.as_format_string(Format::Hex);
      let expr_oct = value.as_format_string(Format::Octal);
      let expr_bin = value.as_format_string(Format::Binary);
      format!(
        "
<?xml version=\"1.0\"?>
<items>
  <item arg=\"{expr}\" valid=\"YES\" autocomplete=\"{expr}\" type=\"default\">
    <title>{expr}</title>
    <subtitle>copy+paste as \"{expr}\"</subtitle>
    <mod key=\"shift\" subtitle=\"copy+paste as &quot;{expr}&quot;\" valid=\"yes\" arg=\"{expr}\"/>
    <icon>dec.png</icon>
  </item>
  <item arg=\"{hexstr}\" valid=\"YES\" autocomplete=\"{hexstr}\" type=\"default\">
    <title>{hexstr}</title>
    <subtitle>copy+paste as \"{hexstr}\"</subtitle>
    <mod key=\"shift\" subtitle=\"copy+paste as &quot;{hexstr}&quot;\" valid=\"yes\" arg=\"{hexstr}\"/>
    <icon>hex.png</icon>
  </item>
  <item arg=\"{octstr}\" valid=\"YES\" autocomplete=\"{octstr}\" type=\"default\">
    <title>{octstr}</title>
    <subtitle>copy+paste as \"{octstr}\"</subtitle>
    <mod key=\"shift\" subtitle=\"copy+paste as &quot;{octstr}&quot;\" valid=\"yes\" arg=\"{octstr}\"/>
    <icon>oct.png</icon>
  </item>
  <item arg=\"{binstr}\" valid=\"YES\" autocomplete=\"{binstr}\" type=\"default\">
    <title>{binstr}</title>
    <subtitle>copy+paste as \"{binstr}\"</subtitle>
    <mod key=\"shift\" subtitle=\"copy+paste as &quot;{binstr}&quot;\" valid=\"yes\" arg=\"{binstr}\"/>
    <icon>bin.png</icon>
  </item>
</items>",
        expr = expr,
        hexstr = expr_hex,
        octstr = expr_oct,
        binstr = expr_bin,
      )
    }
    Value::Float(v) => {
      format!(
        "\
<?xml version=\"1.0\"?>
<items>
  <item arg=\"{expr}\" valid=\"YES\" autocomplete=\"{expr}\" type=\"default\">
    <title>{expr}</title>
    <subtitle>copy+paste as \"{expr}\"</subtitle>
    <mod key=\"shift\" subtitle=\"copy+paste as &quot;{expr}&quot;\" valid=\"yes\" arg=\"{expr}\"/>
    <icon>dec.png</icon>
  </item>
</items>",
        expr = v,
      )
    }
  }
}
