use crate::{
    ExtraFields, IntegerWithUnit, NumberArrayWithUnit, NumberWithUnit, Numeric, is_empty_map,
};
use serde::{Deserialize, Serialize};

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
    pub alignment_marks: Option<AlignmentMarks>,

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
pub struct AlignmentMarks {
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<NumberArrayWithUnit>,

    #[serde(
        rename = "fiducialPositions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fiducial_positions: Option<FiducialPositions>,

    #[serde(
        rename = "fiducialType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fiducial_type: Option<String>,

    #[serde(
        rename = "fiducialSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fiducial_size: Option<NumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FiducialPositions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mark1: Option<NumberArrayWithUnit>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mark2: Option<NumberArrayWithUnit>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mark3: Option<NumberArrayWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}
