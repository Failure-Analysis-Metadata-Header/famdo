use crate::{ExtraFields, JsonMap, is_empty_map};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ToolSpecific {
    #[serde(
        rename = "vendorName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_name: Option<String>,

    #[serde(
        rename = "softwareVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub software_version: Option<String>,

    #[serde(
        rename = "customParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_parameters: Option<JsonMap>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

impl ToolSpecific {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vendor_name(mut self, vendor_name: impl Into<String>) -> Self {
        self.vendor_name = Some(vendor_name.into());
        self
    }

    pub fn software_version(mut self, software_version: impl Into<String>) -> Self {
        self.software_version = Some(software_version.into());
        self
    }

    pub fn custom_parameters(mut self, custom_parameters: JsonMap) -> Self {
        self.custom_parameters = Some(custom_parameters);
        self
    }
}
