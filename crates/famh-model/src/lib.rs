use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::{Read, Write};

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

impl From<i64> for Numeric {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Numeric {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
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

impl<T> UnitValue<T> {
    pub fn empty() -> Self {
        Self {
            value: None,
            unit: None,
            extra: ExtraFields::new(),
        }
    }

    pub fn new(value: T, unit: impl Into<String>) -> Self {
        Self {
            value: Some(value),
            unit: Some(unit.into()),
            extra: ExtraFields::new(),
        }
    }

    pub fn from_value(value: T) -> Self {
        Self {
            value: Some(value),
            unit: None,
            extra: ExtraFields::new(),
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}

impl UnitValue<Vec<Option<Numeric>>> {
    pub fn from_f64_values<I>(values: I, unit: impl Into<String>) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        let values = values
            .into_iter()
            .map(|value| Some(Numeric::Float(value)))
            .collect();
        Self::new(values, unit)
    }
}

impl UnitValue<Vec<Vec<Option<Numeric>>>> {
    pub fn from_f64_points<I, J>(points: I, unit: impl Into<String>) -> Self
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = f64>,
    {
        let points = points
            .into_iter()
            .map(|point| {
                point
                    .into_iter()
                    .map(|value| Some(Numeric::Float(value)))
                    .collect()
            })
            .collect();
        Self::new(points, unit)
    }
}

impl<T> From<T> for UnitValue<T> {
    fn from(value: T) -> Self {
        Self::from_value(value)
    }
}

impl<T, U> From<(T, U)> for UnitValue<T>
where
    U: Into<String>,
{
    fn from((value, unit): (T, U)) -> Self {
        Self::new(value, unit)
    }
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

    pub fn header(mut self, header: T) -> Self {
        self.header = header;
        self
    }

    pub fn metadata(mut self, metadata: ExtraFields) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn insert_metadata(mut self, key: impl Into<String>, value: Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
}

impl<T> MetadataDocument<T>
where
    T: DeserializeOwned,
{
    pub fn from_reader<R: Read>(reader: R) -> serde_json::Result<Self> {
        crate::from_reader(reader)
    }

    pub fn from_str(json: &str) -> serde_json::Result<Self> {
        crate::from_str(json)
    }

    pub fn from_value(value: Value) -> serde_json::Result<Self> {
        crate::from_value(value)
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

    #[test]
    fn test_numeric_from_primitive_values() {
        assert_eq!(Numeric::from(5_i64), Numeric::Integer(5));
        assert_eq!(Numeric::from(1.25_f64), Numeric::Float(1.25));
    }

    #[test]
    fn test_unit_value_builders_and_conversions() {
        let with_new = UnitValue::new(10_i64, "px");
        let with_tuple: UnitValue<i64> = (10_i64, "px").into();
        let with_chain = UnitValue::from_value(10_i64).with_unit("px");

        assert_eq!(with_new, with_tuple);
        assert_eq!(with_new, with_chain);
        assert_eq!(with_new.value, Some(10));
        assert_eq!(with_new.unit.as_deref(), Some("px"));
    }

    #[test]
    fn test_number_array_with_unit_from_f64_values() {
        let value = NumberArrayWithUnit::from_f64_values(vec![1.0, 2.5], "um");
        assert_eq!(
            value.value,
            Some(vec![Some(Numeric::Float(1.0)), Some(Numeric::Float(2.5))])
        );
        assert_eq!(value.unit.as_deref(), Some("um"));
    }

    #[test]
    fn test_point_array_with_unit_from_f64_points() {
        let value = PointArrayWithUnit::from_f64_points(vec![vec![1.0, 2.0], vec![3.0, 4.0]], "px");
        assert_eq!(
            value.value,
            Some(vec![
                vec![Some(Numeric::Float(1.0)), Some(Numeric::Float(2.0))],
                vec![Some(Numeric::Float(3.0)), Some(Numeric::Float(4.0))]
            ])
        );
        assert_eq!(value.unit.as_deref(), Some("px"));
    }
}
