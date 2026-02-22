use crate::{ExtraFields, NumberWithUnit, Numeric, is_empty_map};
use serde::{Deserialize, Serialize};

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
