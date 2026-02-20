use crate::{ExtraFields, is_empty_map};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{Read, Write};

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

pub type JsonMap = std::collections::BTreeMap<String, Value>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Numeric {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UnitValue<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

pub type NumberWithUnit = UnitValue<Numeric>;
pub type IntegerWithUnit = UnitValue<i64>;
pub type NumberArrayWithUnit = UnitValue<Vec<Option<Numeric>>>;
pub type PointArrayWithUnit = UnitValue<Vec<Vec<Option<Numeric>>>>;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GeneralSection {
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,

    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    #[serde(
        rename = "fileFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub file_format: Option<String>,

    #[serde(rename = "fileSize", default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<NumberWithUnit>,

    #[serde(
        rename = "logfilePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub logfile_path: Option<String>,

    #[serde(
        rename = "previousHeaderFile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_header_file: Option<String>,

    #[serde(
        rename = "headerType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub header_type: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,

    #[serde(rename = "toolName", default, skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,

    #[serde(
        rename = "serialNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub serial_number: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    #[serde(
        rename = "imageWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_width: Option<IntegerWithUnit>,

    #[serde(
        rename = "imageHeight",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_height: Option<IntegerWithUnit>,

    #[serde(
        rename = "pixelWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pixel_width: Option<NumberWithUnit>,

    #[serde(
        rename = "pixelHeight",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pixel_height: Option<NumberWithUnit>,

    #[serde(rename = "bitDepth", default, skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<i64>,

    #[serde(
        rename = "compressedBitsPerPixel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compressed_bits_per_pixel: Option<Numeric>,

    #[serde(rename = "colorMode", default, skip_serializing_if = "Option::is_none")]
    pub color_mode: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(
        rename = "sampleHolder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sample_holder: Option<String>,

    #[serde(
        rename = "toolCalibrated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tool_calibrated: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Coordinates>,

    #[serde(
        rename = "alignmentMarks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub alignment_marks: Option<Value>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Coordinates {
    #[serde(
        rename = "stageCoordinateSystem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_coordinate_system: Option<String>,

    #[serde(
        rename = "globalOrLocalFrame",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_or_local_frame: Option<bool>,

    #[serde(
        rename = "relativeOrientation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_orientation: Option<NumberArrayWithUnit>,

    #[serde(
        rename = "stagePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_position: Option<NumberArrayWithUnit>,

    #[serde(
        rename = "stageRotationRx",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_rotation_rx: Option<NumberWithUnit>,

    #[serde(
        rename = "stageRotationRy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_rotation_ry: Option<NumberWithUnit>,

    #[serde(
        rename = "stageRotationRz",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_rotation_rz: Option<NumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MethodSpecific {
    #[serde(
        rename = "scanningElectronMicroscopy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scanning_electron_microscopy: Option<ScanningElectronMicroscopy>,

    #[serde(
        rename = "focusedIonBeam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub focused_ion_beam: Option<FocusedIonBeam>,

    #[serde(
        rename = "opticalMicroscopy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub optical_microscopy: Option<OpticalMicroscopy>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScanningElectronMicroscopy {
    #[serde(
        rename = "supplementaryMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_method: Option<String>,

    #[serde(
        rename = "acceleratingVoltage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub accelerating_voltage: Option<NumberWithUnit>,

    #[serde(
        rename = "deceleratingVoltage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub decelerating_voltage: Option<NumberWithUnit>,

    #[serde(
        rename = "workingDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_distance: Option<NumberWithUnit>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub magnification: Option<String>,

    #[serde(
        rename = "signalMixing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signal_mixing: Option<bool>,

    #[serde(
        rename = "signalTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signal_types: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detectors: Option<Vec<String>>,

    #[serde(
        rename = "signalProportion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signal_proportion: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "apertureSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aperture_size: Option<NumberWithUnit>,

    #[serde(
        rename = "apertureAlignment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aperture_alignment: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "stigmatorAlignment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stigmator_alignment: Option<Vec<Option<Numeric>>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brightness: Option<Vec<Option<Numeric>>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contrast: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "emissionCurrent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub emission_current: Option<NumberWithUnit>,

    #[serde(
        rename = "probeCurrent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub probe_current: Option<NumberWithUnit>,

    #[serde(
        rename = "highCurrentMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub high_current_mode: Option<bool>,

    #[serde(
        rename = "tiltCorrectionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tilt_correction_mode: Option<bool>,

    #[serde(
        rename = "correctedTiltAngle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub corrected_tilt_angle: Option<NumberWithUnit>,

    #[serde(rename = "beamShift", default, skip_serializing_if = "Option::is_none")]
    pub beam_shift: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "scanRotationMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scan_rotation_mode: Option<bool>,

    #[serde(
        rename = "scanRotation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scan_rotation: Option<NumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FocusedIonBeam {
    #[serde(
        rename = "fibSemIntersectionPoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fib_sem_intersection_point: Option<NumberWithUnit>,

    #[serde(
        rename = "fibTiltAngle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fib_tilt_angle: Option<NumberWithUnit>,

    #[serde(
        rename = "acceleratingVoltage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub accelerating_voltage: Option<NumberWithUnit>,

    #[serde(
        rename = "deceleratingVoltage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub decelerating_voltage: Option<NumberWithUnit>,

    #[serde(
        rename = "workingDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_distance: Option<NumberWithUnit>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub magnification: Option<String>,

    #[serde(
        rename = "signalMixing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signal_mixing: Option<bool>,

    #[serde(
        rename = "signalTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signal_types: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detectors: Option<Vec<String>>,

    #[serde(
        rename = "signalProportion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signal_proportion: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "ionCurrent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ion_current: Option<NumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OpticalMicroscopy {
    #[serde(
        rename = "objectiveMagnification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub objective_magnification: Option<String>,

    #[serde(
        rename = "opticalZoom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub optical_zoom: Option<Numeric>,

    #[serde(
        rename = "digitalZoom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub digital_zoom: Option<Numeric>,

    #[serde(
        rename = "contrastMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contrast_method: Option<String>,

    #[serde(rename = "hdrMode", default, skip_serializing_if = "Option::is_none")]
    pub hdr_mode: Option<bool>,

    #[serde(
        rename = "exposureTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposure_time: Option<NumberWithUnit>,

    #[serde(
        rename = "illuminationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub illumination_type: Option<String>,

    #[serde(
        rename = "numericalAperture",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub numerical_aperture: Option<Numeric>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomerSpecific {
    #[serde(rename = "sampleId", default, skip_serializing_if = "Option::is_none")]
    pub sample_id: Option<String>,

    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    #[serde(rename = "lotNumber", default, skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,

    #[serde(
        rename = "waferNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wafer_number: Option<String>,

    #[serde(
        rename = "dieCoordinates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub die_coordinates: Option<DieCoordinates>,

    #[serde(
        rename = "customFields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_fields: Option<JsonMap>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DieCoordinates {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<Value>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<Value>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ToolSpecific {
    #[serde(
        rename = "vendorName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_name: Option<String>,

    #[serde(
        rename = "softwareVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub software_version: Option<String>,

    #[serde(
        rename = "customParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_parameters: Option<JsonMap>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

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

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct History {
    #[serde(
        rename = "previousMeasurements",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_measurements: Option<Vec<PreviousMeasurement>>,

    #[serde(
        rename = "workflowStep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub workflow_step: Option<i64>,

    #[serde(
        rename = "workflowDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub workflow_description: Option<String>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PreviousMeasurement {
    #[serde(
        rename = "headerFilePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub header_file_path: Option<String>,

    #[serde(
        rename = "imageFilePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_file_path: Option<String>,

    #[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
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
}
