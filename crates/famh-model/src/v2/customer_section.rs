use crate::{ExtraFields, JsonMap, is_empty_map};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomerSpecific {
    #[serde(rename = "sampleId", default, skip_serializing_if = "Option::is_none")]
    pub sample_id: Option<String>,

    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    #[serde(rename = "lotNumber", default, skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,

    #[serde(
        rename = "waferNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wafer_number: Option<String>,

    #[serde(
        rename = "dieCoordinates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub die_coordinates: Option<DieCoordinates>,

    #[serde(
        rename = "customFields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_fields: Option<JsonMap>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DieCoordinates {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<Value>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<Value>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}
