use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Index, IndexMut},
};

#[derive(PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl Debug for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn inner_fmt(
            json: &JsonValue,
            f: &mut std::fmt::Formatter,
            indent: &mut i32,
        ) -> std::fmt::Result {
            match *json {
                JsonValue::Null => write!(f, "null"),
                JsonValue::Bool(v) => Debug::fmt(&v, f),
                JsonValue::Number(v) => Debug::fmt(&v, f),
                JsonValue::String(ref v) => Debug::fmt(v, f),
                JsonValue::Array(ref v) => {
                    *indent += 1;
                    writeln!(f, "[")?;
                    for ov in v.iter() {
                        for _ in 0..*indent {
                            write!(f, "  ")?;
                        }
                        inner_fmt(ov, f, indent)?;
                        writeln!(f, ",")?;
                    }
                    *indent -= 1;
                    for _ in 0..*indent {
                        write!(f, "  ")?;
                    }
                    write!(f, "]")
                }
                JsonValue::Object(ref v) => {
                    *indent += 1;
                    writeln!(f, "{{")?;
                    for (ok, ov) in v {
                        for _ in 0..*indent {
                            write!(f, "  ")?;
                        }
                        Debug::fmt(ok, f)?;
                        write!(f, " : ")?;
                        inner_fmt(ov, f, indent)?;
                        writeln!(f, ",")?;
                    }
                    *indent -= 1;
                    for _ in 0..*indent {
                        write!(f, "  ")?;
                    }
                    write!(f, "}}")
                }
            }
        }
        let mut indent = 0;
        inner_fmt(self, f, &mut indent)
    }
}

impl IndexMut<&str> for JsonValue {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match *self {
            JsonValue::Object(ref mut inner) => {
                inner.entry(index.to_string()).or_insert(JsonValue::Null)
            }
            _ => panic!("cannot index string on JsonValue that is not JsonObject"),
        }
    }
}

impl Index<&str> for JsonValue {
    type Output = JsonValue;

    fn index(&self, index: &str) -> &Self::Output {
        match *self {
            JsonValue::Object(ref inner) => inner.get(index).unwrap_or(&JsonValue::Null),
            _ => &JsonValue::Null,
        }
    }
}

impl Index<usize> for JsonValue {
    type Output = JsonValue;

    fn index(&self, index: usize) -> &Self::Output {
        match *self {
            JsonValue::Array(ref inner) => inner.get(index).unwrap_or(&JsonValue::Null),
            _ => &JsonValue::Null,
        }
    }
}

impl IndexMut<usize> for JsonValue {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match *self {
            JsonValue::Array(ref mut inner) => {
                if inner.len() <= index {
                    inner.resize(index + 1, JsonValue::Null);
                }
                &mut inner[index]
            }
            _ => panic!("cannot index usize on JsonValue that is not JsonObject"),
        }
    }
}
