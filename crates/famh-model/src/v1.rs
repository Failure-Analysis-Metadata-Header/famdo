use crate::{ExtraFields, is_empty_map};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{Read, Write};
use std::str::FromStr;
mod customer_section;
mod data_evaluation;
mod general_section;
mod history_section;
mod method_specific;
mod tool_specific;
mod unit_types;

pub use customer_section::{CustomerSection, VendorSpecificSection};
pub use data_evaluation::{
    DataEvaluation, PointOfInterest, RegionsOfInterest, RoiPolygon, RoiPolyline, RoiRectangle,
};
pub use general_section::{
    AlignmentMarksSubSection, CoordinatesSubSection, GeneralSection, PositionOfFiducials,
};
pub use history_section::HistorySection;
pub use method_specific::{
    FocusedIonBeam, MethodSpecific, OpticalMicroscopy, ScanningElectronMicroscopy,
};
pub use tool_specific::ToolSpecific;
pub use unit_types::{
    LegacyIntegerWithUnit, LegacyNumberArrayWithUnit, LegacyNumberWithUnit,
    LegacyPointArrayWithUnit, LegacyUnitValue,
};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FaMetadataHeader {
    #[serde(rename = "General Section", default)]
    pub general_section: GeneralSection,

    #[serde(
        rename = "Method Specific",
        default,
        skip_serializing_if = "MethodSpecific::is_empty"
    )]
    pub method_specific: MethodSpecific,

    #[serde(
        rename = "Customer Section",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_section: Option<CustomerSection>,

    #[serde(
        rename = "Tool Specific",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tool_specific: Option<ToolSpecific>,

    #[serde(
        rename = "Data Evaluation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_evaluation: Option<DataEvaluation>,

    #[serde(rename = "History", default, skip_serializing_if = "Option::is_none")]
    pub history: Option<HistorySection>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

impl FaMetadataHeader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn general_section(mut self, general_section: GeneralSection) -> Self {
        self.general_section = general_section;
        self
    }

    pub fn method_specific(mut self, method_specific: MethodSpecific) -> Self {
        self.method_specific = method_specific;
        self
    }

    pub fn customer_section(mut self, customer_section: CustomerSection) -> Self {
        self.customer_section = Some(customer_section);
        self
    }

    pub fn tool_specific(mut self, tool_specific: ToolSpecific) -> Self {
        self.tool_specific = Some(tool_specific);
        self
    }

    pub fn data_evaluation(mut self, data_evaluation: DataEvaluation) -> Self {
        self.data_evaluation = Some(data_evaluation);
        self
    }

    pub fn history(mut self, history: HistorySection) -> Self {
        self.history = Some(history);
        self
    }

    pub fn from_reader<R: Read>(reader: R) -> serde_json::Result<Self> {
        crate::from_reader(reader)
    }
    pub fn from_value(value: Value) -> serde_json::Result<Self> {
        crate::from_value(value)
    }

    pub fn to_writer_pretty<W: Write>(&self, writer: W) -> serde_json::Result<()> {
        crate::to_writer_pretty(writer, self)
    }

    pub fn to_string_pretty(&self) -> serde_json::Result<String> {
        crate::to_string_pretty(self)
    }

    pub fn to_value(&self) -> serde_json::Result<Value> {
        crate::to_value(self)
    }
}

impl FromStr for FaMetadataHeader {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::from_str(s)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Numeric;
    use serde_json::json;

    #[test]
    fn parses_v1_top_level_sections() {
        let input = json!({
            "General Section": {
                "File Name": "sample.tif"
            },
            "Method Specific": {
                "Scanning Electron Microscopy": {}
            },
            "Custom Top Level": true
        });

        let model = FaMetadataHeader::from_value(input).unwrap();

        assert_eq!(model.extra.get("Custom Top Level"), Some(&json!(true)));
    }

    #[test]
    fn round_trips_v1_unknown_fields() {
        let input = json!({
            "General Section": {
                "File Name": "sample.tif",
                "Vendor Field": "kept"
            }
        });

        let model = FaMetadataHeader::from_value(input.clone()).unwrap();
        let serialized = model.to_value().unwrap();

        assert_eq!(serialized, input);
    }

    #[test]
    fn omits_empty_method_specific_section_during_serialization() {
        let header = FaMetadataHeader::new();

        let serialized = header.to_value().unwrap();

        assert!(serialized.get("Method Specific").is_none());
    }

    #[test]
    fn parses_nested_v1_typed_fields() {
        let input = json!({
            "General Section": {
                "File Name": "sample.tif",
                "Coordinates Sub Section": {
                    "Stage Coordinates X Y Z": {
                        "Value": [1.0, 2.0, 3.0],
                        "Unit": "mm"
                    }
                },
                "Alignment Marks Sub Section": {
                    "Position of the Fiducials": {
                        "Mark1": {
                            "Value": [11.0, 12.0, 13.0],
                            "Unit": "um"
                        }
                    }
                }
            },
            "Data Evaluation": {
                "POI": [
                    {
                        "Name": "POI-1",
                        "Coordinates": {
                            "Value": [120.0, 220.0],
                            "Unit": "px"
                        }
                    }
                ]
            }
        });

        let model = FaMetadataHeader::from_value(input.clone()).unwrap();

        let stage_coords = model
            .general_section
            .coordinates_sub_section
            .as_ref()
            .and_then(|c| c.stage_coordinates_x_y_z.as_ref())
            .and_then(|v| v.value.as_ref())
            .unwrap();
        assert_eq!(
            stage_coords,
            &vec![
                Some(Numeric::Float(1.0)),
                Some(Numeric::Float(2.0)),
                Some(Numeric::Float(3.0))
            ]
        );

        let poi_name = model
            .data_evaluation
            .as_ref()
            .and_then(|d| d.poi.as_ref())
            .and_then(|poi| poi.first())
            .and_then(|poi| poi.name.as_deref());
        assert_eq!(poi_name, Some("POI-1"));

        assert_eq!(model.to_value().unwrap(), input);
    }

    #[test]
    fn round_trips_fib_corrected_tilt_angle_values_typo_without_typed_value() {
        let input = json!({
            "General Section": {},
            "Method Specific": {
                "Focused Ion Beam": {
                    "Corrected Tilt Angle": {
                        "values": 1.5,
                        "Unit": "degrees"
                    }
                }
            }
        });

        let model = FaMetadataHeader::from_value(input.clone()).unwrap();

        let corrected_tilt = model
            .method_specific
            .focused_ion_beam
            .as_ref()
            .and_then(|f| f.corrected_tilt_angle.as_ref())
            .and_then(|a| a.value.as_ref());
        let corrected_tilt_raw = model
            .method_specific
            .focused_ion_beam
            .as_ref()
            .and_then(|f| f.corrected_tilt_angle.as_ref())
            .and_then(|a| a.extra.get("values"));

        assert_eq!(corrected_tilt, None);
        assert_eq!(corrected_tilt_raw, Some(&json!(1.5)));
        assert_eq!(model.to_value().unwrap(), input);
    }

    #[test]
    fn builds_v1_header_with_fluent_constructors() {
        let header = FaMetadataHeader::new()
            .general_section(GeneralSection::new().file_name("builder-v1.tif"))
            .method_specific(
                MethodSpecific::new().optical_microscopy(OpticalMicroscopy::new().hdr_mode(true)),
            )
            .data_evaluation(DataEvaluation::new().push_point_of_interest(
                PointOfInterest::from_f64_coordinates([10.0, 20.0], "px").with_name("POI-1"),
            ));

        let value = header.to_value().unwrap();
        assert_eq!(
            value["General Section"]["File Name"],
            json!("builder-v1.tif")
        );
        assert_eq!(value["Data Evaluation"]["POI"][0]["Name"], json!("POI-1"));
    }
}
