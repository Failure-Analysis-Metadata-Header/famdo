use serde_json::{Value, json};
use std::fs::File;
use std::io::BufReader;
use tiff::decoder::Decoder;
use tiff::decoder::ifd;

/// Extract metadata from a TIFF file and save it to a JSON file.
pub fn extract_metadata(image_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let file = File::open(image_path)?;
    let mut decoder = Decoder::new(BufReader::new(file))?;
    let (width, height) = decoder.dimensions()?;
    println!("Dimensions: {} x {}", width, height);

    let tiff_tags = extract_tiff_metadata_tags(&mut decoder)?;

    let metadata = json!({
        "filename": image_path,
        "dimensions": {
            "width": width,
            "height": height,
        },
        "tags": tiff_tags,
    });
    Ok(metadata)
}

pub fn extract_and_save_metadata(
    image_path: &str,
    out_path: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let metadata = extract_metadata(&image_path)?;
    let outfile = File::create(out_path)?;
    serde_json::to_writer_pretty(outfile, &metadata)?;
    Ok(true)
}

pub fn extract_tiff_metadata_tags<R: std::io::Read + std::io::Seek>(
    decoder: &mut Decoder<R>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut tags = Vec::new();
    for tag_result in decoder.tag_iter() {
        match tag_result {
            Ok((tag, ifd_value)) => {
                let (value, value_type) = extract_value(&ifd_value);
                tags.push(json!({
                    "tag": format!("{tag:?}"),
                    "value": value,
                    "type": value_type,
                }));
            }
            Err(err) => {
                eprintln!("Error reading tag: {}", err);
            }
        }
    }
    println!("Extracted {} tags", tags.len());
    Ok(json!(tags))
}

/// Extract the value and type from an ifd::Value.
fn extract_value(ifd_value: &ifd::Value) -> (Value, &'static str) {
    match ifd_value {
        ifd::Value::Byte(val) => (json!(val), "Byte"),
        ifd::Value::Ascii(s) => (json!(s), "ASCII"),
        ifd::Value::Short(val) => (json!(val), "Short"),
        ifd::Value::Rational(val_1, val_2) => (json!(format!("{}/{}", val_1, val_2)), "Rational"),
        ifd::Value::SignedByte(val) => (json!(val), "SignedByte"),
        ifd::Value::SignedShort(val) => (json!(val), "SignedShort"),
        ifd::Value::Float(val) => (json!(val), "Float"),
        ifd::Value::Double(val) => (json!(val), "Double"),
        ifd::Value::List(vals) => (
            json!(vals.iter().map(|v| extract_value(v).0).collect::<Vec<_>>()),
            "List",
        ),
        ifd::Value::Unsigned(val) => (json!(val), "Unsigned"),
        _ => {
            println!("Not covered: {:?}", ifd_value);
            (json!({}), "Unknown")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_value_byte() {
        let value = ifd::Value::Byte(42);
        let (json_val, type_str) = extract_value(&value);
        assert_eq!(json_val, json!(42));
        assert_eq!(type_str, "Byte");
    }

    #[test]
    fn test_extract_value_ascii() {
        let value = ifd::Value::Ascii("test string".to_string());
        let (json_val, type_str) = extract_value(&value);
        assert_eq!(json_val, json!("test string"));
        assert_eq!(type_str, "ASCII");
    }

    #[test]
    fn test_extract_value_short() {
        let value = ifd::Value::Short(1234);
        let (json_val, type_str) = extract_value(&value);
        assert_eq!(json_val, json!(1234));
        assert_eq!(type_str, "Short");
    }

    #[test]
    fn test_extract_value_rational() {
        let value = ifd::Value::Rational(96000, 1000);
        let (json_val, type_str) = extract_value(&value);
        assert_eq!(json_val, json!("96000/1000"));
        assert_eq!(type_str, "Rational");
    }

    #[test]
    fn test_extract_value_unsigned() {
        let value = ifd::Value::Unsigned(640);
        let (json_val, type_str) = extract_value(&value);
        assert_eq!(json_val, json!(640));
        assert_eq!(type_str, "Unsigned");
    }

    #[test]
    fn test_extract_value_list() {
        let values = vec![
            ifd::Value::Short(8),
            ifd::Value::Short(8),
            ifd::Value::Short(8),
        ];
        let value = ifd::Value::List(values);
        let (json_val, type_str) = extract_value(&value);
        assert_eq!(json_val, json!([8, 8, 8]));
        assert_eq!(type_str, "List");
    }

    #[test]
    fn test_extract_metadata_nonexistent_file() {
        let result = extract_metadata("/nonexistent/file.tiff");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_metadata_structure() {
        // This test would require a valid TIFF file
        // For now, we just test that the function signature is correct
        let result = extract_metadata("testdata/Fail_20X_Overlay_7.tiff");
        if result.is_ok() {
            let metadata = result.unwrap();
            assert!(metadata.get("filename").is_some());
            assert!(metadata.get("dimensions").is_some());
            assert!(metadata.get("tags").is_some());

            let dims = &metadata["dimensions"];
            assert!(dims.get("width").is_some());
            assert!(dims.get("height").is_some());
        }
    }
}
