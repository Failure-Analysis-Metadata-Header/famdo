use serde_json::{Value, json};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tiff::decoder::Decoder;
use tiff::decoder::ifd;

use crate::utils::write_bytes_atomically;

/// Extract metadata from a TIFF file and save it to a JSON file.
pub fn extract_metadata(image_path: impl AsRef<Path>) -> Result<Value, Box<dyn std::error::Error>> {
    let image_path = image_path.as_ref();
    let file = File::open(image_path)?;
    let mut decoder = Decoder::new(BufReader::new(file))?;
    let (width, height) = decoder.dimensions()?;

    let (tiff_tags, tag_errors) = extract_tiff_metadata_tags(&mut decoder)?;

    let metadata = json!({
        "filename": image_path.display().to_string(),
        "dimensions": {
            "width": width,
            "height": height,
        },
        "tags": tiff_tags,
        "tag_errors": tag_errors,
    });
    Ok(metadata)
}

pub fn extract_and_save_metadata(
    image_path: impl AsRef<Path>,
    out_path: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = extract_metadata(image_path)?;
    let bytes = serde_json::to_vec_pretty(&metadata)?;
    write_bytes_atomically(out_path, &bytes)?;
    Ok(())
}

pub fn extract_tiff_metadata_tags<R: std::io::Read + std::io::Seek>(
    decoder: &mut Decoder<R>,
) -> Result<(Value, usize), Box<dyn std::error::Error>> {
    let mut tags = Vec::new();
    let mut tag_error_count: usize = 0;
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
            Err(_err) => {
                tag_error_count += 1;
            }
        }
    }
    Ok((json!(tags), tag_error_count))
}

/// Extract the value and type from an ifd::Value.
fn extract_value(ifd_value: &ifd::Value) -> (Value, &'static str) {
    match ifd_value {
        ifd::Value::Byte(val) => (json!(val), "Byte"),
        ifd::Value::Ascii(s) => (json!(s), "ASCII"),
        ifd::Value::Short(val) => (json!(val), "Short"),
        ifd::Value::SignedByte(val) => (json!(val), "SignedByte"),
        ifd::Value::SignedShort(val) => (json!(val), "SignedShort"),
        ifd::Value::Signed(val) => (json!(val), "Signed"),
        ifd::Value::Unsigned(val) => (json!(val), "Unsigned"),
        ifd::Value::SignedBig(val) => (json!(val), "SignedBig"),
        ifd::Value::UnsignedBig(val) => (json!(val), "UnsignedBig"),
        ifd::Value::Float(val) => (json!(val), "Float"),
        ifd::Value::Double(val) => (json!(val), "Double"),
        ifd::Value::Rational(num, den) => {
            (json!({ "numerator": num, "denominator": den }), "Rational")
        }
        ifd::Value::RationalBig(num, den) => (
            json!({ "numerator": num, "denominator": den }),
            "RationalBig",
        ),
        ifd::Value::SRational(num, den) => {
            (json!({ "numerator": num, "denominator": den }), "SRational")
        }
        ifd::Value::SRationalBig(num, den) => (
            json!({ "numerator": num, "denominator": den }),
            "SRationalBig",
        ),
        ifd::Value::Ifd(offset) => (json!(offset), "Ifd"),
        ifd::Value::IfdBig(offset) => (json!(offset), "IfdBig"),
        ifd::Value::List(vals) => (
            json!(vals.iter().map(|v| extract_value(v).0).collect::<Vec<_>>()),
            "List",
        ),
        _ => (json!({}), "Unknown"),
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
        assert_eq!(
            json_val,
            json!({ "numerator": 96000u32, "denominator": 1000u32 })
        );
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
    fn test_extract_value_signed() {
        let value = ifd::Value::Signed(-42);
        let (json_val, type_str) = extract_value(&value);
        assert_eq!(json_val, json!(-42));
        assert_eq!(type_str, "Signed");
    }

    #[test]
    fn test_extract_value_signed_big() {
        let value = ifd::Value::SignedBig(-100_000);
        let (json_val, type_str) = extract_value(&value);
        assert_eq!(json_val, json!(-100_000i64));
        assert_eq!(type_str, "SignedBig");
    }

    #[test]
    fn test_extract_value_unsigned_big() {
        let value = ifd::Value::UnsignedBig(100_000);
        let (json_val, type_str) = extract_value(&value);
        assert_eq!(json_val, json!(100_000u64));
        assert_eq!(type_str, "UnsignedBig");
    }

    #[test]
    fn test_extract_value_srational() {
        let value = ifd::Value::SRational(-96000, 1000);
        let (json_val, type_str) = extract_value(&value);
        assert_eq!(
            json_val,
            json!({ "numerator": -96000i32, "denominator": 1000i32 })
        );
        assert_eq!(type_str, "SRational");
    }

    #[test]
    fn test_extract_value_rational_big() {
        let value = ifd::Value::RationalBig(96000, 1000);
        let (json_val, type_str) = extract_value(&value);
        assert_eq!(
            json_val,
            json!({ "numerator": 96000u64, "denominator": 1000u64 })
        );
        assert_eq!(type_str, "RationalBig");
    }

    #[test]
    fn test_extract_value_ifd() {
        let value = ifd::Value::Ifd(42);
        let (json_val, type_str) = extract_value(&value);
        assert_eq!(json_val, json!(42));
        assert_eq!(type_str, "Ifd");
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
        let result = extract_metadata("testdata/example_image.tif");
        if let Ok(metadata) = result {
            assert!(metadata.get("filename").is_some());
            assert!(metadata.get("dimensions").is_some());
            assert!(metadata.get("tags").is_some());
            assert!(metadata.get("tag_errors").is_some());

            let dims = &metadata["dimensions"];
            assert_eq!(dims["width"], json!(640));
            assert_eq!(dims["height"], json!(480));
        }
    }
}
