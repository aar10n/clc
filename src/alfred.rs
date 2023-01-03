use crate::value::{Unit, Value};

fn format_items(results: Vec<String>) -> String {
  let items = results
    .into_iter()
    .map(|result| {
      format!(
        r#"{{
        "arg": "{0}",
        "valid": "YES",
        "autocomplete": "{0}",
        "type": "default",
        "title": "{0}",
        "subtitle": "copy+paste as \"{0}\""
      }}"#,
        result
      )
    })
    .collect::<Vec<_>>();

  format!(r#"{{"items": [{}]}}"#, items.join(","))
}

pub fn alfred_result(value: Value) -> String {
  if value.is_raw() {
    let results = if value.is_integer() {
      vec![
        format!("{}", value.number),
        format!("{:#x}", value.number),
        format!("{:#o}", value.number),
        format!("{:#b}", value.number),
      ]
    } else {
      vec![format!("{}", value.number.as_pretty_string())]
    };
    format_items(results)
  } else {
    let units = Unit::for_group(value.unit.group());
    let results = units[..usize::min(units.len(), 4)]
      .iter()
      .map(|unit| format!("{}", value.convert(*unit).unwrap()))
      .collect::<Vec<_>>();
    format_items(results)
  }
}

pub fn alfred_error(err: String) -> String {
  format!(
    r#"{{"items": [
      {{
        "arg": "...",
        "valid": "NO",
        "autocomplete": "...",
        "type": "default",
        "title": "{0}",
        "subtitle": "..."
      }}
    ]}}"#,
    err
  )
}
