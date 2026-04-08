use famh_model::{v1, v2};
use serde_json::Value;
use std::path::Path;

use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use crate::schema::SchemaVersion;
use crate::utils::field_to_json_pointer;

/// Parse the value from the user
fn parse_value_str(value: &str) -> Value {
    serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.to_owned()))
}

pub fn edit_famh_file(
    file_path: impl AsRef<Path>,
    field: String,
    value: String,
    out_path: impl AsRef<Path>,
    version: SchemaVersion,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path_ref = file_path.as_ref();

    let file = File::open(file_path_ref)?;
    let reader = BufReader::new(file);

    let mut schema_doc = match version {
        SchemaVersion::V1 => {
            let schema: v1::FaMetadataHeader = v1::FaMetadataHeader::from_reader(reader)?;
            schema.to_value()?
        }
        SchemaVersion::V2 => {
            let schema: v2::FaMetadataHeader = v2::FaMetadataHeader::from_reader(reader)?;
            schema.to_value()?
        }
    };

    let pointer = field_to_json_pointer(&field);

    if let Some(slot) = schema_doc.pointer_mut(&pointer) {
        *slot = parse_value_str(&value)
    } else {
        return Err(format!("Field path not found: {pointer}").into());
    }

    let out_path_ref = out_path.as_ref();
    let out_file = File::create(out_path_ref)?;
    let writer = BufWriter::new(out_file);
    match version {
        SchemaVersion::V1 => {
            let updated_schema = v1::FaMetadataHeader::from_value(schema_doc)?;
            updated_schema.to_writer_pretty(writer)?;
        }
        SchemaVersion::V2 => {
            let updated_schema = v2::FaMetadataHeader::from_value(schema_doc)?;
            updated_schema.to_writer_pretty(writer)?;
        }
    }

    Ok(())
}
