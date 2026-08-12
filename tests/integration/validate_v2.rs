//! Integration tests for validation of v2 FAMH files

use famdo::commands::validate::{FindingSeverity, validate_json, validate_json_report};
use famdo::schema::SchemaVersion;

#[tokio::test]
async fn test_valid_complete_example() {
    let famh_path = "tests/fixtures/v2/complete_example_v2.json";
    let result = validate_json(&famh_path, SchemaVersion::V2, true, false).await;

    assert!(
        result.is_ok(),
        "Should successfully validate complete example"
    );
    assert!(result.unwrap(), "Complete example should be valid v2");
}

#[tokio::test]
async fn test_invalid_multiple_failures_example() {
    let famh_path = "tests/fixtures/v2/complete_example_multiple_failures_v2.json";
    let result = validate_json(&famh_path, SchemaVersion::V2, true, false).await;

    assert!(
        result.is_ok(),
        "Validation should run without errors for invalid example."
    );

    assert!(!result.unwrap(), "Invalid v2 example should return false.")
}

#[tokio::test]
async fn test_minimal_example_optical() {
    let famh_path = "tests/fixtures/v2/minimal_example_optical.json";
    let result = validate_json(&famh_path, SchemaVersion::V2, true, false).await;

    assert!(
        result.is_ok(),
        "Should successfully validate optical example"
    );
    assert!(result.unwrap(), "Optical example should be valid v2");
}

#[tokio::test]
async fn test_empty_v2_document_reports_required_and_optional_sections() {
    let report = validate_json_report(
        "tests/fixtures/v2/empty_object.json",
        SchemaVersion::V2,
        true,
        false,
    )
    .await
    .expect("empty fixture should produce a report");

    assert!(report.has_errors());
    assert!(report.findings.iter().any(|finding| {
        finding.rule == "required-section" && finding.path == "/generalSection"
    }));
    assert!(report.findings.iter().any(|finding| {
        finding.rule == "required-section" && finding.path == "/methodSpecific"
    }));
    assert!(report.findings.iter().any(|finding| {
        finding.rule == "optional-section"
            && finding.severity == FindingSeverity::Info
            && finding.path == "/history"
    }));
}

#[tokio::test]
async fn test_v2_wrong_type_has_stable_field_path() {
    let report = validate_json_report(
        "tests/fixtures/v2/wrong_value_type.json",
        SchemaVersion::V2,
        true,
        false,
    )
    .await
    .expect("wrong-type fixture should produce a report");

    assert!(report.findings.iter().any(|finding| {
        finding.rule == "json-schema" && finding.path == "/generalSection/fileName"
    }));
}

#[tokio::test]
async fn test_v2_missing_required_field_is_reported() {
    let report = validate_json_report(
        "tests/fixtures/v2/missing_required_field.json",
        SchemaVersion::V2,
        true,
        false,
    )
    .await
    .expect("missing-field fixture should produce a report");

    assert!(report.findings.iter().any(|finding| {
        finding.rule == "json-schema"
            && finding.path == "/generalSection"
            && finding.message.contains("fileName")
    }));
}

#[tokio::test]
async fn test_v2_missing_required_section_is_reported() {
    let report = validate_json_report(
        "tests/fixtures/v2/missing_required_section.json",
        SchemaVersion::V2,
        true,
        false,
    )
    .await
    .expect("missing-section fixture should produce a report");

    assert!(report.findings.iter().any(|finding| {
        finding.rule == "required-section" && finding.path == "/methodSpecific"
    }));
}

#[tokio::test]
async fn test_v2_permissive_schema_allows_unknown_nested_core_field() {
    let report = validate_json_report(
        "tests/fixtures/v2/unknown_nested_core_field.json",
        SchemaVersion::V2,
        true,
        false,
    )
    .await
    .expect("nested-field fixture should produce a report");

    assert!(!report.has_errors());
}

#[tokio::test]
async fn test_v2_unknown_root_gets_alias_suggestion() {
    let report = validate_json_report(
        "tests/fixtures/v2/unknown_root_section.json",
        SchemaVersion::V2,
        true,
        false,
    )
    .await
    .expect("unknown-root fixture should produce a report");

    assert!(!report.has_errors());
    assert!(report.findings.iter().any(|finding| {
        finding.rule == "unknown-root-section"
            && finding.severity == FindingSeverity::Warning
            && finding.path == "/Method Section"
            && finding.suggestion.as_deref() == Some("methodSpecific")
    }));
}

#[tokio::test]
async fn test_v2_open_tool_extension_is_not_rejected() {
    let report = validate_json_report(
        "tests/fixtures/v2/tool_specific_extension.json",
        SchemaVersion::V2,
        true,
        false,
    )
    .await
    .expect("extension fixture should produce a report");

    assert!(!report.has_errors());
    assert!(
        !report
            .findings
            .iter()
            .any(|finding| finding.path.starts_with("/toolSpecific"))
    );
}

#[tokio::test]
async fn test_v2_poi_coordinate_dimensions_follow_schema_boundary() {
    let valid_report = validate_json_report(
        "tests/fixtures/v2/poi_2d.json",
        SchemaVersion::V2,
        true,
        false,
    )
    .await
    .expect("2D POI fixture should produce a report");
    assert!(!valid_report.has_errors());

    for path in [
        "tests/fixtures/v2/poi_1d.json",
        "tests/fixtures/v2/poi_4d.json",
    ] {
        let report = validate_json_report(path, SchemaVersion::V2, true, false)
            .await
            .expect("invalid POI fixture should produce a report");
        assert!(report.findings.iter().any(|finding| {
            finding.rule == "json-schema"
                && finding.path == "/dataEvaluation/pointsOfInterest/0/coordinates/value"
        }));
    }
}
