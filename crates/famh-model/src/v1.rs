use crate::{ExtraFields, is_empty_map};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FaMetadataHeader {
    #[serde(
        rename = "General Section",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub general_section: Option<Section>,

    #[serde(
        rename = "Method Specific",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub method_specific: Option<Section>,

    #[serde(
        rename = "Customer Section",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_section: Option<Section>,

    #[serde(
        rename = "Tool Specific",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tool_specific: Option<Section>,

    #[serde(
        rename = "Data Evaluation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_evaluation: Option<Section>,

    #[serde(rename = "History", default, skip_serializing_if = "Option::is_none")]
    pub history: Option<Section>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

impl FaMetadataHeader {
    pub fn from_reader<R: Read>(reader: R) -> serde_json::Result<Self> {
        crate::from_reader(reader)
    }

    pub fn from_str(json: &str) -> serde_json::Result<Self> {
        crate::from_str(json)
    }

    pub fn from_value(value: Value) -> serde_json::Result<Self> {
        crate::from_value(value)
    }

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

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Section {
    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub fields: ExtraFields,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_v1_top_level_sections() {
        let input = json!({
            "General Section": {"File Name": "sample.tif"},
            "Method Specific": {"Scanning Electron Microscopy": {}},
            "Custom Top Level": true
        });

        let model = FaMetadataHeader::from_value(input).unwrap();

        assert!(model.general_section.is_some());
        assert!(model.method_specific.is_some());
        assert_eq!(model.extra.get("Custom Top Level"), Some(&json!(true)));
    }

    #[test]
    fn round_trips_v1_unknown_fields() {
        let input = json!({
            "General Section": {
                "File Name": "sample.tif",
                "Vendor Field": "kept"
            }
        });

        let model = FaMetadataHeader::from_value(input.clone()).unwrap();
        let serialized = model.to_value().unwrap();

        assert_eq!(serialized, input);
    }
}
