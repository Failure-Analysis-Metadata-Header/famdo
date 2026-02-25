use crate::{ExtraFields, is_empty_map};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomerSection {
    #[serde(rename = "Infineon", default, skip_serializing_if = "Option::is_none")]
    pub infineon: Option<VendorSpecificSection>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

impl CustomerSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn infineon(mut self, infineon: VendorSpecificSection) -> Self {
        self.infineon = Some(infineon);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VendorSpecificSection {
    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

impl VendorSpecificSection {
    pub fn new() -> Self {
        Self::default()
    }
}
