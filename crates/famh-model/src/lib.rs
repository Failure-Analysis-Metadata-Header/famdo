use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::str::FromStr;

pub mod v1;
pub mod v2;

pub type ExtraFields = BTreeMap<String, Value>;
pub type JsonMap = BTreeMap<String, Value>;

pub(crate) fn is_empty_map(map: &ExtraFields) -> bool {
    map.is_empty()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Numeric {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UnitValue<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

pub type NumberWithUnit = UnitValue<Numeric>;
pub type IntegerWithUnit = UnitValue<i64>;
pub type NumberArrayWithUnit = UnitValue<Vec<Option<Numeric>>>;
pub type PointArrayWithUnit = UnitValue<Vec<Vec<Option<Numeric>>>>;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MetadataDocument<T> {
    #[serde(flatten)]
    pub header: T,

    #[serde(rename = "_meta", default, skip_serializing_if = "is_empty_map")]
    pub metadata: ExtraFields,
}

impl<T> MetadataDocument<T> {
    pub fn new(header: T) -> Self {
        Self {
            header,
            metadata: ExtraFields::new(),
        }
    }

    pub fn with_metadata(header: T, metadata: ExtraFields) -> Self {
        Self { header, metadata }
    }
}

impl<T> MetadataDocument<T>
where
    T: DeserializeOwned,
{
    pub fn from_reader<R: Read>(reader: R) -> serde_json::Result<Self> {
        crate::from_reader(reader)
    }

    pub fn from_value(value: Value) -> serde_json::Result<Self> {
        crate::from_value(value)
    }
}

impl<T> FromStr for MetadataDocument<T>
where
    T: DeserializeOwned,
{
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl<T> MetadataDocument<T>
where
    T: Serialize,
{
    pub fn to_writer_pretty<W: Write>(&self, writer: W) -> serde_json::Result<()> {
        crate::to_writer_pretty(writer, self)
    }

    pub fn to_string_pretty(&self) -> serde_json::Result<String> {
        crate::to_string_pretty(self)
    }

    pub fn to_value(&self) -> serde_json::Result<Value> {
        crate::to_value(self)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn metadata_document_round_trips_with_reserved_meta_field() {
        let input = json!({
            "generalSection": {
                "fileName": "sample.tif"
            },
            "methodSpecific": {},
            "_meta": {
                "source": "connector",
                "runId": "abc-123"
            }
        });

        let document: MetadataDocument<crate::v2::FaMetadataHeader> =
            MetadataDocument::from_value(input.clone()).unwrap();

        assert_eq!(document.metadata.get("source"), Some(&json!("connector")));
        assert!(document.header.general_section.is_some());
        assert_eq!(document.to_value().unwrap(), input);
    }

    #[test]
    fn metadata_document_new_starts_without_extra_metadata() {
        let document = MetadataDocument::new(crate::v2::FaMetadataHeader::default());
        assert!(document.metadata.is_empty());
    }
}
