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
