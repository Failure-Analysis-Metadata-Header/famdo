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

impl<T> LegacyUnitValue<T> {
    pub fn empty() -> Self {
        Self {
            value: None,
            unit: None,
            extra: ExtraFields::new(),
        }
    }

    pub fn new(value: T, unit: impl Into<String>) -> Self {
        Self {
            value: Some(value),
            unit: Some(unit.into()),
            extra: ExtraFields::new(),
        }
    }

    pub fn from_value(value: T) -> Self {
        Self {
            value: Some(value),
            unit: None,
            extra: ExtraFields::new(),
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}

impl LegacyUnitValue<Vec<Option<Numeric>>> {
    pub fn from_f64_values<I>(values: I, unit: impl Into<String>) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        let values = values
            .into_iter()
            .map(|value| Some(Numeric::Float(value)))
            .collect();
        Self::new(values, unit)
    }
}

impl LegacyUnitValue<Vec<Vec<Option<Numeric>>>> {
    pub fn from_f64_points<I, J>(points: I, unit: impl Into<String>) -> Self
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = f64>,
    {
        let points = points
            .into_iter()
            .map(|point| {
                point
                    .into_iter()
                    .map(|value| Some(Numeric::Float(value)))
                    .collect()
            })
            .collect();
        Self::new(points, unit)
    }
}

impl<T> From<T> for LegacyUnitValue<T> {
    fn from(value: T) -> Self {
        Self::from_value(value)
    }
}

impl<T, U> From<(T, U)> for LegacyUnitValue<T>
where
    U: Into<String>,
{
    fn from((value, unit): (T, U)) -> Self {
        Self::new(value, unit)
    }
}

pub type LegacyNumberWithUnit = LegacyUnitValue<Numeric>;
pub type LegacyIntegerWithUnit = LegacyUnitValue<i64>;
pub type LegacyNumberArrayWithUnit = LegacyUnitValue<Vec<Option<Numeric>>>;
pub type LegacyPointArrayWithUnit = LegacyUnitValue<Vec<Vec<Option<Numeric>>>>;
