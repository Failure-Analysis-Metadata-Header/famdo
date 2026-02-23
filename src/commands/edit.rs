use famh_model::{Numeric, v1};
use serde_json::Value;
use std::path::Path;

use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

/// Parse a field string and create a pointer from it
/// Pointer should use RFC 6901 JSON Pointer navigation
fn parse_field_str(field: &str) -> String {
    let field_pointer = if field.starts_with("/") {
        field.to_string()
    } else {
        format!("/{}", field.split(".").collect::<Vec<_>>().join("/"))
    };
    field_pointer
}

/// Parse the value from the user
fn parse_value_str(value: &str) -> Value {
    serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.to_owned()))
}

pub fn edit_famh_file(
    file_path: impl AsRef<Path>,
    field: String,
    value: String,
    out_path: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path_ref = file_path.as_ref();

    let file = File::open(file_path_ref)?;
    let reader = BufReader::new(file);

    let schema: v1::FaMetadataHeader = v1::FaMetadataHeader::from_reader(reader)?;
    let mut schema_doc = schema.to_value()?;

    let pointer = parse_field_str(&field);

    if let Some(slot) = schema_doc.pointer_mut(&pointer) {
        *slot = parse_value_str(&value)
    } else {
        return Err(format!("Field path not found: {pointer}").into());
    }

    let updated_schema = v1::FaMetadataHeader::from_value(schema_doc)?;

    let out_path_ref = out_path.as_ref();
    let out_file = File::create(out_path_ref)?;
    let writer = BufWriter::new(out_file);
    updated_schema.to_writer_pretty(writer)?;

    Ok(())
}
