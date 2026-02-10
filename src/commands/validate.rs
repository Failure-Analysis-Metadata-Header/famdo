use crate::schema::{SchemaCache, SchemaVersion};
use crate::utils::load_json;
use colored::Colorize;
use jsonschema;

pub async fn validate_json(
    json_file_path: &str,
    version: SchemaVersion,
    no_cache: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let schema_cache = SchemaCache::download_all(version, !no_cache).await?;
    let json_file = load_json(json_file_path)?;
    let mut json_valid: bool = true;

    for (section_name, schema) in schema_cache.all_sections() {
        if let Some(section_data) = json_file.get(section_name) {
            let validator = jsonschema::validator_for(&schema)?;

            match validator.validate(section_data) {
                Ok(_) => {
                    println!("{} {}", section_name, "section is valid.".green());
                }
                Err(error) => {
                    println!("{} section - Validation error: {}", section_name, error);
                    json_valid = false
                }
            }
        }
    }
    Ok(json_valid)
}
