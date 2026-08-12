use serde_json::Value;
use std::process::Command;
use tempfile::NamedTempFile;

fn run_validate(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_famdo"))
        .args(["validate"])
        .args(arguments)
        .output()
        .expect("famdo binary should run")
}

#[test]
fn invalid_metadata_returns_validation_failure() {
    let output = run_validate(&[
        "tests/fixtures/v2/complete_example_multiple_failures_v2.json",
        "--version",
        "v2",
    ]);

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Schema: v2-draft"));
    assert!(stdout.contains("Result: invalid"));
}

#[test]
fn json_format_is_parseable_and_contains_report_contract() {
    let output = run_validate(&[
        "tests/fixtures/v2/complete_example_multiple_failures_v2.json",
        "--version",
        "v2",
        "--format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).expect("stdout should be JSON");
    assert_eq!(report["schema"]["family"], "v2-draft");
    assert_eq!(report["result"], "invalid");
    assert!(report["counts"]["error"].as_u64().unwrap_or_default() > 0);
    assert!(report["findings"].is_array());
    assert!(report["sections"].is_array());
}

#[test]
fn missing_input_returns_operational_failure() {
    let output = run_validate(&["tests/fixtures/does-not-exist.json"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Error validating JSON"));
}

#[test]
fn fail_on_warning_controls_warning_only_reports() {
    let mut document: Value = serde_json::from_str(
        &std::fs::read_to_string("tests/fixtures/v2/minimal_example_optical.json")
            .expect("fixture should be readable"),
    )
    .expect("fixture should be valid JSON");
    document["Unexpected Root Section"] = serde_json::json!({});

    let temporary_file = NamedTempFile::new().expect("temporary file should be created");
    std::fs::write(
        temporary_file.path(),
        serde_json::to_vec(&document).expect("document should be serializable"),
    )
    .expect("temporary fixture should be writable");
    let path = temporary_file
        .path()
        .to_str()
        .expect("path should be UTF-8");

    let default_output = run_validate(&[path, "--version", "v2"]);
    assert_eq!(default_output.status.code(), Some(0));

    let warning_output = run_validate(&[path, "--version", "v2", "--fail-on", "warning"]);
    assert_eq!(warning_output.status.code(), Some(1));
}

#[test]
fn cache_inspect_reports_missing_metadata() {
    let output = Command::new(env!("CARGO_BIN_EXE_famdo"))
        .args([
            "cache",
            "inspect",
            "--version",
            "v2",
            "--revision",
            "phase5-test-cache-does-not-exist",
        ])
        .output()
        .expect("famdo binary should run");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("No schema cache metadata"));
}
