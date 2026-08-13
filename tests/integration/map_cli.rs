use std::process::Command;
use tempfile::NamedTempFile;

fn run_map(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_famdo"))
        .args(["map"])
        .args(arguments)
        .output()
        .expect("famdo binary should run")
}

#[test]
fn missing_connector_returns_operational_failure() {
    let output = run_map(&[
        "tests/fixtures/images/example_image.tif",
        "tests/fixtures/connectors/does-not-exist.json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Could not read connector"));
}

#[test]
fn unsupported_v2_connector_returns_mapping_failure_without_network_access() {
    let schema = NamedTempFile::new().expect("temporary schema should be created");
    std::fs::write(schema.path(), b"{}").expect("temporary schema should be writable");
    let connector = NamedTempFile::new().expect("temporary connector should be created");
    std::fs::write(
        connector.path(),
        br#"{"targetSchemaVersion":"2","mappings":[]}"#,
    )
    .expect("temporary connector should be writable");
    let output = NamedTempFile::new().expect("temporary output should be created");

    let output = run_map(&[
        "tests/fixtures/images/example_image.tif",
        connector
            .path()
            .to_str()
            .expect("connector path should be UTF-8"),
        "--connector-schema",
        schema.path().to_str().expect("schema path should be UTF-8"),
        "--out",
        output.path().to_str().expect("output path should be UTF-8"),
    ]);

    assert_eq!(output.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("targetSchemaVersion '2' is not supported")
    );
}
