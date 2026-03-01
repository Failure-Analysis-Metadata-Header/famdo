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

    pub fn file_size(mut self, file_size: NumberWithUnit) -> Self {
        self.file_size = Some(file_size);
        self
    }

    pub fn logfile_path(mut self, logfile_path: impl Into<String>) -> Self {
        self.logfile_path = Some(logfile_path.into());
        self
    }

    pub fn previous_header_file(mut self, previous_header_file: impl Into<String>) -> Self {
        self.previous_header_file = Some(previous_header_file.into());
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

    pub fn image_width(mut self, image_width: IntegerWithUnit) -> Self {
        self.image_width = Some(image_width);
        self
    }

    pub fn image_height(mut self, image_height: IntegerWithUnit) -> Self {
        self.image_height = Some(image_height);
        self
    }

    pub fn pixel_width(mut self, pixel_width: NumberWithUnit) -> Self {
        self.pixel_width = Some(pixel_width);
        self
    }

    pub fn pixel_height(mut self, pixel_height: NumberWithUnit) -> Self {
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

    pub fn coordinates(mut self, coordinates: Coordinates) -> Self {
        self.coordinates = Some(coordinates);
        self
    }

    pub fn alignment_marks(mut self, alignment_marks: AlignmentMarks) -> Self {
        self.alignment_marks = Some(alignment_marks);
        self
    }
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

impl Coordinates {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stage_coordinate_system(mut self, stage_coordinate_system: impl Into<String>) -> Self {
        self.stage_coordinate_system = Some(stage_coordinate_system.into());
        self
    }

    pub fn global_or_local_frame(mut self, global_or_local_frame: bool) -> Self {
        self.global_or_local_frame = Some(global_or_local_frame);
        self
    }

    pub fn relative_orientation(mut self, relative_orientation: NumberArrayWithUnit) -> Self {
        self.relative_orientation = Some(relative_orientation);
        self
    }

    pub fn stage_position(mut self, stage_position: NumberArrayWithUnit) -> Self {
        self.stage_position = Some(stage_position);
        self
    }

    pub fn stage_rotation_rx(mut self, stage_rotation_rx: NumberWithUnit) -> Self {
        self.stage_rotation_rx = Some(stage_rotation_rx);
        self
    }

    pub fn stage_rotation_ry(mut self, stage_rotation_ry: NumberWithUnit) -> Self {
        self.stage_rotation_ry = Some(stage_rotation_ry);
        self
    }

    pub fn stage_rotation_rz(mut self, stage_rotation_rz: NumberWithUnit) -> Self {
        self.stage_rotation_rz = Some(stage_rotation_rz);
        self
    }
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

impl AlignmentMarks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn offset(mut self, offset: NumberArrayWithUnit) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn fiducial_positions(mut self, fiducial_positions: FiducialPositions) -> Self {
        self.fiducial_positions = Some(fiducial_positions);
        self
    }

    pub fn fiducial_type(mut self, fiducial_type: impl Into<String>) -> Self {
        self.fiducial_type = Some(fiducial_type.into());
        self
    }

    pub fn fiducial_size(mut self, fiducial_size: NumberWithUnit) -> Self {
        self.fiducial_size = Some(fiducial_size);
        self
    }
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

impl FiducialPositions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark1(mut self, mark1: NumberArrayWithUnit) -> Self {
        self.mark1 = Some(mark1);
        self
    }

    pub fn mark2(mut self, mark2: NumberArrayWithUnit) -> Self {
        self.mark2 = Some(mark2);
        self
    }

    pub fn mark3(mut self, mark3: NumberArrayWithUnit) -> Self {
        self.mark3 = Some(mark3);
        self
    }
}
