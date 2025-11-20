use crate::commands::extract::extract_metadata;
use crate::utils::load_json;
use serde_json::{Value, json};
use std::fs::File;
use std::io::BufReader;
use tiff::decoder::Decoder;
use tiff::decoder::ifd;

pub fn map_metadata(
    image_path: &str,
    connector_path: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let tiff_metadata = extract_metadata(&image_path);

    let connector = load_json(connector_path)?;

    Ok(true)
}
