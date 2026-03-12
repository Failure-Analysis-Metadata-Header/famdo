use std::fs::{self, File};
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::path::Path;

use crate::commands::edit::parse_field_str;
use crate::schema::SchemaVersion;
use famh_model::{v1, v2};
use serde_json::Value;

/// Split an RFC 6901 JSON Pointer into the parent pointer and final child token.
///
/// For example, `/generalSection/fileName` becomes
/// `("/generalSection", "fileName")`.
///
/// The returned child token is still JSON Pointer encoded. Call
/// [`decode_reference_token`] before using it as an object key.
pub fn extract_parent_pointer(pointer: &str) -> Result<(&str, &str), String> {
    if pointer.is_empty() {
        return Err("Invalid JSON pointer - cannot delete the document root.".to_string());
    }

    let (parent, child) = pointer
        .rsplit_once("/")
        .ok_or_else(|| "Invalid JSON pointer - could not extract field pointers.".to_string())?;
    Ok((parent, child))
}

/// Decode a single RFC 6901 reference token into the real object key.
///
/// JSON Pointer escapes `/` as `~1` and `~` as `~0`. `serde_json` handles
/// those escapes when traversing with `pointer()` and `pointer_mut()`, but
/// once we have the parent container we must decode the final token ourselves
/// before calling `Map::remove`.
///
/// For example, the final token in `/a~1b` is `a~1b`, but the actual JSON key
/// is `a/b`.
fn decode_reference_token(token: &str) -> Result<String, String> {
    let mut decoded = String::with_capacity(token.len());
    let mut chars = token.chars();

    while let Some(ch) = chars.next() {
        if ch != '~' {
            decoded.push(ch);
            continue;
        }

        match chars.next() {
            Some('0') => decoded.push('~'),
            Some('1') => decoded.push('/'),
            _ => return Err(format!("Invalid JSON pointer token: {token}")),
        }
    }

    Ok(decoded)
}

/// Remove the value addressed by `pointer` from its parent container.
///
/// This differs from `pointer_mut(pointer)` on the full path: deletion must
/// happen on the parent object or array so the child key/index is removed
/// entirely instead of being replaced with `null`.
fn remove_at_pointer(schema_doc: &mut Value, pointer: &str) -> Result<(), String> {
    let (parent_pointer, child_pointer) = extract_parent_pointer(pointer)?;
    let child_pointer = decode_reference_token(child_pointer)?;

    let parent = schema_doc
        .pointer_mut(parent_pointer)
        .ok_or_else(|| format!("Field path not found: {pointer}"))?;

    match parent {
        Value::Object(map) => map
            .remove(&child_pointer)
            .map(|_| ())
            .ok_or_else(|| format!("Field path not found: {pointer}")),
        Value::Array(items) => {
            let index = child_pointer
                .parse::<usize>()
                .map_err(|_| format!("Field path not found: {pointer}"))?;

            if index >= items.len() {
                return Err(format!("Field path not found: {pointer}"));
            }

            items.remove(index);
            Ok(())
        }
        _ => Err(format!("Field path not found: {pointer}")),
    }
}

fn serialize_schema_doc(
    schema_doc: Value,
    version: SchemaVersion,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut output = Vec::new();

    match version {
        SchemaVersion::V1 => {
            let updated_schema = v1::FaMetadataHeader::from_value(schema_doc)?;
            updated_schema.to_writer_pretty(&mut output)?;
        }
        SchemaVersion::V2 => {
            let updated_schema = v2::FaMetadataHeader::from_value(schema_doc)?;
            updated_schema.to_writer_pretty(&mut output)?;
        }
    }

    Ok(output)
}

pub fn delete_metadata_field(
    file_path: impl AsRef<Path>,
    field: String,
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

    let pointer = parse_field_str(&field);
    remove_at_pointer(&mut schema_doc, &pointer)
        .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

    let output = serialize_schema_doc(schema_doc, version)?;
    fs::write(file_path_ref, output)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_fixture_to_temp(path: &str) -> NamedTempFile {
        let fixture = fs::read(path).unwrap();
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(&fixture).unwrap();
        temp_file.flush().unwrap();
        temp_file
    }

    #[test]
    fn test_delete_metadata_field_removes_optional_v2_field_in_place() {
        let temp_file = write_fixture_to_temp("tests/fixtures/v2/minimal_example_optical.json");

        delete_metadata_field(
            temp_file.path(),
            "generalSection.fileName".to_string(),
            SchemaVersion::V2,
        )
        .unwrap();

        let updated: Value =
            serde_json::from_str(&fs::read_to_string(temp_file.path()).unwrap()).unwrap();

        assert!(updated["generalSection"].get("fileName").is_none());
        assert_eq!(
            updated["generalSection"]["toolName"],
            Value::String("DSX1000".to_string())
        );
        assert_eq!(
            updated["methodSpecific"]["opticalMicroscopy"]["objectiveMagnification"],
            Value::String("50x".to_string())
        );
    }

    #[test]
    fn test_delete_metadata_field_returns_error_for_missing_field_without_changing_file() {
        let temp_file = write_fixture_to_temp("tests/fixtures/v2/minimal_example_optical.json");
        let original = fs::read_to_string(temp_file.path()).unwrap();

        let result = delete_metadata_field(
            temp_file.path(),
            "generalSection.doesNotExist".to_string(),
            SchemaVersion::V2,
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Field path not found: /generalSection/doesNotExist"
        );
        assert_eq!(fs::read_to_string(temp_file.path()).unwrap(), original);
    }

    #[test]
    fn test_delete_metadata_field_removes_top_level_v1_section() {
        let temp_file = write_fixture_to_temp("tests/fixtures/v1/minimal_example_optical.json");

        delete_metadata_field(
            temp_file.path(),
            "/Method Specific".to_string(),
            SchemaVersion::V1,
        )
        .unwrap();

        let updated: Value =
            serde_json::from_str(&fs::read_to_string(temp_file.path()).unwrap()).unwrap();

        assert!(updated.get("Method Specific").is_none());
        assert_eq!(
            updated["General Section"]["File Name"],
            Value::String("optical_measurement_001.tiff".to_string())
        );
    }

    #[test]
    fn test_extract_parent_pointer_handles_root_and_nested_fields() {
        assert_eq!(
            extract_parent_pointer("/generalSection/fileName").unwrap(),
            ("/generalSection", "fileName")
        );
        assert_eq!(
            extract_parent_pointer("/General Section").unwrap(),
            ("", "General Section")
        );
    }

    #[test]
    fn test_extract_parent_pointer_rejects_root_document_pointer() {
        assert_eq!(
            extract_parent_pointer("").unwrap_err(),
            "Invalid JSON pointer - cannot delete the document root."
        );
    }
}
