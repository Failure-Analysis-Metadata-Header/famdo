use crate::{ExtraFields, is_empty_map};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{Read, Write};

mod customer_section;
mod data_evaluation;
mod general_section;
mod history_section;
mod method_specific;
mod tool_specific;

pub use customer_section::{CustomerSpecific, DieCoordinates};
pub use data_evaluation::{
    DataEvaluation, PointOfInterest, PolygonRegion, PolylineRegion, RegionsOfInterest,
};
pub use general_section::{AlignmentMarks, Coordinates, FiducialPositions, GeneralSection};
pub use history_section::{History, PreviousMeasurement};
pub use method_specific::{
    FocusedIonBeam, MethodSpecific, OpticalMicroscopy, ScanningElectronMicroscopy,
};
pub use tool_specific::ToolSpecific;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FaMetadataHeader {
    #[serde(
        rename = "generalSection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub general_section: Option<GeneralSection>,

    #[serde(
        rename = "methodSpecific",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub method_specific: Option<MethodSpecific>,

    #[serde(
        rename = "customerSpecific",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_specific: Option<CustomerSpecific>,

    #[serde(
        rename = "toolSpecific",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tool_specific: Option<ToolSpecific>,

    #[serde(
        rename = "dataEvaluation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_evaluation: Option<DataEvaluation>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history: Option<History>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

impl FaMetadataHeader {
    pub fn from_reader<R: Read>(reader: R) -> serde_json::Result<Self> {
        crate::from_reader(reader)
    }

    pub fn from_str(json: &str) -> serde_json::Result<Self> {
        crate::from_str(json)
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
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_and_serializes_connector_style_v2_payload() {
        let input = json!({
            "generalSection": {
                "fileName": "image.tiff",
                "timeStamp": "2026-02-20T10:00:00+00:00",
                "manufacturer": "ZEISS",
                "toolName": "GeminiSEM 500",
                "method": "Optical",
                "imageWidth": {"value": 640, "unit": "px"}
            },
            "methodSpecific": {
                "opticalMicroscopy": {
                    "objectiveMagnification": "50x"
                }
            }
        });

        let model = FaMetadataHeader::from_value(input.clone()).unwrap();
        let serialized = model.to_value().unwrap();

        assert_eq!(
            serialized["generalSection"]["fileName"],
            json!("image.tiff")
        );
        assert_eq!(
            serialized["methodSpecific"]["opticalMicroscopy"]["objectiveMagnification"],
            json!("50x")
        );
    }

    #[test]
    fn keeps_unknown_fields_via_flatten() {
        let input = json!({
            "generalSection": {
                "fileName": "image.tiff",
                "timeStamp": "2026-02-20T10:00:00+00:00",
                "manufacturer": "ZEISS",
                "toolName": "GeminiSEM 500",
                "method": "SEM",
                "vendorPrivateField": "keep-me"
            },
            "unknownTopLevel": {
                "value": 1
            }
        });

        let model = FaMetadataHeader::from_value(input).unwrap();

        assert_eq!(
            model
                .general_section
                .as_ref()
                .and_then(|s| s.extra.get("vendorPrivateField")),
            Some(&json!("keep-me"))
        );
        assert_eq!(
            model.extra.get("unknownTopLevel"),
            Some(&json!({"value": 1}))
        );
    }

    #[test]
    fn parses_alignment_marks_with_fiducials() {
        let input = json!({
            "generalSection": {
                "fileName": "image.tiff",
                "timeStamp": "2026-02-20T10:00:00+00:00",
                "manufacturer": "ZEISS",
                "toolName": "GeminiSEM 500",
                "method": "SEM",
                "alignmentMarks": {
                    "offset": {
                        "value": [1.0, 2.0, 3.0],
                        "unit": "um"
                    },
                    "fiducialPositions": {
                        "mark1": {"value": [10.0, 11.0, 12.0], "unit": "um"},
                        "mark2": {"value": [20.0, 21.0, 22.0], "unit": "um"},
                        "mark3": {"value": [30.0, 31.0, 32.0], "unit": "um"}
                    },
                    "fiducialType": "cross",
                    "fiducialSize": {"value": 5.0, "unit": "um"}
                }
            }
        });

        let model = FaMetadataHeader::from_value(input.clone()).unwrap();

        let alignment_marks = model
            .general_section
            .as_ref()
            .and_then(|g| g.alignment_marks.as_ref())
            .unwrap();
        assert_eq!(alignment_marks.fiducial_type.as_deref(), Some("cross"));

        let serialized = model.to_value().unwrap();
        assert_eq!(serialized, input);
    }
}
