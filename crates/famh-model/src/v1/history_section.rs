use crate::{ExtraFields, is_empty_map};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HistorySection {
    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

impl HistorySection {
    pub fn new() -> Self {
        Self::default()
    }
}
