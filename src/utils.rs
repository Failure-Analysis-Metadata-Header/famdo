// Utility functions
use serde_json::Value;
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use tempfile::Builder;

pub fn load_json(path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let json_text = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json_text)?)
}

fn encode_pointer_token(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

pub fn field_to_json_pointer(field: &str) -> String {
    if field.starts_with('/') {
        return field.to_string();
    }

    let encoded_tokens = field
        .split('.')
        .map(encode_pointer_token)
        .collect::<Vec<_>>();
    format!("/{}", encoded_tokens.join("/"))
}

pub fn write_bytes_atomically(
    path: impl AsRef<Path>,
    bytes: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let path_ref = path.as_ref();
    let parent = path_ref
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let file_name = path_ref.file_name().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("Output path does not name a file: {}", path_ref.display()),
        )
    })?;

    let prefix = format!(".{}.tmp.", file_name.to_string_lossy());
    let mut temp_file = Builder::new().prefix(&prefix).tempfile_in(parent)?;

    temp_file.write_all(bytes)?;
    temp_file.flush()?;
    temp_file.as_file_mut().sync_all()?;

    match temp_file.persist(path_ref) {
        Ok(_) => Ok(()),
        Err(err) if path_ref.exists() => {
            fs::remove_file(path_ref)?;
            err.file
                .persist(path_ref)
                .map(|_| ())
                .map_err(|err| err.error.into())
        }
        Err(err) => Err(err.error.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_json_valid_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_json = r#"{"name": "test", "value": 42}"#;
        temp_file.write_all(test_json.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let result = load_json(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());

        let json = result.unwrap();
        assert_eq!(json["name"], "test");
        assert_eq!(json["value"], 42);
    }

    #[test]
    fn test_load_json_invalid_path() {
        let result = load_json("/nonexistent/path/to/file.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_json_invalid_json() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"not valid json").unwrap();
        temp_file.flush().unwrap();

        let result = load_json(temp_file.path().to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_field_to_json_pointer_encodes_special_characters() {
        assert_eq!(
            field_to_json_pointer("General Section.Compressed Bits/Pixel"),
            "/General Section/Compressed Bits~1Pixel"
        );
        assert_eq!(
            field_to_json_pointer("generalSection.value~raw"),
            "/generalSection/value~0raw"
        );
    }

    #[test]
    fn test_field_to_json_pointer_preserves_explicit_json_pointer_input() {
        assert_eq!(
            field_to_json_pointer("/General Section/Compressed Bits~1Pixel"),
            "/General Section/Compressed Bits~1Pixel"
        );
    }
}
