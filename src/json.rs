use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

impl JsonValue {
    pub fn as_object(&self) -> Option<&BTreeMap<String, JsonValue>> {
        match self {
            JsonValue::Object(map) => Some(map),
            _ => None,
        }
    }
}

impl From<bool> for JsonValue {
    fn from(b: bool) -> Self {
        JsonValue::Bool(b)
    }
}

impl From<i64> for JsonValue {
    fn from(i: i64) -> Self {
        JsonValue::Number(i as f64)
    }
}

impl From<u64> for JsonValue {
    fn from(u: u64) -> Self {
        JsonValue::Number(u as f64)
    }
}

impl From<usize> for JsonValue {
    fn from(u: usize) -> Self {
        JsonValue::Number(u as f64)
    }
}

impl From<f64> for JsonValue {
    fn from(f: f64) -> Self {
        JsonValue::Number(f)
    }
}

impl From<&str> for JsonValue {
    fn from(s: &str) -> Self {
        JsonValue::String(s.to_string())
    }
}

impl From<String> for JsonValue {
    fn from(s: String) -> Self {
        JsonValue::String(s)
    }
}

#[macro_export]
macro_rules! json {
    (null) => {
        $crate::json::JsonValue::Null
    };
    ({$($key:tt : $value:tt),* $(,)?}) => {{
        let mut map = std::collections::BTreeMap::new();
        $( map.insert($key.to_string(), $crate::json::json!($value)); )*
        $crate::json::JsonValue::Object(map)
    }};
    ([$($elem:tt),* $(,)?]) => {
        $crate::json::JsonValue::Array(vec![ $( $crate::json::json!($elem) ),* ])
    };
    ($other:expr) => {
        $crate::json::JsonValue::from($other)
    };
}
