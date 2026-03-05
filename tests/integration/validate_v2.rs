//! Integration tests for validation of v2 FAMH files

use famdo::commands::validate::validate_json;
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
