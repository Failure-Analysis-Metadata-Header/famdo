use crate::{ExtraFields, Numeric, is_empty_map};
use serde::{Deserialize, Serialize};

use super::{LegacyIntegerWithUnit, LegacyNumberArrayWithUnit, LegacyNumberWithUnit};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GeneralSection {
    #[serde(rename = "File Path", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,

    #[serde(rename = "File Name", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    #[serde(
        rename = "File Format",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub file_format: Option<String>,

    #[serde(rename = "File Size", default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<LegacyIntegerWithUnit>,

    #[serde(
        rename = "Logfile Path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub logfile_path: Option<String>,

    #[serde(
        rename = "Header Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub header_type: Option<String>,

    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(
        rename = "Time Stamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_stamp: Option<String>,

    #[serde(
        rename = "Manufacturer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub manufacturer: Option<String>,

    #[serde(rename = "Tool Name", default, skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,

    #[serde(
        rename = "Serial Number",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub serial_number: Option<String>,

    #[serde(rename = "Method", default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    #[serde(
        rename = "Image Width",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_width: Option<LegacyIntegerWithUnit>,

    #[serde(
        rename = "Image Height",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_height: Option<LegacyIntegerWithUnit>,

    #[serde(
        rename = "Pixel Width",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pixel_width: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Pixel Height",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pixel_height: Option<LegacyNumberWithUnit>,

    #[serde(rename = "Bit Depth", default, skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<i64>,

    #[serde(
        rename = "Compressed Bits/Pixel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compressed_bits_per_pixel: Option<Numeric>,

    #[serde(
        rename = "Color Mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub color_mode: Option<String>,

    #[serde(rename = "Customer", default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(
        rename = "Sample Holder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sample_holder: Option<String>,

    #[serde(
        rename = "Tool Calibrated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tool_calibrated: Option<bool>,

    #[serde(
        rename = "Coordinates Sub Section",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub coordinates_sub_section: Option<CoordinatesSubSection>,

    #[serde(
        rename = "Alignment Marks Sub Section",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub alignment_marks_sub_section: Option<AlignmentMarksSubSection>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CoordinatesSubSection {
    #[serde(
        rename = "Stage Coordinate System Orientation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_coordinate_system_orientation: Option<String>,

    #[serde(
        rename = "global or local frame movement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_or_local_frame_movement: Option<bool>,

    #[serde(
        rename = "Relative Orientation to screens coordinate frame",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_orientation_to_screens_coordinate_frame: Option<LegacyNumberArrayWithUnit>,

    #[serde(
        rename = "Stage Coordinates X Y Z",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_coordinates_x_y_z: Option<LegacyNumberArrayWithUnit>,

    #[serde(
        rename = "Stage Rotation Rx",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_rotation_rx: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Stage Rotation Ry",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_rotation_ry: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Stage Rotation Rz",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_rotation_rz: Option<LegacyNumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AlignmentMarksSubSection {
    #[serde(
        rename = "Offset(x y z)",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offset_xyz: Option<LegacyNumberArrayWithUnit>,

    #[serde(
        rename = "Position of the Fiducials",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_of_the_fiducials: Option<PositionOfFiducials>,

    #[serde(
        rename = "Type of Fiducials",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_of_fiducials: Option<String>,

    #[serde(
        rename = "Fiducial Size",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fiducial_size: Option<LegacyNumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PositionOfFiducials {
    #[serde(rename = "Mark1", default, skip_serializing_if = "Option::is_none")]
    pub mark1: Option<LegacyNumberArrayWithUnit>,

    #[serde(rename = "Mark2", default, skip_serializing_if = "Option::is_none")]
    pub mark2: Option<LegacyNumberArrayWithUnit>,

    #[serde(rename = "Mark3", default, skip_serializing_if = "Option::is_none")]
    pub mark3: Option<LegacyNumberArrayWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}
