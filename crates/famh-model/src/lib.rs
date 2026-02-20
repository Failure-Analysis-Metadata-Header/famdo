use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::{Read, Write};

pub mod v1;
pub mod v2;

pub type ExtraFields = BTreeMap<String, Value>;

pub(crate) fn is_empty_map(map: &ExtraFields) -> bool {
    map.is_empty()
}

pub fn from_reader<T, R>(reader: R) -> serde_json::Result<T>
where
    T: DeserializeOwned,
    R: Read,
{
    serde_json::from_reader(reader)
}

pub fn from_str<T>(json: &str) -> serde_json::Result<T>
where
    T: DeserializeOwned,
{
    serde_json::from_str(json)
}

pub fn from_value<T>(value: Value) -> serde_json::Result<T>
where
    T: DeserializeOwned,
{
    serde_json::from_value(value)
}

pub fn to_writer_pretty<T, W>(writer: W, value: &T) -> serde_json::Result<()>
where
    T: Serialize,
    W: Write,
{
    serde_json::to_writer_pretty(writer, value)
}

pub fn to_string_pretty<T>(value: &T) -> serde_json::Result<String>
where
    T: Serialize,
{
    serde_json::to_string_pretty(value)
}

pub fn to_value<T>(value: &T) -> serde_json::Result<Value>
where
    T: Serialize,
{
    serde_json::to_value(value)
}
