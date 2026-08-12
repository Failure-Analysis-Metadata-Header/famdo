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
