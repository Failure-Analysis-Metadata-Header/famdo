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

impl CustomerSpecific {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sample_id(mut self, sample_id: impl Into<String>) -> Self {
        self.sample_id = Some(sample_id.into());
        self
    }

    pub fn order_id(mut self, order_id: impl Into<String>) -> Self {
        self.order_id = Some(order_id.into());
        self
    }

    pub fn project_id(mut self, project_id: impl Into<String>) -> Self {
        self.project_id = Some(project_id.into());
        self
    }

    pub fn lot_number(mut self, lot_number: impl Into<String>) -> Self {
        self.lot_number = Some(lot_number.into());
        self
    }

    pub fn wafer_number(mut self, wafer_number: impl Into<String>) -> Self {
        self.wafer_number = Some(wafer_number.into());
        self
    }

    pub fn die_coordinates(mut self, die_coordinates: DieCoordinates) -> Self {
        self.die_coordinates = Some(die_coordinates);
        self
    }

    pub fn custom_fields(mut self, custom_fields: JsonMap) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }
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

impl DieCoordinates {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn x(mut self, x: Value) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: Value) -> Self {
        self.y = Some(y);
        self
    }
}
