use crate::{ExtraFields, Numeric, is_empty_map};
use serde::{Deserialize, Serialize};

use super::LegacyNumberWithUnit;

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

impl MethodSpecific {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scanning_electron_microscopy(
        mut self,
        scanning_electron_microscopy: ScanningElectronMicroscopy,
    ) -> Self {
        self.scanning_electron_microscopy = Some(scanning_electron_microscopy);
        self
    }

    pub fn focused_ion_beam(mut self, focused_ion_beam: FocusedIonBeam) -> Self {
        self.focused_ion_beam = Some(focused_ion_beam);
        self
    }

    pub fn optical_microscopy(mut self, optical_microscopy: OpticalMicroscopy) -> Self {
        self.optical_microscopy = Some(optical_microscopy);
        self
    }
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

impl ScanningElectronMicroscopy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn supplementary_method(mut self, supplementary_method: impl Into<String>) -> Self {
        self.supplementary_method = Some(supplementary_method.into());
        self
    }

    pub fn accelerating_voltage(mut self, accelerating_voltage: LegacyNumberWithUnit) -> Self {
        self.accelerating_voltage = Some(accelerating_voltage);
        self
    }

    pub fn decelerating_voltage(mut self, decelerating_voltage: LegacyNumberWithUnit) -> Self {
        self.decelerating_voltage = Some(decelerating_voltage);
        self
    }

    pub fn working_distance(mut self, working_distance: LegacyNumberWithUnit) -> Self {
        self.working_distance = Some(working_distance);
        self
    }

    pub fn magnification(mut self, magnification: impl Into<String>) -> Self {
        self.magnification = Some(magnification.into());
        self
    }

    pub fn signal_mixing(mut self, signal_mixing: bool) -> Self {
        self.signal_mixing = Some(signal_mixing);
        self
    }

    pub fn signal_types(mut self, signal_types: Vec<String>) -> Self {
        self.signal_types = Some(signal_types);
        self
    }

    pub fn detectors(mut self, detectors: Vec<String>) -> Self {
        self.detectors = Some(detectors);
        self
    }

    pub fn signal_proportion(mut self, signal_proportion: Vec<Option<Numeric>>) -> Self {
        self.signal_proportion = Some(signal_proportion);
        self
    }

    pub fn signal_proportion_from_f64<I>(mut self, signal_proportion: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.signal_proportion = Some(
            signal_proportion
                .into_iter()
                .map(|value| Some(Numeric::Float(value)))
                .collect(),
        );
        self
    }

    pub fn aperture_size(mut self, aperture_size: LegacyNumberWithUnit) -> Self {
        self.aperture_size = Some(aperture_size);
        self
    }

    pub fn aperture_alignment_xy(mut self, aperture_alignment_xy: Vec<Option<Numeric>>) -> Self {
        self.aperture_alignment_xy = Some(aperture_alignment_xy);
        self
    }

    pub fn aperture_alignment_xy_from_f64<I>(mut self, aperture_alignment_xy: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.aperture_alignment_xy = Some(
            aperture_alignment_xy
                .into_iter()
                .map(|value| Some(Numeric::Float(value)))
                .collect(),
        );
        self
    }

    pub fn stigmator_alignment_xy(mut self, stigmator_alignment_xy: Vec<Option<Numeric>>) -> Self {
        self.stigmator_alignment_xy = Some(stigmator_alignment_xy);
        self
    }

    pub fn stigmator_alignment_xy_from_f64<I>(mut self, stigmator_alignment_xy: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.stigmator_alignment_xy = Some(
            stigmator_alignment_xy
                .into_iter()
                .map(|value| Some(Numeric::Float(value)))
                .collect(),
        );
        self
    }

    pub fn brightness(mut self, brightness: Vec<Option<Numeric>>) -> Self {
        self.brightness = Some(brightness);
        self
    }

    pub fn brightness_from_f64<I>(mut self, brightness: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.brightness = Some(
            brightness
                .into_iter()
                .map(|value| Some(Numeric::Float(value)))
                .collect(),
        );
        self
    }

    pub fn contrast(mut self, contrast: Vec<Option<Numeric>>) -> Self {
        self.contrast = Some(contrast);
        self
    }

    pub fn contrast_from_f64<I>(mut self, contrast: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.contrast = Some(
            contrast
                .into_iter()
                .map(|value| Some(Numeric::Float(value)))
                .collect(),
        );
        self
    }

    pub fn emission_current(mut self, emission_current: LegacyNumberWithUnit) -> Self {
        self.emission_current = Some(emission_current);
        self
    }

    pub fn probe_current(mut self, probe_current: LegacyNumberWithUnit) -> Self {
        self.probe_current = Some(probe_current);
        self
    }

    pub fn high_current_mode(mut self, high_current_mode: bool) -> Self {
        self.high_current_mode = Some(high_current_mode);
        self
    }

    pub fn tilt_correction_mode(mut self, tilt_correction_mode: bool) -> Self {
        self.tilt_correction_mode = Some(tilt_correction_mode);
        self
    }

    pub fn corrected_tilt_angle(mut self, corrected_tilt_angle: LegacyNumberWithUnit) -> Self {
        self.corrected_tilt_angle = Some(corrected_tilt_angle);
        self
    }

    pub fn beam_shift_x(mut self, beam_shift_x: LegacyNumberWithUnit) -> Self {
        self.beam_shift_x = Some(beam_shift_x);
        self
    }

    pub fn beam_shift_y(mut self, beam_shift_y: LegacyNumberWithUnit) -> Self {
        self.beam_shift_y = Some(beam_shift_y);
        self
    }

    pub fn scan_rotation_mode(mut self, scan_rotation_mode: bool) -> Self {
        self.scan_rotation_mode = Some(scan_rotation_mode);
        self
    }

    pub fn scan_rotation(mut self, scan_rotation: LegacyNumberWithUnit) -> Self {
        self.scan_rotation = Some(scan_rotation);
        self
    }
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

impl FocusedIonBeam {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fib_sem_intersection_point(
        mut self,
        fib_sem_intersection_point: LegacyNumberWithUnit,
    ) -> Self {
        self.fib_sem_intersection_point = Some(fib_sem_intersection_point);
        self
    }

    pub fn fib_tilt_angle(mut self, fib_tilt_angle: LegacyNumberWithUnit) -> Self {
        self.fib_tilt_angle = Some(fib_tilt_angle);
        self
    }

    pub fn accelerating_voltage(mut self, accelerating_voltage: LegacyNumberWithUnit) -> Self {
        self.accelerating_voltage = Some(accelerating_voltage);
        self
    }

    pub fn decelerating_voltage(mut self, decelerating_voltage: LegacyNumberWithUnit) -> Self {
        self.decelerating_voltage = Some(decelerating_voltage);
        self
    }

    pub fn working_distance(mut self, working_distance: LegacyNumberWithUnit) -> Self {
        self.working_distance = Some(working_distance);
        self
    }

    pub fn magnification(mut self, magnification: impl Into<String>) -> Self {
        self.magnification = Some(magnification.into());
        self
    }

    pub fn signal_mixing(mut self, signal_mixing: bool) -> Self {
        self.signal_mixing = Some(signal_mixing);
        self
    }

    pub fn signal_types(mut self, signal_types: Vec<String>) -> Self {
        self.signal_types = Some(signal_types);
        self
    }

    pub fn detectors(mut self, detectors: Vec<String>) -> Self {
        self.detectors = Some(detectors);
        self
    }

    pub fn signal_proportion(mut self, signal_proportion: Vec<Option<Numeric>>) -> Self {
        self.signal_proportion = Some(signal_proportion);
        self
    }

    pub fn signal_proportion_from_f64<I>(mut self, signal_proportion: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.signal_proportion = Some(
            signal_proportion
                .into_iter()
                .map(|value| Some(Numeric::Float(value)))
                .collect(),
        );
        self
    }

    pub fn aperture_size(mut self, aperture_size: LegacyNumberWithUnit) -> Self {
        self.aperture_size = Some(aperture_size);
        self
    }

    pub fn aperture_alignment_xy(mut self, aperture_alignment_xy: Vec<Option<Numeric>>) -> Self {
        self.aperture_alignment_xy = Some(aperture_alignment_xy);
        self
    }

    pub fn aperture_alignment_xy_from_f64<I>(mut self, aperture_alignment_xy: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.aperture_alignment_xy = Some(
            aperture_alignment_xy
                .into_iter()
                .map(|value| Some(Numeric::Float(value)))
                .collect(),
        );
        self
    }

    pub fn stigmator_alignment_xy(mut self, stigmator_alignment_xy: Vec<Option<Numeric>>) -> Self {
        self.stigmator_alignment_xy = Some(stigmator_alignment_xy);
        self
    }

    pub fn stigmator_alignment_xy_from_f64<I>(mut self, stigmator_alignment_xy: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.stigmator_alignment_xy = Some(
            stigmator_alignment_xy
                .into_iter()
                .map(|value| Some(Numeric::Float(value)))
                .collect(),
        );
        self
    }

    pub fn brightness(mut self, brightness: Vec<Option<Numeric>>) -> Self {
        self.brightness = Some(brightness);
        self
    }

    pub fn brightness_from_f64<I>(mut self, brightness: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.brightness = Some(
            brightness
                .into_iter()
                .map(|value| Some(Numeric::Float(value)))
                .collect(),
        );
        self
    }

    pub fn contrast(mut self, contrast: Vec<Option<Numeric>>) -> Self {
        self.contrast = Some(contrast);
        self
    }

    pub fn contrast_from_f64<I>(mut self, contrast: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.contrast = Some(
            contrast
                .into_iter()
                .map(|value| Some(Numeric::Float(value)))
                .collect(),
        );
        self
    }

    pub fn emission_current(mut self, emission_current: LegacyNumberWithUnit) -> Self {
        self.emission_current = Some(emission_current);
        self
    }

    pub fn probe_current(mut self, probe_current: LegacyNumberWithUnit) -> Self {
        self.probe_current = Some(probe_current);
        self
    }

    pub fn high_current_mode(mut self, high_current_mode: bool) -> Self {
        self.high_current_mode = Some(high_current_mode);
        self
    }

    pub fn tilt_correction_mode(mut self, tilt_correction_mode: bool) -> Self {
        self.tilt_correction_mode = Some(tilt_correction_mode);
        self
    }

    pub fn corrected_tilt_angle(mut self, corrected_tilt_angle: LegacyNumberWithUnit) -> Self {
        self.corrected_tilt_angle = Some(corrected_tilt_angle);
        self
    }

    pub fn beam_shift_x(mut self, beam_shift_x: LegacyNumberWithUnit) -> Self {
        self.beam_shift_x = Some(beam_shift_x);
        self
    }

    pub fn beam_shift_y(mut self, beam_shift_y: LegacyNumberWithUnit) -> Self {
        self.beam_shift_y = Some(beam_shift_y);
        self
    }

    pub fn scan_rotation_mode(mut self, scan_rotation_mode: bool) -> Self {
        self.scan_rotation_mode = Some(scan_rotation_mode);
        self
    }

    pub fn scan_rotation(mut self, scan_rotation: LegacyNumberWithUnit) -> Self {
        self.scan_rotation = Some(scan_rotation);
        self
    }
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

impl OpticalMicroscopy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn objective_lens_magnification(
        mut self,
        objective_lens_magnification: impl Into<Numeric>,
    ) -> Self {
        self.objective_lens_magnification = Some(objective_lens_magnification.into());
        self
    }

    pub fn optical_zoom(mut self, optical_zoom: impl Into<Numeric>) -> Self {
        self.optical_zoom = Some(optical_zoom.into());
        self
    }

    pub fn digital_zoom(mut self, digital_zoom: impl Into<Numeric>) -> Self {
        self.digital_zoom = Some(digital_zoom.into());
        self
    }

    pub fn contrast_method(mut self, contrast_method: impl Into<String>) -> Self {
        self.contrast_method = Some(contrast_method.into());
        self
    }

    pub fn hdr_mode(mut self, hdr_mode: bool) -> Self {
        self.hdr_mode = Some(hdr_mode);
        self
    }

    pub fn exposure_time(mut self, exposure_time: LegacyNumberWithUnit) -> Self {
        self.exposure_time = Some(exposure_time);
        self
    }
}
