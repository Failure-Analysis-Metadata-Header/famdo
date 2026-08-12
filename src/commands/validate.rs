use crate::cli::FailOn;
use crate::schema::{SchemaCache, SchemaVersion, SectionDefinition};
use crate::utils::load_json;
use serde_json::{Map, Value};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingSeverity {
    Error,
    Warning,
    Info,
}
impl FindingSeverity {
    fn label(self) -> &'static str {
        match self {
            FindingSeverity::Error => "ERROR",
            FindingSeverity::Warning => "WARNING",
            FindingSeverity::Info => "INFO",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationFinding {
    pub severity: FindingSeverity,
    pub path: String,
    pub message: String,
    pub suggestion: Option<String>,
    pub rule: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionValidationReport {
    pub name: String,
    pub present: bool,
    pub valid: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub schema_version: String,
    pub schema_source: String,
    pub sections: Vec<SectionValidationReport>,
    pub findings: Vec<ValidationFinding>,
}

impl ValidationReport {
    pub fn has_errors(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Error)
    }

    pub fn counts(&self) -> (usize, usize, usize) {
        self.findings.iter().fold(
            (0, 0, 0),
            |(errors, warnings, infos), finding| match finding.severity {
                FindingSeverity::Error => (errors + 1, warnings, infos),
                FindingSeverity::Warning => (errors, warnings + 1, infos),
                FindingSeverity::Info => (errors, warnings, infos + 1),
            },
        )
    }

    pub fn fails_on(&self, fail_on: FailOn) -> bool {
        self.has_errors()
            || (fail_on == FailOn::Warning
                && self
                    .findings
                    .iter()
                    .any(|finding| finding.severity == FindingSeverity::Warning))
    }

    pub fn json_value(&self) -> Value {
        let (errors, warnings, infos) = self.counts();
        serde_json::json!({
            "tool_version": env!("CARGO_PKG_VERSION"),
            "schema": {
                "family": self.schema_version,
                "source": self.schema_source,
            },
            "result": if self.has_errors() { "invalid" } else { "valid" },
            "counts": {
                "error": errors,
                "warning": warnings,
                "info": infos,
            },
            "sections": self.sections.iter().map(|section| serde_json::json!({
                "name": section.name,
                "present": section.present,
                "valid": section.valid,
            })).collect::<Vec<_>>(),
            "findings": self.findings.iter().map(|finding| serde_json::json!({
                "severity": finding.severity.label().to_ascii_lowercase(),
                "path": finding.path,
                "message": finding.message,
                "suggestion": finding.suggestion,
                "rule": finding.rule,
            })).collect::<Vec<_>>(),
        })
    }

    pub fn render_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.json_value())
    }

    pub fn render_text(&self) -> String {
        self.render_text_with_color(false)
    }

    pub fn render_text_with_color(&self, color: bool) -> String {
        let mut output = format!(
            "Schema: {} (source: {})\n",
            self.schema_version, self.schema_source
        );

        if !self.findings.is_empty() {
            output.push('\n');
            for finding in &self.findings {
                output.push_str(&format!(
                    "{} {}\n        {}\n",
                    colorize_severity(finding.severity, color),
                    finding.path,
                    finding.message
                ));
                if let Some(suggestion) = &finding.suggestion {
                    output.push_str(&format!("        Suggestion: {suggestion}\n"));
                }
            }
        }

        let (errors, warnings, infos) = self.counts();
        output.push_str(&format!(
            "\nResult: {} ({errors} errors, {warnings} warnings, {infos} info)\n",
            if self.has_errors() {
                "invalid"
            } else {
                "valid"
            }
        ));
        output
    }
}

impl fmt::Display for ValidationFinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.path, self.message)
    }
}

pub async fn validate_json_report(
    json_file_path: &str,
    version: SchemaVersion,
    no_cache: bool,
    strict: bool,
) -> Result<ValidationReport, Box<dyn std::error::Error>> {
    let json_file = load_json(json_file_path)?;
    let schema_cache = SchemaCache::download_all(version, !no_cache).await?;
    Ok(validate_json_content(&json_file, &schema_cache, strict))
}

pub async fn validate_json(
    json_file_path: &str,
    version: SchemaVersion,
    no_cache: bool,
    strict: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let report = validate_json_report(json_file_path, version, no_cache, strict).await?;
    Ok(!report.has_errors())
}

fn validate_json_content(
    json_file: &Value,
    schema_cache: &SchemaCache,
    strict: bool,
) -> ValidationReport {
    let mut report = ValidationReport {
        schema_version: schema_cache.version_label().to_owned(),
        schema_source: schema_cache.schema_source(),
        sections: Vec::new(),
        findings: Vec::new(),
    };

    let Some(top_level) = json_file.as_object() else {
        report.findings.push(ValidationFinding {
            severity: FindingSeverity::Error,
            path: "/".to_owned(),
            message: "Input JSON must be an object at the top level.".to_owned(),
            suggestion: None,
            rule: "top-level-object",
        });
        return report;
    };

    let definitions = schema_cache.section_definitions();
    report_unknown_sections(top_level, &definitions, strict, &mut report);

    for section in definitions {
        let present = top_level.contains_key(section.name);
        report.sections.push(SectionValidationReport {
            name: section.name.to_owned(),
            present,
            valid: present || !section.required,
        });

        if !present {
            let (severity, rule, message) = if section.required {
                (
                    FindingSeverity::Error,
                    "required-section",
                    "Required section is missing.",
                )
            } else {
                (
                    FindingSeverity::Info,
                    "optional-section",
                    "Optional section is not present.",
                )
            };
            report.findings.push(ValidationFinding {
                severity,
                path: section_path(section.name),
                message: message.to_owned(),
                suggestion: None,
                rule,
            });
            continue;
        }

        let section_data = top_level.get(section.name).expect("section was checked");
        match validate_section(section, section_data) {
            Ok(errors) => {
                let section_report = report
                    .sections
                    .last_mut()
                    .expect("section report was just added");
                section_report.valid = errors.is_empty();
                report.findings.extend(errors);
            }
            Err(error) => {
                let section_report = report
                    .sections
                    .last_mut()
                    .expect("section report was just added");
                section_report.valid = false;
                report.findings.push(ValidationFinding {
                    severity: FindingSeverity::Error,
                    path: section_path(section.name),
                    message: format!("Could not prepare section validation: {error}"),
                    suggestion: None,
                    rule: "schema-preparation",
                });
            }
        }
    }

    report
}

fn report_unknown_sections(
    top_level: &Map<String, Value>,
    definitions: &[SectionDefinition<'_>],
    strict: bool,
    report: &mut ValidationReport,
) {
    for section_name in top_level.keys().filter(|name| {
        !definitions
            .iter()
            .any(|definition| definition.name == name.as_str())
    }) {
        let suggestion = definitions.iter().find_map(|definition| {
            if definition.aliases.contains(&section_name.as_str()) {
                Some(definition.name.to_owned())
            } else {
                None
            }
        });
        let message = if let Some(target) = &suggestion {
            format!("Unexpected root-level section; this is a known alias for '{target}'.")
        } else {
            "Unexpected root-level section.".to_owned()
        };
        report.findings.push(ValidationFinding {
            severity: if strict {
                FindingSeverity::Error
            } else {
                FindingSeverity::Warning
            },
            path: section_path(section_name),
            message,
            suggestion,
            rule: "unknown-root-section",
        });
    }
}

fn validate_section(
    section: SectionDefinition<'_>,
    section_data: &Value,
) -> Result<Vec<ValidationFinding>, Box<dyn std::error::Error>> {
    let section_schema = get_section_validation_schema(section.name, section.schema)?;
    let validator = jsonschema::validator_for(section_schema)?;
    Ok(validator
        .iter_errors(section_data)
        .map(|error| ValidationFinding {
            severity: FindingSeverity::Error,
            path: format!("{}{}", section_path(section.name), error.instance_path),
            message: error.to_string(),
            suggestion: None,
            rule: "json-schema",
        })
        .collect())
}

fn section_path(section_name: &str) -> String {
    format!("/{}", escape_json_pointer_token(section_name))
}

fn colorize_severity(severity: FindingSeverity, color: bool) -> String {
    let label = severity.label();
    if !color {
        return label.to_owned();
    }

    let code = match severity {
        FindingSeverity::Error => 31,
        FindingSeverity::Warning => 33,
        FindingSeverity::Info => 36,
    };
    format!("\x1b[{code}m{label}\x1b[0m")
}

fn escape_json_pointer_token(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
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
    use crate::schema::{SchemaCache, V2SchemaCache};
    use serde_json::json;

    fn v2_test_cache() -> SchemaCache {
        SchemaCache::V2(V2SchemaCache {
            general: json!({
                "type": "object",
                "properties": {
                    "generalSection": {
                        "type": "object",
                        "required": ["fileName"],
                        "properties": {"fileName": {"type": "string"}}
                    }
                }
            }),
            customer: json!({"properties": {"customerSpecific": {"type": "object"}}}),
            tool: json!({"properties": {"toolSpecific": {"type": "object"}}}),
            method: json!({"properties": {"methodSpecific": {"type": "object"}}}),
            data_evaluation: json!({"properties": {"dataEvaluation": {"type": "object"}}}),
            history: json!({"properties": {"history": {"type": "object"}}}),
        })
    }

    #[test]
    fn reports_required_and_optional_missing_sections() {
        let input = json!({"generalSection": {"fileName": "sample.tif"}});
        let report = validate_json_content(&input, &v2_test_cache(), false);

        assert!(report.has_errors());
        assert!(report.findings.iter().any(|finding| {
            finding.rule == "required-section" && finding.path == "/methodSpecific"
        }));
        assert!(report.findings.iter().any(|finding| {
            finding.rule == "optional-section" && finding.path == "/customerSpecific"
        }));
    }

    #[test]
    fn reports_alias_without_aborting_other_sections() {
        let input = json!({
            "generalSection": {"fileName": "sample.tif"},
            "methodSpecific": {},
            "General Section": {}
        });
        let report = validate_json_content(&input, &v2_test_cache(), false);

        assert!(!report.has_errors());
        assert!(report.findings.iter().any(|finding| {
            finding.rule == "unknown-root-section"
                && finding.suggestion.as_deref() == Some("generalSection")
        }));
        assert!(
            report
                .findings
                .iter()
                .filter(|finding| finding.rule == "optional-section")
                .count()
                >= 3
        );
    }

    #[test]
    fn reports_schema_errors_with_stable_paths() {
        let input = json!({
            "generalSection": {"fileName": 123},
            "methodSpecific": {}
        });
        let report = validate_json_content(&input, &v2_test_cache(), false);

        assert!(report.findings.iter().any(|finding| {
            finding.rule == "json-schema" && finding.path == "/generalSection/fileName"
        }));
    }

    #[test]
    fn renders_schema_and_summary() {
        let input = json!({});
        let report = validate_json_content(&input, &v2_test_cache(), false);
        let rendered = report.render_text();

        assert!(rendered.contains("Schema: v2-draft"));
        assert!(rendered.contains("Result: invalid"));
    }

    #[test]
    fn renders_stable_json_report() {
        let input = json!({});
        let report = validate_json_content(&input, &v2_test_cache(), false);
        let rendered = report
            .render_json()
            .expect("report should be JSON serializable");
        let json: Value = serde_json::from_str(&rendered).expect("report should be valid JSON");

        assert_eq!(json["tool_version"], env!("CARGO_PKG_VERSION"));
        assert_eq!(json["schema"]["family"], "v2-draft");
        assert_eq!(json["result"], "invalid");
        assert_eq!(json["counts"]["error"], 2);
        assert!(json["findings"].is_array());
        assert!(json["sections"].is_array());
    }

    #[test]
    fn fail_on_warning_also_fails_on_errors() {
        let input = json!({
            "generalSection": {"fileName": "sample.tif"},
            "methodSpecific": {},
            "Unexpected": {}
        });
        let report = validate_json_content(&input, &v2_test_cache(), false);

        assert!(!report.fails_on(FailOn::Error));
        assert!(report.fails_on(FailOn::Warning));
    }
}
