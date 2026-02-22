use crate::{ExtraFields, Numeric, is_empty_map};
use serde::{Deserialize, Serialize};

use super::{LegacyNumberArrayWithUnit, LegacyNumberWithUnit, LegacyPointArrayWithUnit};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DataEvaluation {
    #[serde(
        rename = "Image Label",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_label: Option<String>,

    #[serde(rename = "Image ID", default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,

    #[serde(rename = "POI", default, skip_serializing_if = "Option::is_none")]
    pub poi: Option<Vec<PointOfInterest>>,

    #[serde(
        rename = "ROI (Region of Interest)",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub roi_region_of_interest: Option<RegionsOfInterest>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PointOfInterest {
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "Label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename = "Coordinates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub coordinates: Option<LegacyNumberArrayWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RegionsOfInterest {
    #[serde(
        rename = "ROI-Polygon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub roi_polygon: Option<Vec<RoiPolygon>>,

    #[serde(
        rename = "ROI-Polyline",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub roi_polyline: Option<Vec<RoiPolyline>>,

    #[serde(
        rename = "ROI-Rectangle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub roi_rectangle: Option<Vec<RoiRectangle>>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RoiPolygon {
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "Label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename = "Coordinates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub coordinates: Option<LegacyPointArrayWithUnit>,

    #[serde(rename = "Area", default, skip_serializing_if = "Option::is_none")]
    pub area: Option<LegacyNumberWithUnit>,

    #[serde(rename = "FillColor", default, skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "StrokeColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stroke_color: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "StrokeWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stroke_width: Option<LegacyNumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RoiPolyline {
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "Label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename = "Coordinates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub coordinates: Option<LegacyPointArrayWithUnit>,

    #[serde(
        rename = "StrokeColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stroke_color: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "StrokeWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stroke_width: Option<LegacyNumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RoiRectangle {
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "Label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename = "Center Coordinates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub center_coordinates: Option<LegacyNumberArrayWithUnit>,

    #[serde(rename = "Width", default, skip_serializing_if = "Option::is_none")]
    pub width: Option<LegacyNumberWithUnit>,

    #[serde(rename = "Height", default, skip_serializing_if = "Option::is_none")]
    pub height: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Rotation Angle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rotation_angle: Option<LegacyNumberWithUnit>,

    #[serde(rename = "Area", default, skip_serializing_if = "Option::is_none")]
    pub area: Option<LegacyNumberWithUnit>,

    #[serde(rename = "FillColor", default, skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "StrokeColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stroke_color: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "StrokeWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stroke_width: Option<LegacyNumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}
