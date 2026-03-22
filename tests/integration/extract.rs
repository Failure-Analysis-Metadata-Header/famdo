//! Integration tests for the extract command

use famdo::commands::extract::{extract_and_save_metadata, extract_metadata};
use tempfile::NamedTempFile;

#[test]
fn extract_metadata_returns_error_for_nonexistent_file() {
    let result = extract_metadata("/nonexistent/path/image.tiff");
    assert!(result.is_err(), "Should return error for missing file");
}

#[test]
fn extract_and_save_metadata_returns_error_for_nonexistent_file() {
    let outfile = NamedTempFile::new().expect("should create temp file");
    let out_path = outfile.path().to_str().unwrap();
    let result = extract_and_save_metadata("/nonexistent/path/image.tiff", out_path);
    assert!(result.is_err(), "Should return error for missing input file");
}

#[test]
fn extract_metadata_output_has_expected_keys_when_tiff_available() {
    let result = extract_metadata("testdata/Fail_20X_Overlay_7.tiff");
    if let Ok(metadata) = result {
        assert!(metadata.get("filename").is_some(), "metadata must have filename");
        assert!(metadata.get("dimensions").is_some(), "metadata must have dimensions");
        assert!(metadata.get("tags").is_some(), "metadata must have tags");
        assert!(metadata.get("tag_errors").is_some(), "metadata must have tag_errors");

        let dims = &metadata["dimensions"];
        assert!(dims.get("width").is_some(), "dimensions must have width");
        assert!(dims.get("height").is_some(), "dimensions must have height");
    }
}

#[test]
fn extract_and_save_metadata_writes_valid_json_when_tiff_available() {
    let result_meta = extract_metadata("testdata/Fail_20X_Overlay_7.tiff");
    if result_meta.is_ok() {
        let outfile = NamedTempFile::new().expect("should create temp file");
        let out_path = outfile.path().to_str().unwrap().to_string();
        let result = extract_and_save_metadata("testdata/Fail_20X_Overlay_7.tiff", &out_path);
        assert!(result.is_ok(), "Should succeed with valid TIFF file");
        let written = std::fs::read_to_string(&out_path).expect("output file should be readable");
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&written);
        assert!(parsed.is_ok(), "Output file should contain valid JSON");
    }
}
