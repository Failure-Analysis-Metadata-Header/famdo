use crate::{ExtraFields, Numeric, is_empty_map};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LegacyUnitValue<T> {
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,

    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

pub type LegacyNumberWithUnit = LegacyUnitValue<Numeric>;
pub type LegacyIntegerWithUnit = LegacyUnitValue<i64>;
pub type LegacyNumberArrayWithUnit = LegacyUnitValue<Vec<Option<Numeric>>>;
pub type LegacyPointArrayWithUnit = LegacyUnitValue<Vec<Vec<Option<Numeric>>>>;
