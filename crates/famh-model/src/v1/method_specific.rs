use crate::{ExtraFields, Numeric, is_empty_map};
use serde::{Deserialize, Serialize};

use super::{LegacyNumberWithUnit, LegacyNumberWithUnitTypoValues};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MethodSpecific {
    #[serde(
        rename = "Scanning Electron Microscopy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scanning_electron_microscopy: Option<ScanningElectronMicroscopy>,

    #[serde(
        rename = "Focused Ion Beam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub focused_ion_beam: Option<FocusedIonBeam>,

    #[serde(
        rename = "Optical Microscopy",
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
        rename = "Supplementary Method",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplementary_method: Option<String>,

    #[serde(
        rename = "Accelerating Voltage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub accelerating_voltage: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Decelerating Voltage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub decelerating_voltage: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Working Distance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_distance: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Magnification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub magnification: Option<String>,

    #[serde(
        rename = "Signal Mixing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signal_mixing: Option<bool>,

    #[serde(
        rename = "Signal Type(s)",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signal_types: Option<Vec<String>>,

    #[serde(
        rename = "Detector(s)",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detectors: Option<Vec<String>>,

    #[serde(
        rename = "Signal Proportion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signal_proportion: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "Aperture Size",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aperture_size: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Aperture Alignment X Y",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aperture_alignment_xy: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "Stigmator Alignment X Y",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stigmator_alignment_xy: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "Brightness",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub brightness: Option<Vec<Option<Numeric>>>,

    #[serde(rename = "Contrast", default, skip_serializing_if = "Option::is_none")]
    pub contrast: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "Emission Current",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub emission_current: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Probe Current",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub probe_current: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "High Current Mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub high_current_mode: Option<bool>,

    #[serde(
        rename = "Tilt Correction Mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tilt_correction_mode: Option<bool>,

    #[serde(
        rename = "Corrected Tilt Angle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub corrected_tilt_angle: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Beam Shift X",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub beam_shift_x: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Beam Shift Y",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub beam_shift_y: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Scan Rotation Mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scan_rotation_mode: Option<bool>,

    #[serde(
        rename = "Scan Rotation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scan_rotation: Option<LegacyNumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FocusedIonBeam {
    #[serde(
        rename = "FIB-SEM Intersection Point",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fib_sem_intersection_point: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "FIB Tilt Angle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fib_tilt_angle: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Accelerating Voltage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub accelerating_voltage: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Decelerating Voltage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub decelerating_voltage: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Working Distance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_distance: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Magnification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub magnification: Option<String>,

    #[serde(
        rename = "Signal Mixing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signal_mixing: Option<bool>,

    #[serde(
        rename = "Signal Type(s)",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signal_types: Option<Vec<String>>,

    #[serde(
        rename = "Detector(s)",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detectors: Option<Vec<String>>,

    #[serde(
        rename = "Signal Proportion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signal_proportion: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "Aperture Size",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aperture_size: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Aperture Alignment X Y",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aperture_alignment_xy: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "Stigmator Alignment X Y",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stigmator_alignment_xy: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "Brightness",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub brightness: Option<Vec<Option<Numeric>>>,

    #[serde(rename = "Contrast", default, skip_serializing_if = "Option::is_none")]
    pub contrast: Option<Vec<Option<Numeric>>>,

    #[serde(
        rename = "Emission Current",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub emission_current: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Probe Current",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub probe_current: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "High Current Mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub high_current_mode: Option<bool>,

    #[serde(
        rename = "Tilt Correction Mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tilt_correction_mode: Option<bool>,

    #[serde(
        rename = "Corrected Tilt Angle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub corrected_tilt_angle: Option<LegacyNumberWithUnitTypoValues>,

    #[serde(
        rename = "Beam Shift X",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub beam_shift_x: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Beam Shift Y",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub beam_shift_y: Option<LegacyNumberWithUnit>,

    #[serde(
        rename = "Scan Rotation Mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scan_rotation_mode: Option<bool>,

    #[serde(
        rename = "Scan Rotation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scan_rotation: Option<LegacyNumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OpticalMicroscopy {
    #[serde(
        rename = "Objective Lens Magnification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub objective_lens_magnification: Option<Numeric>,

    #[serde(
        rename = "Optical Zoom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub optical_zoom: Option<Numeric>,

    #[serde(
        rename = "Digital Zoom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub digital_zoom: Option<Numeric>,

    #[serde(
        rename = "Contrast Method",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contrast_method: Option<String>,

    #[serde(rename = "HDR Mode", default, skip_serializing_if = "Option::is_none")]
    pub hdr_mode: Option<bool>,

    #[serde(
        rename = "Exposure Time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposure_time: Option<LegacyNumberWithUnit>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}
