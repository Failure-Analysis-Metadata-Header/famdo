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

impl DataEvaluation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn image_label(mut self, image_label: impl Into<String>) -> Self {
        self.image_label = Some(image_label.into());
        self
    }

    pub fn image_id(mut self, image_id: impl Into<String>) -> Self {
        self.image_id = Some(image_id.into());
        self
    }

    pub fn poi(mut self, poi: Vec<PointOfInterest>) -> Self {
        self.poi = Some(poi);
        self
    }

    pub fn push_point_of_interest(mut self, point_of_interest: PointOfInterest) -> Self {
        self.poi
            .get_or_insert_with(Vec::new)
            .push(point_of_interest);
        self
    }

    pub fn roi_region_of_interest(mut self, roi_region_of_interest: RegionsOfInterest) -> Self {
        self.roi_region_of_interest = Some(roi_region_of_interest);
        self
    }

    pub fn with_poi(poi: Vec<PointOfInterest>) -> Self {
        Self {
            poi: Some(poi),
            ..Self::default()
        }
    }
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
}

impl PointOfInterest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn coordinates(mut self, coordinates: LegacyNumberArrayWithUnit) -> Self {
        self.coordinates = Some(coordinates);
        self
    }

    pub fn from_coordinates(coordinates: LegacyNumberArrayWithUnit) -> Self {
        Self {
            coordinates: Some(coordinates),
            ..Self::default()
        }
    }

    pub fn from_f64_coordinates<I>(coordinates: I, unit: impl Into<String>) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        Self::from_coordinates(LegacyNumberArrayWithUnit::from_f64_values(
            coordinates,
            unit,
        ))
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_coordinates(mut self, coordinates: LegacyNumberArrayWithUnit) -> Self {
        self.coordinates = Some(coordinates);
        self
    }
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
}

impl RegionsOfInterest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roi_polygon(mut self, roi_polygon: Vec<RoiPolygon>) -> Self {
        self.roi_polygon = Some(roi_polygon);
        self
    }

    pub fn push_roi_polygon(mut self, roi_polygon: RoiPolygon) -> Self {
        self.roi_polygon
            .get_or_insert_with(Vec::new)
            .push(roi_polygon);
        self
    }

    pub fn roi_polyline(mut self, roi_polyline: Vec<RoiPolyline>) -> Self {
        self.roi_polyline = Some(roi_polyline);
        self
    }

    pub fn push_roi_polyline(mut self, roi_polyline: RoiPolyline) -> Self {
        self.roi_polyline
            .get_or_insert_with(Vec::new)
            .push(roi_polyline);
        self
    }

    pub fn roi_rectangle(mut self, roi_rectangle: Vec<RoiRectangle>) -> Self {
        self.roi_rectangle = Some(roi_rectangle);
        self
    }

    pub fn push_roi_rectangle(mut self, roi_rectangle: RoiRectangle) -> Self {
        self.roi_rectangle
            .get_or_insert_with(Vec::new)
            .push(roi_rectangle);
        self
    }
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
}

impl RoiPolygon {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn coordinates(mut self, coordinates: LegacyPointArrayWithUnit) -> Self {
        self.coordinates = Some(coordinates);
        self
    }

    pub fn area(mut self, area: LegacyNumberWithUnit) -> Self {
        self.area = Some(area);
        self
    }

    pub fn fill_color(mut self, fill_color: Vec<Option<Numeric>>) -> Self {
        self.fill_color = Some(fill_color);
        self
    }

    pub fn stroke_color(mut self, stroke_color: Vec<Option<Numeric>>) -> Self {
        self.stroke_color = Some(stroke_color);
        self
    }

    pub fn stroke_width(mut self, stroke_width: LegacyNumberWithUnit) -> Self {
        self.stroke_width = Some(stroke_width);
        self
    }
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
}

impl RoiPolyline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn coordinates(mut self, coordinates: LegacyPointArrayWithUnit) -> Self {
        self.coordinates = Some(coordinates);
        self
    }

    pub fn stroke_color(mut self, stroke_color: Vec<Option<Numeric>>) -> Self {
        self.stroke_color = Some(stroke_color);
        self
    }

    pub fn stroke_width(mut self, stroke_width: LegacyNumberWithUnit) -> Self {
        self.stroke_width = Some(stroke_width);
        self
    }
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
}

impl RoiRectangle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn center_coordinates(mut self, center_coordinates: LegacyNumberArrayWithUnit) -> Self {
        self.center_coordinates = Some(center_coordinates);
        self
    }

    pub fn width(mut self, width: LegacyNumberWithUnit) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: LegacyNumberWithUnit) -> Self {
        self.height = Some(height);
        self
    }

    pub fn rotation_angle(mut self, rotation_angle: LegacyNumberWithUnit) -> Self {
        self.rotation_angle = Some(rotation_angle);
        self
    }

    pub fn area(mut self, area: LegacyNumberWithUnit) -> Self {
        self.area = Some(area);
        self
    }

    pub fn fill_color(mut self, fill_color: Vec<Option<Numeric>>) -> Self {
        self.fill_color = Some(fill_color);
        self
    }

    pub fn stroke_color(mut self, stroke_color: Vec<Option<Numeric>>) -> Self {
        self.stroke_color = Some(stroke_color);
        self
    }

    pub fn stroke_width(mut self, stroke_width: LegacyNumberWithUnit) -> Self {
        self.stroke_width = Some(stroke_width);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_point_of_interest_helpers_create_expected_values() {
        let poi = PointOfInterest::from_f64_coordinates(vec![10.0, 20.0], "px")
            .with_name("POI-1")
            .with_label("hotspot")
            .with_id("1");

        assert_eq!(poi.name.as_deref(), Some("POI-1"));
        assert_eq!(poi.label.as_deref(), Some("hotspot"));
        assert_eq!(poi.id.as_deref(), Some("1"));
        assert_eq!(
            poi.coordinates.as_ref().and_then(|c| c.value.as_ref()),
            Some(&vec![
                Some(Numeric::Float(10.0)),
                Some(Numeric::Float(20.0))
            ])
        );
        assert_eq!(
            poi.coordinates.as_ref().and_then(|c| c.unit.as_deref()),
            Some("px")
        );
    }

    #[test]
    fn test_data_evaluation_with_poi_serializes() {
        let payload = DataEvaluation::with_poi(vec![
            PointOfInterest::from_f64_coordinates(vec![1.0, 2.0], "px")
                .with_name("POI-1")
                .with_id("1"),
            PointOfInterest::from_f64_coordinates(vec![3.0, 4.0], "px")
                .with_name("POI-2")
                .with_id("2"),
        ]);

        let value = serde_json::to_value(payload).unwrap();
        assert_eq!(value["POI"][0]["Name"], json!("POI-1"));
        assert_eq!(value["POI"][0]["Coordinates"]["Unit"], json!("px"));
        assert_eq!(value["POI"][1]["Coordinates"]["Value"], json!([3.0, 4.0]));
    }
}
