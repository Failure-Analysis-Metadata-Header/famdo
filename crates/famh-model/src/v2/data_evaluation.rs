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

    pub fn points_of_interest(mut self, points_of_interest: Vec<PointOfInterest>) -> Self {
        self.points_of_interest = Some(points_of_interest);
        self
    }

    pub fn push_point_of_interest(mut self, point_of_interest: PointOfInterest) -> Self {
        self.points_of_interest
            .get_or_insert_with(Vec::new)
            .push(point_of_interest);
        self
    }

    pub fn regions_of_interest(mut self, regions_of_interest: RegionsOfInterest) -> Self {
        self.regions_of_interest = Some(regions_of_interest);
        self
    }

    pub fn with_points_of_interest(points_of_interest: Vec<PointOfInterest>) -> Self {
        Self {
            points_of_interest: Some(points_of_interest),
            ..Self::default()
        }
    }
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

    pub fn coordinates(mut self, coordinates: NumberArrayWithUnit) -> Self {
        self.coordinates = Some(coordinates);
        self
    }

    pub fn from_coordinates(coordinates: NumberArrayWithUnit) -> Self {
        Self {
            coordinates: Some(coordinates),
            ..Self::default()
        }
    }

    pub fn from_f64_coordinates<I>(coordinates: I, unit: impl Into<String>) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        Self::from_coordinates(NumberArrayWithUnit::from_f64_values(coordinates, unit))
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

    pub fn with_coordinates(mut self, coordinates: NumberArrayWithUnit) -> Self {
        self.coordinates = Some(coordinates);
        self
    }
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

impl RegionsOfInterest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn polygons(mut self, polygons: Vec<PolygonRegion>) -> Self {
        self.polygons = Some(polygons);
        self
    }

    pub fn push_polygon(mut self, polygon: PolygonRegion) -> Self {
        self.polygons.get_or_insert_with(Vec::new).push(polygon);
        self
    }

    pub fn polylines(mut self, polylines: Vec<PolylineRegion>) -> Self {
        self.polylines = Some(polylines);
        self
    }

    pub fn push_polyline(mut self, polyline: PolylineRegion) -> Self {
        self.polylines.get_or_insert_with(Vec::new).push(polyline);
        self
    }
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

impl PolygonRegion {
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

    pub fn coordinates(mut self, coordinates: PointArrayWithUnit) -> Self {
        self.coordinates = Some(coordinates);
        self
    }

    pub fn area(mut self, area: NumberWithUnit) -> Self {
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

    pub fn stroke_width(mut self, stroke_width: NumberWithUnit) -> Self {
        self.stroke_width = Some(stroke_width);
        self
    }
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

impl PolylineRegion {
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

    pub fn coordinates(mut self, coordinates: PointArrayWithUnit) -> Self {
        self.coordinates = Some(coordinates);
        self
    }

    pub fn stroke_color(mut self, stroke_color: Vec<Option<Numeric>>) -> Self {
        self.stroke_color = Some(stroke_color);
        self
    }

    pub fn stroke_width(mut self, stroke_width: NumberWithUnit) -> Self {
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
    fn test_data_evaluation_with_points_of_interest_serializes() {
        let payload = DataEvaluation::with_points_of_interest(vec![
            PointOfInterest::from_f64_coordinates(vec![1.0, 2.0], "px")
                .with_name("POI-1")
                .with_id("1"),
            PointOfInterest::from_f64_coordinates(vec![3.0, 4.0], "px")
                .with_name("POI-2")
                .with_id("2"),
        ]);

        let value = serde_json::to_value(payload).unwrap();
        assert_eq!(value["pointsOfInterest"][0]["name"], json!("POI-1"));
        assert_eq!(
            value["pointsOfInterest"][0]["coordinates"]["unit"],
            json!("px")
        );
        assert_eq!(
            value["pointsOfInterest"][1]["coordinates"]["value"],
            json!([3.0, 4.0])
        );
    }
}
