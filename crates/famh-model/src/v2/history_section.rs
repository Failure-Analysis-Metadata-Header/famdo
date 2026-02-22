use crate::{ExtraFields, is_empty_map};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct History {
    #[serde(
        rename = "previousMeasurements",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_measurements: Option<Vec<PreviousMeasurement>>,

    #[serde(
        rename = "workflowStep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub workflow_step: Option<i64>,

    #[serde(
        rename = "workflowDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub workflow_description: Option<String>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PreviousMeasurement {
    #[serde(
        rename = "headerFilePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub header_file_path: Option<String>,

    #[serde(
        rename = "imageFilePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_file_path: Option<String>,

    #[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}
