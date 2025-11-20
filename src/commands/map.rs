use crate::commands::extract::extract_metadata;
use crate::utils::load_json;
use serde_json::{Value, json};
use serde_json_path::JsonPath;
use std::fs;

pub fn map_metadata(
    image_path: &str,
    connector_path: &str,
    output_path: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    // Extract metadata from TIFF
    let tiff_metadata = extract_metadata(image_path)?;

    // Load connector configuration
    let connector = load_json(connector_path)?;

    // Map to FAM v2 format
    let fam_output = apply_mapping(&tiff_metadata, &connector)?;

    let outfile = fs::File::create(output_path)?;
    serde_json::to_writer_pretty(outfile, &fam_output)?;

    Ok(fam_output)
}

fn apply_mapping(metadata: &Value, connector: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    let mappings = connector["mappings"]
        .as_object()
        .ok_or("Missing 'mappings' in connector")?;

    let mut output = json!({});

    // Process each section (generalSection, methodSpecific, etc.)
    for (section_name, section_mappings) in mappings {
        let mut section_output = json!({});

        if let Some(fields) = section_mappings.as_object() {
            for (field_name, field_config) in fields {
                if let Some(value) = extract_field(metadata, field_config)? {
                    // Handle nested objects (e.g., methodSpecific.opticalMicroscopy)
                    if let Some(nested) = field_config.as_object() {
                        if nested.contains_key("objectiveMagnification")
                            || nested.values().any(|v| v.is_object())
                        {
                            // This is a nested section
                            let mut nested_output = json!({});
                            for (nested_field, nested_config) in nested {
                                if let Some(nested_value) = extract_field(metadata, nested_config)?
                                {
                                    nested_output[nested_field] = nested_value;
                                }
                            }
                            if !nested_output.as_object().unwrap().is_empty() {
                                section_output[field_name] = nested_output;
                            }
                            continue;
                        }
                    }
                    section_output[field_name] = value;
                }
            }
        }

        if !section_output.as_object().unwrap().is_empty() {
            output[section_name] = section_output;
        }
    }

    Ok(output)
}

fn extract_field(
    metadata: &Value,
    config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    // Handle default values
    if let Some(default) = config.get("default") {
        if !default.is_null() {
            return Ok(Some(default.clone()));
        }
    }

    // Get source path
    let source = match config.get("source") {
        Some(s) if !s.is_null() => s.as_str().ok_or("Source must be a string")?,
        _ => return Ok(None),
    };

    // Handle simple paths (e.g., "filename", "dimensions.width")
    if !source.contains('[') {
        let value = extract_simple_path(metadata, source);
        if value.is_some() {
            return format_output(value, config);
        }
    } else {
        // Handle JSONPath queries (e.g., "tags[?tag=='DateTime'].value")
        if let Some(value) = extract_jsonpath(metadata, source)? {
            return format_output(Some(value), config);
        }
    }

    // Try fallback
    if let Some(fallback) = config.get("fallback") {
        if let Some(fallback_str) = fallback.as_str() {
            // Apply transformation to fallback if it's a transform name
            if let Some(transform) = config.get("transform").and_then(|t| t.as_str()) {
                if fallback_str == "current_timestamp" || fallback_str == transform {
                    return format_output(Some(apply_transform(json!(""), transform)?), config);
                }
            }
            return Ok(Some(json!(fallback_str)));
        }
    }

    Ok(None)
}

fn extract_simple_path(metadata: &Value, path: &str) -> Option<Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = metadata;

    for part in parts {
        current = current.get(part)?;
    }

    Some(current.clone())
}

fn extract_jsonpath(
    metadata: &Value,
    query: &str,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    // Convert our simplified query to proper JSONPath
    // "tags[?tag=='DateTime'].value" -> "$.tags[?(@.tag=='DateTime')].value"
    let jsonpath_query = if let Some(filter_start) = query.find("[?") {
        let before = &query[..filter_start];
        let rest = &query[filter_start + 2..];

        if let Some(filter_end) = rest.find(']') {
            let filter = &rest[..filter_end];
            let after = &rest[filter_end + 1..];

            // Parse "tag=='DateTime'" -> "@.tag == 'DateTime'"
            if let Some(eq_pos) = filter.find("==") {
                let field = filter[..eq_pos].trim();
                let value_part = filter[eq_pos + 2..].trim();

                format!("$.{}[?(@.{} == {})]{}", before, field, value_part, after)
            } else {
                format!("$.{}", query)
            }
        } else {
            format!("$.{}", query)
        }
    } else {
        format!("$.{}", query)
    };

    let path = JsonPath::parse(&jsonpath_query)?;
    let nodes = path.query(metadata).all();

    Ok(nodes.first().map(|v| (*v).clone()))
}

fn format_output(
    value: Option<Value>,
    config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    let mut value = match value {
        Some(v) => v,
        None => return Ok(None),
    };

    // Apply transformations
    if let Some(transform) = config.get("transform").and_then(|t| t.as_str()) {
        value = apply_transform(value, transform)?;
    }

    // If unit is specified, wrap in {value, unit} object
    if let Some(unit) = config.get("unit").and_then(|u| u.as_str()) {
        return Ok(Some(json!({
            "value": value,
            "unit": unit
        })));
    }

    Ok(Some(value))
}

fn apply_transform(value: Value, transform: &str) -> Result<Value, Box<dyn std::error::Error>> {
    match transform {
        "extract_basename" => {
            if let Some(path_str) = value.as_str() {
                let path = std::path::Path::new(path_str);
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    return Ok(json!(name));
                }
            }
            Ok(value)
        }
        "extract_extension" => {
            if let Some(path_str) = value.as_str() {
                let path = std::path::Path::new(path_str);
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    return Ok(json!(format!(".{}", ext)));
                }
            }
            Ok(value)
        }
        "extract_first_numeric" => {
            if let Some(arr) = value.as_array() {
                if let Some(first) = arr.first() {
                    return Ok(first.clone());
                }
            }
            Ok(value)
        }
        "resolution_to_nanometers" => {
            // Convert rational "96000/1000" to DPI then to nanometers
            if let Some(rational) = value.as_str() {
                if let Some((num, den)) = rational.split_once('/') {
                    if let (Ok(n), Ok(d)) = (num.parse::<f64>(), den.parse::<f64>()) {
                        let dpi = n / d;
                        // Convert DPI to nanometers: (25.4mm / inch) * (1e6 nm / mm) / DPI
                        let nm = 25400000.0 / dpi;
                        return Ok(json!(nm.round() as i64));
                    }
                }
            }
            Ok(value)
        }
        "clean_string" => {
            if let Some(s) = value.as_str() {
                Ok(json!(s.trim()))
            } else {
                Ok(value)
            }
        }
        "current_timestamp" | "tiff_datetime_to_iso8601" => {
            // For now, just use current timestamp as ISO8601
            use chrono::Local;
            Ok(json!(Local::now().to_rfc3339()))
        }
        _ => Ok(value), // Unknown transform, return as-is
    }
}
