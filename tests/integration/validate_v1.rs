//! Integration tests for validation of v1 FAMH files

use famdo::commands::validate::{FindingSeverity, validate_json, validate_json_report};
use famdo::schema::SchemaVersion;

#[tokio::test]
async fn test_valid_minimal_example_optical() {
    let famh_path = "tests/fixtures/v1/minimal_example_optical.json";
    let result = validate_json(&famh_path, SchemaVersion::V1, true, false).await;

    assert!(
        result.is_ok(),
        "Should successfully validate optical example"
    );
    assert!(result.unwrap(), "Optical example should be valid v1");
}

#[tokio::test]
async fn test_valid_minimal_example_fib() {
    let famh_path = "tests/fixtures/v1/minimal_example_fib.json";
    let result = validate_json(&famh_path, SchemaVersion::V1, true, false).await;

    assert!(result.is_ok(), "Should successfully validate FIB example");
    assert!(result.unwrap(), "FIB example should be valid v1");
}

#[tokio::test]
async fn test_valid_minimal_example_sem() {
    let famh_path = "tests/fixtures/v1/minimal_example_sem.json";
    let result = validate_json(&famh_path, SchemaVersion::V1, true, false).await;

    assert!(result.is_ok(), "Should successfully validate SEM example");
    assert!(result.unwrap(), "SEM example should be valid v1");
}

#[tokio::test]
async fn test_valid_rectangle_example_sem() {
    let famh_path = "tests/fixtures/v1/rectangle_example_sem.json";
    let result = validate_json(&famh_path, SchemaVersion::V1, true, false).await;

    assert!(
        result.is_ok(),
        "Should successfully validate rectangle example"
    );
    assert!(result.unwrap(), "Rectangle example should be valid v1");
}

#[tokio::test]
async fn test_valid_rectangle_example_with_multiple_failures() {
    let famh_path = "tests/fixtures/v1/rectangle_example_sem_multiple_failures.json";
    let result = validate_json(&famh_path, SchemaVersion::V1, true, false).await;

    assert!(result.is_ok(), "Should successfully process the file");
    assert!(result.unwrap(), "Rectangle example should be valid v1");
}

#[tokio::test]
async fn test_valid_customer_specific_section_is_recognized() {
    let famh_path = "tests/fixtures/v1/minimal_example_optical_customer_specific.json";
    let report = validate_json_report(famh_path, SchemaVersion::V1, true, false)
        .await
        .expect("Should produce a validation report");

    assert!(!report.has_errors());
    assert!(
        report
            .sections
            .iter()
            .any(|section| section.name == "Customer Specific" && section.present)
    );
    assert!(
        !report
            .findings
            .iter()
            .any(|finding| finding.path == "/Customer Specific")
    );
}

#[tokio::test]
async fn test_report_lists_optional_sections_and_schema_errors() {
    let famh_path = "tests/fixtures/v1/minimal_example_optical.json";
    let report = validate_json_report(famh_path, SchemaVersion::V1, true, false)
        .await
        .expect("Should produce a validation report");

    assert!(!report.has_errors());
    assert!(report.findings.iter().any(|finding| {
        finding.severity == FindingSeverity::Info
            && finding.path == "/Data Evaluation"
            && finding.rule == "optional-section"
    }));
}

#[tokio::test]
async fn test_empty_v1_document_reports_required_sections() {
    let report = validate_json_report(
        "tests/fixtures/v1/empty_object.json",
        SchemaVersion::V1,
        true,
        false,
    )
    .await
    .expect("empty fixture should produce a report");

    assert!(report.has_errors());
    assert!(report.findings.iter().any(|finding| {
        finding.rule == "required-section" && finding.path == "/General Section"
    }));
    assert!(report.findings.iter().any(|finding| {
        finding.rule == "required-section" && finding.path == "/Method Specific"
    }));
}

#[tokio::test]
async fn test_v1_wrong_type_has_stable_field_path() {
    let report = validate_json_report(
        "tests/fixtures/v1/wrong_value_type.json",
        SchemaVersion::V1,
        true,
        false,
    )
    .await
    .expect("wrong-type fixture should produce a report");

    assert!(report.findings.iter().any(|finding| {
        finding.rule == "json-schema" && finding.path == "/General Section/File Name"
    }));
}

#[tokio::test]
async fn test_v1_missing_required_section_is_reported() {
    let report = validate_json_report(
        "tests/fixtures/v1/missing_required_section.json",
        SchemaVersion::V1,
        true,
        false,
    )
    .await
    .expect("missing-section fixture should produce a report");

    assert!(report.findings.iter().any(|finding| {
        finding.rule == "required-section" && finding.path == "/Method Specific"
    }));
}

#[tokio::test]
async fn test_v1_unknown_root_gets_alias_suggestion() {
    let report = validate_json_report(
        "tests/fixtures/v1/unknown_root_section.json",
        SchemaVersion::V1,
        true,
        false,
    )
    .await
    .expect("unknown-root fixture should produce a report");

    assert!(report.findings.iter().any(|finding| {
        finding.rule == "unknown-root-section"
            && finding.severity == FindingSeverity::Warning
            && finding.path == "/Method Section"
            && finding.suggestion.as_deref() == Some("Method Specific")
    }));
}

#[tokio::test]
async fn test_v1_missing_required_field_is_reported() {
    let report = validate_json_report(
        "tests/fixtures/v1/missing_required_field.json",
        SchemaVersion::V1,
        true,
        false,
    )
    .await
    .expect("missing-field fixture should produce a report");

    assert!(report.findings.iter().any(|finding| {
        finding.rule == "json-schema"
            && finding.path == "/General Section"
            && finding.message.contains("File Name")
    }));
}

#[tokio::test]
async fn test_v1_alias_gets_warning_and_canonical_suggestion() {
    let report = validate_json_report(
        "tests/fixtures/v1/customer_section_alias.json",
        SchemaVersion::V1,
        true,
        false,
    )
    .await
    .expect("alias fixture should produce a report");

    assert!(!report.has_errors());
    assert!(report.findings.iter().any(|finding| {
        finding.rule == "unknown-root-section"
            && finding.severity == FindingSeverity::Warning
            && finding.path == "/Customer Section"
            && finding.suggestion.as_deref() == Some("Customer Specific")
    }));
}
