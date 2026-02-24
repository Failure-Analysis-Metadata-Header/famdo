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

impl History {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn previous_measurements(
        mut self,
        previous_measurements: Vec<PreviousMeasurement>,
    ) -> Self {
        self.previous_measurements = Some(previous_measurements);
        self
    }

    pub fn workflow_step(mut self, workflow_step: i64) -> Self {
        self.workflow_step = Some(workflow_step);
        self
    }

    pub fn workflow_description(mut self, workflow_description: impl Into<String>) -> Self {
        self.workflow_description = Some(workflow_description.into());
        self
    }
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

impl PreviousMeasurement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header_file_path(mut self, header_file_path: impl Into<String>) -> Self {
        self.header_file_path = Some(header_file_path.into());
        self
    }

    pub fn image_file_path(mut self, image_file_path: impl Into<String>) -> Self {
        self.image_file_path = Some(image_file_path.into());
        self
    }

    pub fn time_stamp(mut self, time_stamp: impl Into<String>) -> Self {
        self.time_stamp = Some(time_stamp.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
}
