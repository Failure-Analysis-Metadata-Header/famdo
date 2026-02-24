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

impl GeneralSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn file_path(mut self, file_path: impl Into<String>) -> Self {
        self.file_path = Some(file_path.into());
        self
    }

    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    pub fn file_format(mut self, file_format: impl Into<String>) -> Self {
        self.file_format = Some(file_format.into());
        self
    }

    pub fn file_size(mut self, file_size: LegacyIntegerWithUnit) -> Self {
        self.file_size = Some(file_size);
        self
    }

    pub fn logfile_path(mut self, logfile_path: impl Into<String>) -> Self {
        self.logfile_path = Some(logfile_path.into());
        self
    }

    pub fn header_type(mut self, header_type: impl Into<String>) -> Self {
        self.header_type = Some(header_type.into());
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn time_stamp(mut self, time_stamp: impl Into<String>) -> Self {
        self.time_stamp = Some(time_stamp.into());
        self
    }

    pub fn manufacturer(mut self, manufacturer: impl Into<String>) -> Self {
        self.manufacturer = Some(manufacturer.into());
        self
    }

    pub fn tool_name(mut self, tool_name: impl Into<String>) -> Self {
        self.tool_name = Some(tool_name.into());
        self
    }

    pub fn serial_number(mut self, serial_number: impl Into<String>) -> Self {
        self.serial_number = Some(serial_number.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    pub fn image_width(mut self, image_width: LegacyIntegerWithUnit) -> Self {
        self.image_width = Some(image_width);
        self
    }

    pub fn image_height(mut self, image_height: LegacyIntegerWithUnit) -> Self {
        self.image_height = Some(image_height);
        self
    }

    pub fn pixel_width(mut self, pixel_width: LegacyNumberWithUnit) -> Self {
        self.pixel_width = Some(pixel_width);
        self
    }

    pub fn pixel_height(mut self, pixel_height: LegacyNumberWithUnit) -> Self {
        self.pixel_height = Some(pixel_height);
        self
    }

    pub fn bit_depth(mut self, bit_depth: i64) -> Self {
        self.bit_depth = Some(bit_depth);
        self
    }

    pub fn compressed_bits_per_pixel(
        mut self,
        compressed_bits_per_pixel: impl Into<Numeric>,
    ) -> Self {
        self.compressed_bits_per_pixel = Some(compressed_bits_per_pixel.into());
        self
    }

    pub fn color_mode(mut self, color_mode: impl Into<String>) -> Self {
        self.color_mode = Some(color_mode.into());
        self
    }

    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.customer = Some(customer.into());
        self
    }

    pub fn sample_holder(mut self, sample_holder: impl Into<String>) -> Self {
        self.sample_holder = Some(sample_holder.into());
        self
    }

    pub fn tool_calibrated(mut self, tool_calibrated: bool) -> Self {
        self.tool_calibrated = Some(tool_calibrated);
        self
    }

    pub fn coordinates_sub_section(
        mut self,
        coordinates_sub_section: CoordinatesSubSection,
    ) -> Self {
        self.coordinates_sub_section = Some(coordinates_sub_section);
        self
    }

    pub fn alignment_marks_sub_section(
        mut self,
        alignment_marks_sub_section: AlignmentMarksSubSection,
    ) -> Self {
        self.alignment_marks_sub_section = Some(alignment_marks_sub_section);
        self
    }
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

impl CoordinatesSubSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stage_coordinate_system_orientation(
        mut self,
        stage_coordinate_system_orientation: impl Into<String>,
    ) -> Self {
        self.stage_coordinate_system_orientation = Some(stage_coordinate_system_orientation.into());
        self
    }

    pub fn global_or_local_frame_movement(mut self, global_or_local_frame_movement: bool) -> Self {
        self.global_or_local_frame_movement = Some(global_or_local_frame_movement);
        self
    }

    pub fn relative_orientation_to_screens_coordinate_frame(
        mut self,
        relative_orientation_to_screens_coordinate_frame: LegacyNumberArrayWithUnit,
    ) -> Self {
        self.relative_orientation_to_screens_coordinate_frame =
            Some(relative_orientation_to_screens_coordinate_frame);
        self
    }

    pub fn stage_coordinates_x_y_z(
        mut self,
        stage_coordinates_x_y_z: LegacyNumberArrayWithUnit,
    ) -> Self {
        self.stage_coordinates_x_y_z = Some(stage_coordinates_x_y_z);
        self
    }

    pub fn stage_rotation_rx(mut self, stage_rotation_rx: LegacyNumberWithUnit) -> Self {
        self.stage_rotation_rx = Some(stage_rotation_rx);
        self
    }

    pub fn stage_rotation_ry(mut self, stage_rotation_ry: LegacyNumberWithUnit) -> Self {
        self.stage_rotation_ry = Some(stage_rotation_ry);
        self
    }

    pub fn stage_rotation_rz(mut self, stage_rotation_rz: LegacyNumberWithUnit) -> Self {
        self.stage_rotation_rz = Some(stage_rotation_rz);
        self
    }
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

impl AlignmentMarksSubSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn offset_xyz(mut self, offset_xyz: LegacyNumberArrayWithUnit) -> Self {
        self.offset_xyz = Some(offset_xyz);
        self
    }

    pub fn position_of_the_fiducials(
        mut self,
        position_of_the_fiducials: PositionOfFiducials,
    ) -> Self {
        self.position_of_the_fiducials = Some(position_of_the_fiducials);
        self
    }

    pub fn type_of_fiducials(mut self, type_of_fiducials: impl Into<String>) -> Self {
        self.type_of_fiducials = Some(type_of_fiducials.into());
        self
    }

    pub fn fiducial_size(mut self, fiducial_size: LegacyNumberWithUnit) -> Self {
        self.fiducial_size = Some(fiducial_size);
        self
    }
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

impl PositionOfFiducials {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark1(mut self, mark1: LegacyNumberArrayWithUnit) -> Self {
        self.mark1 = Some(mark1);
        self
    }

    pub fn mark2(mut self, mark2: LegacyNumberArrayWithUnit) -> Self {
        self.mark2 = Some(mark2);
        self
    }

    pub fn mark3(mut self, mark3: LegacyNumberArrayWithUnit) -> Self {
        self.mark3 = Some(mark3);
        self
    }
}
