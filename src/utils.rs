// Utility functions
use serde_json::Value;

pub fn load_json(path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let json_text = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json_text)?)
}
