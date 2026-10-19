//! Minimal JSON writer for the C and web APIs.

use std::fmt::Write;

pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(&'static str, Value)>),
}

impl Value {
    pub fn string(text: impl Into<String>) -> Self {
        Self::String(text.into())
    }

    pub fn opt_string(text: Option<impl Into<String>>) -> Self {
        text.map_or(Self::Null, |text| Self::String(text.into()))
    }

    pub fn to_json(&self) -> String {
        let mut out = String::new();
        self.write(&mut out);
        out
    }

    fn write(&self, out: &mut String) {
        match self {
            Self::Null => out.push_str("null"),
            Self::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
            Self::Number(value) if value.is_finite() => {
                let _ = write!(out, "{value}");
            }
            Self::Number(_) => out.push_str("null"),
            Self::String(text) => write_string(out, text),
            Self::Array(items) => {
                out.push('[');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    item.write(out);
                }
                out.push(']');
            }
            Self::Object(fields) => {
                out.push('{');
                for (index, (key, value)) in fields.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    write_string(out, key);
                    out.push(':');
                    value.write(out);
                }
                out.push('}');
            }
        }
    }
}

fn write_string(out: &mut String, text: &str) {
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_strings() {
        let value = Value::Object(vec![
            ("a", Value::string("x\"y\n")),
            ("b", Value::Array(vec![Value::Bool(true), Value::Null])),
        ]);
        assert_eq!(value.to_json(), r#"{"a":"x\"y\n","b":[true,null]}"#);
    }
}
