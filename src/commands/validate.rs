use crate::schema::{SchemaCache, SchemaType, SchemaVersion};
use colored::Colorize;
use jsonschema;
use serde_json::Value;

pub async fn validate_json(
    json_file_path: &str,
    version: SchemaVersion,
    no_cache: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Load schema validator
    let start = std::time::Instant::now();
    let schema_cache = SchemaCache::download_all(version, !no_cache).await?;
    let json_file = load_json(json_file_path)?;
    let mut json_valid: bool = true;

    for schema_type in [SchemaType::General, SchemaType::Method] {
        let schema = schema_cache.get(schema_type);
        let validator = jsonschema::validator_for(&schema)?;

        match validator.validate(&json_file) {
            Ok(_) => {
                println!("{:?} {}", schema_type, "section is valid.".green());
            }
            Err(error) => {
                println!("{:?} section - Validation error: {}", schema_type, error);
                json_valid = false
            }
        }
    }
    eprintln!("Validation took {}ms", start.elapsed().as_millis());
    Ok(json_valid)
}

fn load_json(path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let json_text = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json_text)?)
}
