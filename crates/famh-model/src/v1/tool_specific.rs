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
