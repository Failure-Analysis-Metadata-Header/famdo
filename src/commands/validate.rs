use crate::schema::{self, SchemaCache, SchemaVersion};
use crate::utils::load_json;
use colored::Colorize;
use jsonschema;
use serde_json::{Map, Value};
use std::collections::HashSet;

pub async fn validate_json(
    json_file_path: &str,
    version: SchemaVersion,
    no_cache: bool,
    strict: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let schema_cache = SchemaCache::download_all(version, !no_cache).await?;
    let json_file = load_json(json_file_path)?;

    validate_json_content(&json_file, &schema_cache, strict)
}

fn validate_json_content(
    json_file: &Value,
    schema_cache: &SchemaCache,
    strict: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let Some(top_level) = json_file.as_object() else {
        return Err("Input JSON must be an object at top level".into());
    };

    let mut json_valid: bool = true;

    let known_sections = schema_cache.all_sections();
    let allowed_section_names: HashSet<&str> = known_sections
        .iter()
        .map(|(section_name, _)| *section_name)
        .collect();

    let mut unknown_sections: Vec<&str> = top_level
        .keys()
        .map(String::as_str)
        .filter(|section_name| !allowed_section_names.contains(section_name))
        .collect();
    unknown_sections.sort_unstable();

    if !unknown_sections.is_empty() {
        println!(
            "Unknown root-level sections: {}",
            unknown_sections.join(", ").yellow()
        );
        if strict {
            json_valid = false;
        }
    }

    if !verify_required_sections(schema_cache, top_level) {
        json_valid = false;
    }

    for (section_name, schema) in known_sections {
        let section_is_valid = validate_schema_section(top_level, schema, section_name)?;
        if !section_is_valid {
            json_valid = false;
        }
    }

    Ok(json_valid)
}
/// Verify that all required root-level sections exist in the FA Header
fn verify_required_sections(
    schema_cache: &SchemaCache,
    top_level_schema: &Map<String, Value>,
) -> bool {
    let mut all_required_sections_exist = true;
    let mut missing_required: Vec<&str> = schema_cache
        .required_sections()
        .iter()
        .copied()
        .filter(|required| !top_level_schema.contains_key(*required))
        .collect();
    missing_required.sort_unstable();

    if !missing_required.is_empty() {
        println!(
            "Missing required sections: {}",
            missing_required.join(", ").bold()
        );
        all_required_sections_exist = false;
    }
    all_required_sections_exist
}

fn validate_schema_section(
    top_level_schema: &Map<String, Value>,
    schema: &Value,
    section_name: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut section_is_valid = true;
    if let Some(section_data) = top_level_schema.get(section_name) {
        let section_schema = get_section_validation_schema(section_name, schema)?;
        let validator = jsonschema::validator_for(section_schema)?;
        let errors: Vec<jsonschema::ValidationError<'_>> =
            validator.iter_errors(section_data).collect();

        if errors.is_empty() {
            println!("{} {}", section_name, "section is valid".green());
        } else {
            section_is_valid = false;
            println!(
                "{} section - {} validation error(s):",
                section_name,
                errors.len()
            );
            for err in errors {
                let full_error_path = format!("/{section_name}{}", err.instance_path.as_str());
                println!("{}: {}", full_error_path.red(), err);
            }
        }
    }
    Ok(section_is_valid)
}

fn get_section_validation_schema<'a>(
    section_name: &str,
    schema: &'a Value,
) -> Result<&'a Value, Box<dyn std::error::Error>> {
    if let Some(section_schema) = schema.get("properties").and_then(|p| p.get(section_name)) {
        return Ok(section_schema);
    }

    if let Some(section_schema) = schema.get(section_name) {
        return Ok(section_schema);
    }

    Err(format!("Could not find schema definition for section '{section_name}'").into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::V2SchemaCache;
    use serde_json::json;

    fn v2_test_cache() -> SchemaCache {
        SchemaCache::V2(V2SchemaCache {
            general: json!({
                "type": "object",
                "properties": {
                    "generalSection": {
                        "type": "object",
                        "required": ["fileName"],
                        "properties": {
                            "fileName": { "type": "string" }
                        }
                    }
                }
            }),
            customer: json!({
                "type": "object",
                "properties": {
                    "customerSpecific": { "type": "object" }
                }
            }),
            tool: json!({
                "type": "object",
                "properties": {
                    "toolSpecific": { "type": "object" }
                }
            }),
            method: json!({
                "type": "object",
                "properties": {
                    "methodSpecific": {
                        "type": "object",
                        "required": ["method"],
                        "properties": {
                            "method": { "type": "string" }
                        }
                    }
                }
            }),
            data_evaluation: json!({
                "type": "object",
                "properties": {
                    "dataEvaluation": { "type": "object" }
                }
            }),
            history: json!({
                "type": "object",
                "properties": {
                    "history": { "type": "object" }
                }
            }),
        })
    }

    #[test]
    fn strict_mode_rejects_missing_required_sections() {
        let input = json!({
            "generalSection": { "fileName": "sample.tif" }
        });

        let is_valid = validate_json_content(&input, &v2_test_cache(), true).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn strict_mode_rejects_unknown_sections() {
        let input = json!({
            "generalSection": { "fileName": "sample.tif" },
            "methodSpecific": { "method": "SEM" },
            "General": {}
        });

        let is_valid = validate_json_content(&input, &v2_test_cache(), true).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn section_validation_uses_inner_section_schema() {
        let input = json!({
            "generalSection": {},
            "methodSpecific": { "method": "SEM" }
        });

        let is_valid = validate_json_content(&input, &v2_test_cache(), false).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn validate_schema_section_accepts_valid_section() {
        let input = json!({
            "generalSection": {
                "fileName": "sample.tif",
                "method": "SEM"
            }
        });
        let top_level = input.as_object().unwrap();
        let schema = json!({
            "properties": {
                "generalSection": {
                    "type": "object",
                    "required": ["fileName", "method"],
                    "properties": {
                        "fileName": { "type": "string" },
                        "method": { "type": "string" }
                    }
                }
            }
        });

        let is_valid = validate_schema_section(top_level, &schema, "generalSection").unwrap();
        assert!(is_valid);
    }

    #[test]
    fn validate_schema_section_reports_multiple_failures() {
        let input = json!({
            "generalSection": {
                "fileName": 123,
                "method": 456
            }
        });
        let top_level = input.as_object().unwrap();
        let schema = json!({
            "properties": {
                "generalSection": {
                    "type": "object",
                    "required": ["fileName", "method"],
                    "properties": {
                        "fileName": { "type": "string" },
                        "method": { "type": "string" }
                    }
                }
            }
        });

        let section_data = top_level.get("generalSection").unwrap();
        let section_schema = get_section_validation_schema("generalSection", &schema).unwrap();
        let validator = jsonschema::validator_for(section_schema).unwrap();
        let errors: Vec<_> = validator.iter_errors(section_data).collect();

        assert!(
            errors.len() >= 2,
            "expected multiple errors, got {}",
            errors.len()
        );

        let is_valid = validate_schema_section(top_level, &schema, "generalSection").unwrap();
        assert!(!is_valid);
    }
}
