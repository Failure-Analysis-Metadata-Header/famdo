use famh_model::{from_str, v1, v2};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_v1_example()?;
    run_v2_example()?;
    Ok(())
}

fn run_v1_example() -> Result<(), Box<dyn std::error::Error>> {
    let v1_json = r#"{
      "General Section": {
        "File Name": "sample-v1.tif",
        "Coordinates Sub Section": {
          "Stage Coordinates X Y Z": {
            "Value": [1.0, 2.0, 3.0],
            "Unit": "mm"
          }
        }
      },
      "Method Specific": {
        "Scanning Electron Microscopy": {}
      },
      "Custom Top Level": true
    }"#;

    let parsed = v1::FaMetadataHeader::from_str(v1_json)?;
    let stage_coords = parsed
        .general_section
        .coordinates_sub_section
        .as_ref()
        .and_then(|c| c.stage_coordinates_x_y_z.as_ref())
        .and_then(|v| v.value.as_ref());

    println!("== V1 ==");
    println!(
        "Custom extra field: {:?}",
        parsed.extra.get("Custom Top Level")
    );
    println!("Stage coordinates: {stage_coords:#?}");
    println!("Round-trip JSON:\n{}\n", parsed.to_string_pretty()?);
    Ok(())
}

fn run_v2_example() -> Result<(), Box<dyn std::error::Error>> {
    let v2_json = r#"{
      "generalSection": {
        "fileName": "sample-v2.tif",
        "timeStamp": "2026-02-20T10:00:00+00:00",
        "manufacturer": "ZEISS",
        "toolName": "GeminiSEM 500",
        "method": "SEM",
        "imageWidth": { "value": 640, "unit": "px" }
      },
      "methodSpecific": {
        "opticalMicroscopy": {
          "objectiveMagnification": "50x"
        }
      },
      "unknownTopLevel": { "kept": true }
    }"#;

    let parsed: v2::FaMetadataHeader = from_str(v2_json)?;
    let file_name = parsed
        .general_section
        .as_ref()
        .and_then(|g| g.file_name.as_deref());

    println!("== V2 ==");
    println!("File name: {file_name:?}");
    println!(
        "Unknown top-level field: {:?}",
        parsed.extra.get("unknownTopLevel")
    );
    println!("Round-trip JSON:\n{}\n", parsed.to_string_pretty()?);
    Ok(())
}
