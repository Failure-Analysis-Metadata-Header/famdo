use crate::{
    ExtraFields, NumberArrayWithUnit, NumberWithUnit, Numeric, PointArrayWithUnit, is_empty_map,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DataEvaluation {
    #[serde(
        rename = "imageLabel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_label: Option<String>,

    #[serde(rename = "imageId", default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,

    #[serde(
        rename = "pointsOfInterest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub points_of_interest: Option<Vec<PointOfInterest>>,

    #[serde(
        rename = "regionsOfInterest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub regions_of_interest: Option<RegionsOfInterest>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PointOfInterest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<NumberArrayWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RegionsOfInterest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub polygons: Option<Vec<PolygonRegion>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub polylines: Option<Vec<PolylineRegion>>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PolygonRegion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<PointArrayWithUnit>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<NumberWithUnit>,

    #[serde(rename = "fillColor", default, skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "strokeColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stroke_color: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "strokeWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stroke_width: Option<NumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PolylineRegion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<PointArrayWithUnit>,

    #[serde(
        rename = "strokeColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stroke_color: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "strokeWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stroke_width: Option<NumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}
