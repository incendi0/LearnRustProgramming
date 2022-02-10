use std::{collections::HashMap, fmt::Debug};

#[derive(PartialEq)]
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
        match *self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(v) => Debug::fmt(&v, f),
            JsonValue::Number(v) => Debug::fmt(&v, f),
            JsonValue::String(ref v) => Debug::fmt(v, f),
            JsonValue::Array(ref v) => Debug::fmt(v, f),
            JsonValue::Object(ref v) => Debug::fmt(v, f),
        }
    }
}
