use crate::commands::extract::extract_metadata;
use crate::utils::load_json;
use famh_model::v2::FaMetadataHeader;
use serde_json::{Value, json};
use serde_json_path::JsonPath;
use std::fs;

pub fn map_metadata(
    image_path: &str,
    connector_path: &str,
    output_path: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    // Extract metadata from TIFF
    let tiff_metadata = extract_metadata(image_path)?;

    // Normalize output through typed v2 model while retaining unknown fields.
    let famh = FaMetadataHeader::new();

    Ok(famh.to_value()?)
}
