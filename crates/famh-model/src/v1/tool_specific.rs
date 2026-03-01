use crate::{ExtraFields, is_empty_map};
use serde::{Deserialize, Serialize};

use super::VendorSpecificSection;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ToolSpecific {
    #[serde(
        rename = "Tescan XEIA3",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tescan_xeia3: Option<VendorSpecificSection>,

    #[serde(
        rename = "ZEISS GeminiSEM 500",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zeiss_geminisem_500: Option<VendorSpecificSection>,

    #[serde(
        rename = "ZEISS LEO Gemini 1550",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zeiss_leo_gemini_1550: Option<VendorSpecificSection>,

    #[serde(
        rename = "ZEISS LEO Gemini 1560",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zeiss_leo_gemini_1560: Option<VendorSpecificSection>,

    #[serde(
        rename = "ZEISS Gemini ULTRA 55",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zeiss_gemini_ultra_55: Option<VendorSpecificSection>,

    #[serde(
        rename = "ZEISS Gemini SUPRA 55",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zeiss_gemini_supra_55: Option<VendorSpecificSection>,

    #[serde(
        rename = "HITACHI SU8000",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hitachi_su8000: Option<VendorSpecificSection>,

    #[serde(
        rename = "HITACHI SU8200",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hitachi_su8200: Option<VendorSpecificSection>,

    #[serde(
        rename = "FEI Magellan 400",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fei_magellan_400: Option<VendorSpecificSection>,

    #[serde(
        rename = "Olympus DSX110",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub olympus_dsx110: Option<VendorSpecificSection>,

    #[serde(
        rename = "Olympus DSX500",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub olympus_dsx500: Option<VendorSpecificSection>,

    #[serde(
        rename = "Olympus DSX1000",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub olympus_dsx1000: Option<VendorSpecificSection>,

    #[serde(flatten, default, skip_serializing_if = "is_empty_map")]
    pub extra: ExtraFields,
}

impl ToolSpecific {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tescan_xeia3(mut self, tescan_xeia3: VendorSpecificSection) -> Self {
        self.tescan_xeia3 = Some(tescan_xeia3);
        self
    }

    pub fn zeiss_geminisem_500(mut self, zeiss_geminisem_500: VendorSpecificSection) -> Self {
        self.zeiss_geminisem_500 = Some(zeiss_geminisem_500);
        self
    }

    pub fn zeiss_leo_gemini_1550(mut self, zeiss_leo_gemini_1550: VendorSpecificSection) -> Self {
        self.zeiss_leo_gemini_1550 = Some(zeiss_leo_gemini_1550);
        self
    }

    pub fn zeiss_leo_gemini_1560(mut self, zeiss_leo_gemini_1560: VendorSpecificSection) -> Self {
        self.zeiss_leo_gemini_1560 = Some(zeiss_leo_gemini_1560);
        self
    }

    pub fn zeiss_gemini_ultra_55(mut self, zeiss_gemini_ultra_55: VendorSpecificSection) -> Self {
        self.zeiss_gemini_ultra_55 = Some(zeiss_gemini_ultra_55);
        self
    }

    pub fn zeiss_gemini_supra_55(mut self, zeiss_gemini_supra_55: VendorSpecificSection) -> Self {
        self.zeiss_gemini_supra_55 = Some(zeiss_gemini_supra_55);
        self
    }

    pub fn hitachi_su8000(mut self, hitachi_su8000: VendorSpecificSection) -> Self {
        self.hitachi_su8000 = Some(hitachi_su8000);
        self
    }

    pub fn hitachi_su8200(mut self, hitachi_su8200: VendorSpecificSection) -> Self {
        self.hitachi_su8200 = Some(hitachi_su8200);
        self
    }

    pub fn fei_magellan_400(mut self, fei_magellan_400: VendorSpecificSection) -> Self {
        self.fei_magellan_400 = Some(fei_magellan_400);
        self
    }

    pub fn olympus_dsx110(mut self, olympus_dsx110: VendorSpecificSection) -> Self {
        self.olympus_dsx110 = Some(olympus_dsx110);
        self
    }

    pub fn olympus_dsx500(mut self, olympus_dsx500: VendorSpecificSection) -> Self {
        self.olympus_dsx500 = Some(olympus_dsx500);
        self
    }

    pub fn olympus_dsx1000(mut self, olympus_dsx1000: VendorSpecificSection) -> Self {
        self.olympus_dsx1000 = Some(olympus_dsx1000);
        self
    }
}
