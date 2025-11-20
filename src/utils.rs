// Utility functions
use serde_json::Value;

pub fn load_json(path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let json_text = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json_text)?)
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
}
