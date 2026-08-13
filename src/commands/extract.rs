use serde_json::{Number, Value, json};
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read, Seek};
use tiff::decoder::Decoder;
use tiff::decoder::ifd;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TiffDimensions {
    pub width: u32,
    pub height: u32,
}

impl TiffDimensions {
    fn to_json(&self) -> Value {
        json!({
            "width": self.width,
            "height": self.height,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TiffTag {
    pub id: u16,
    pub name: String,
    pub value: Value,
    pub value_type: String,
}

impl TiffTag {
    fn to_json(&self) -> Value {
        json!({
            "id": self.id,
            "tag": self.name,
            "value": self.value,
            "type": self.value_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TiffIfd {
    pub index: usize,
    pub dimensions: TiffDimensions,
    pub tags: Vec<TiffTag>,
}

impl TiffIfd {
    fn to_json(&self) -> Value {
        json!({
            "index": self.index,
            "dimensions": self.dimensions.to_json(),
            "tags": self.tags.iter().map(TiffTag::to_json).collect::<Vec<_>>(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TiffExtractionDiagnostic {
    pub ifd: usize,
    pub severity: String,
    pub message: String,
}

impl TiffExtractionDiagnostic {
    fn to_json(&self) -> Value {
        json!({
            "ifd": self.ifd,
            "severity": self.severity,
            "message": self.message,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtractedTiffMetadata {
    pub filename: String,
    pub ifds: Vec<TiffIfd>,
    pub diagnostics: Vec<TiffExtractionDiagnostic>,
}

impl ExtractedTiffMetadata {
    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Serialize the current output shape, retaining the first-IFD fields
    /// used by existing consumers while exposing every discovered IFD.
    pub fn to_json(&self) -> Value {
        let first_ifd = self
            .ifds
            .first()
            .expect("TIFF extraction always produces at least one IFD");

        json!({
            "filename": self.filename,
            "dimensions": first_ifd.dimensions.to_json(),
            "tags": first_ifd.tags.iter().map(TiffTag::to_json).collect::<Vec<_>>(),
            "ifds": self.ifds.iter().map(TiffIfd::to_json).collect::<Vec<_>>(),
            "diagnostics": self.diagnostics.iter().map(TiffExtractionDiagnostic::to_json).collect::<Vec<_>>(),
        })
    }
}

/// Extract metadata from a TIFF file into the typed representation shared by
/// the `extract` and `map` commands.
pub fn extract_tiff_metadata(
    image_path: &str,
) -> Result<ExtractedTiffMetadata, Box<dyn std::error::Error>> {
    let file = File::open(image_path)?;
    let mut decoder = Decoder::new(BufReader::new(file))?;
    let mut ifds = Vec::new();
    let mut diagnostics = Vec::new();
    let mut ifd_index = 0;

    loop {
        let (width, height) = decoder.dimensions()?;
        let (tags, mut ifd_diagnostics) = extract_current_ifd_tags(&mut decoder, ifd_index);
        ifds.push(TiffIfd {
            index: ifd_index,
            dimensions: TiffDimensions { width, height },
            tags,
        });
        diagnostics.append(&mut ifd_diagnostics);

        if !decoder.more_images() {
            break;
        }

        decoder.next_image()?;
        ifd_index += 1;
    }

    Ok(ExtractedTiffMetadata {
        filename: image_path.to_owned(),
        ifds,
        diagnostics,
    })
}

/// Extract metadata from a TIFF file and return the established JSON shape.
pub fn extract_metadata(image_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    Ok(extract_tiff_metadata(image_path)?.to_json())
}

pub fn extract_and_save_metadata(
    image_path: &str,
    out_path: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    extract_and_save_metadata_with_report(image_path, out_path)?;
    Ok(true)
}

pub fn extract_and_save_metadata_with_report(
    image_path: &str,
    out_path: &str,
) -> Result<ExtractedTiffMetadata, Box<dyn std::error::Error>> {
    let metadata = extract_tiff_metadata(image_path)?;
    let outfile = File::create(out_path)?;
    serde_json::to_writer_pretty(outfile, &metadata.to_json())?;
    Ok(metadata)
}

/// Extract the current IFD's tags in the legacy JSON array shape.
///
/// Callers that need extraction diagnostics should use [`extract_tiff_metadata`].
pub fn extract_tiff_metadata_tags<R: Read + Seek>(
    decoder: &mut Decoder<R>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let (tags, diagnostics) = extract_current_ifd_tags(decoder, 0);
    if let Some(diagnostic) = diagnostics.first() {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidData,
            diagnostic.message.clone(),
        )));
    }

    Ok(Value::Array(
        tags.iter().map(TiffTag::to_json).collect::<Vec<_>>(),
    ))
}

fn extract_current_ifd_tags<R: Read + Seek>(
    decoder: &mut Decoder<R>,
    ifd_index: usize,
) -> (Vec<TiffTag>, Vec<TiffExtractionDiagnostic>) {
    let mut tags = Vec::new();
    let mut diagnostics = Vec::new();

    for tag_result in decoder.tag_iter() {
        match tag_result {
            Ok((tag, ifd_value)) => {
                let (value, value_type) = extract_value(&ifd_value);
                if value_type == "Unknown" {
                    diagnostics.push(TiffExtractionDiagnostic {
                        ifd: ifd_index,
                        severity: "error".to_owned(),
                        message: format!(
                            "TIFF tag {} ({tag:?}) uses an unsupported value variant: {ifd_value:?}",
                            tag.to_u16()
                        ),
                    });
                }
                tags.push(TiffTag {
                    id: tag.to_u16(),
                    name: format!("{tag:?}"),
                    value,
                    value_type: value_type.to_owned(),
                });
            }
            Err(error) => diagnostics.push(TiffExtractionDiagnostic {
                ifd: ifd_index,
                severity: "error".to_owned(),
                message: format!("Could not read a TIFF tag: {error}"),
            }),
        }
    }

    (tags, diagnostics)
}

/// Extract the JSON-compatible value and TIFF value type from an IFD value.
fn extract_value(ifd_value: &ifd::Value) -> (Value, &'static str) {
    match ifd_value {
        ifd::Value::Byte(value) => (json!(value), "Byte"),
        ifd::Value::Ascii(value) => (json!(value), "ASCII"),
        ifd::Value::Short(value) => (json!(value), "Short"),
        ifd::Value::Rational(numerator, denominator) => {
            (json!(format!("{numerator}/{denominator}")), "Rational")
        }
        ifd::Value::RationalBig(numerator, denominator) => {
            (json!(format!("{numerator}/{denominator}")), "RationalBig")
        }
        ifd::Value::SRational(numerator, denominator) => {
            (json!(format!("{numerator}/{denominator}")), "SRational")
        }
        ifd::Value::SRationalBig(numerator, denominator) => {
            (json!(format!("{numerator}/{denominator}")), "SRationalBig")
        }
        ifd::Value::SignedByte(value) => (json!(value), "SignedByte"),
        ifd::Value::SignedShort(value) => (json!(value), "SignedShort"),
        ifd::Value::Signed(value) => (json!(value), "Signed"),
        ifd::Value::SignedBig(value) => (json!(value), "SignedBig"),
        ifd::Value::Unsigned(value) => (json!(value), "Unsigned"),
        ifd::Value::UnsignedBig(value) => (json!(value), "UnsignedBig"),
        ifd::Value::Float(value) => (json_number_or_string(*value as f64), "Float"),
        ifd::Value::Double(value) => (json_number_or_string(*value), "Double"),
        ifd::Value::List(values) => (
            Value::Array(values.iter().map(|value| extract_value(value).0).collect()),
            "List",
        ),
        ifd::Value::Ifd(value) => (json!(value), "Ifd"),
        ifd::Value::IfdBig(value) => (json!(value), "IfdBig"),
        _ => (json!(format!("{ifd_value:?}")), "Unknown"),
    }
}

fn json_number_or_string(value: f64) -> Value {
    Number::from_f64(value)
        .map(Value::Number)
        .unwrap_or_else(|| Value::String(value.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tiff::encoder::{TiffEncoder, colortype};

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
    fn test_extract_value_signed_and_unsigned_wide_values() {
        let values = [
            (ifd::Value::Signed(-12), json!(-12), "Signed"),
            (ifd::Value::SignedBig(-34), json!(-34), "SignedBig"),
            (ifd::Value::Unsigned(56), json!(56), "Unsigned"),
            (ifd::Value::UnsignedBig(78), json!(78), "UnsignedBig"),
        ];

        for (value, expected_json, expected_type) in values {
            let (json_val, type_str) = extract_value(&value);
            assert_eq!(json_val, expected_json);
            assert_eq!(type_str, expected_type);
        }
    }

    #[test]
    fn test_extract_value_rational() {
        let values = [
            (
                ifd::Value::Rational(96000, 1000),
                json!("96000/1000"),
                "Rational",
            ),
            (
                ifd::Value::RationalBig(96000, 1000),
                json!("96000/1000"),
                "RationalBig",
            ),
            (
                ifd::Value::SRational(-96000, 1000),
                json!("-96000/1000"),
                "SRational",
            ),
            (
                ifd::Value::SRationalBig(-96000, 1000),
                json!("-96000/1000"),
                "SRationalBig",
            ),
        ];

        for (value, expected_json, expected_type) in values {
            let (json_val, type_str) = extract_value(&value);
            assert_eq!(json_val, expected_json);
            assert_eq!(type_str, expected_type);
        }
    }

    #[test]
    fn test_extract_value_float_and_double() {
        let values = [
            (ifd::Value::Float(1.5), json!(1.5), "Float"),
            (ifd::Value::Double(2.5), json!(2.5), "Double"),
        ];

        for (value, expected_json, expected_type) in values {
            let (json_val, type_str) = extract_value(&value);
            assert_eq!(json_val, expected_json);
            assert_eq!(type_str, expected_type);
        }
    }

    #[test]
    fn test_extract_value_non_finite_float_is_explicit() {
        let (json_val, type_str) = extract_value(&ifd::Value::Double(f64::NAN));
        assert_eq!(json_val, json!("NaN"));
        assert_eq!(type_str, "Double");
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
    fn test_extract_value_ifd_pointers() {
        let values = [
            (ifd::Value::Ifd(42), json!(42), "Ifd"),
            (ifd::Value::IfdBig(84), json!(84), "IfdBig"),
        ];

        for (value, expected_json, expected_type) in values {
            let (json_val, type_str) = extract_value(&value);
            assert_eq!(json_val, expected_json);
            assert_eq!(type_str, expected_type);
        }
    }

    #[test]
    fn test_extract_metadata_nonexistent_file() {
        let result = extract_metadata("/nonexistent/file.tiff");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_metadata_structure() {
        let metadata = extract_metadata("tests/fixtures/images/example_image.tif").unwrap();

        assert!(metadata.get("filename").is_some());
        assert!(metadata.get("dimensions").is_some());
        assert!(metadata.get("tags").is_some());
        assert!(metadata.get("ifds").is_some());
        assert_eq!(metadata["diagnostics"], json!([]));

        let dims = &metadata["dimensions"];
        assert_eq!(dims["width"], json!(640));
        assert_eq!(dims["height"], json!(480));

        let tags = metadata["tags"].as_array().unwrap();
        assert!(!tags.is_empty());
        assert!(tags.iter().all(|tag| tag["id"].is_u64()));
        assert!(tags.iter().all(|tag| tag.get("tag").is_some()));

        let ifds = metadata["ifds"].as_array().unwrap();
        assert_eq!(ifds.len(), 1);
        assert_eq!(ifds[0]["index"], json!(0));
    }

    #[test]
    fn test_extract_metadata_reads_all_ifds() {
        let mut file = NamedTempFile::new().unwrap();
        {
            let mut encoder = TiffEncoder::new(file.as_file_mut()).unwrap();
            encoder
                .write_image::<colortype::Gray8>(2, 3, &[0, 1, 2, 3, 4, 5])
                .unwrap();
            encoder
                .write_image::<colortype::Gray8>(4, 5, &[0; 20])
                .unwrap();
        }

        let metadata = extract_tiff_metadata(file.path().to_str().unwrap()).unwrap();
        assert_eq!(metadata.ifds.len(), 2);
        assert_eq!(metadata.ifds[0].index, 0);
        assert_eq!(
            metadata.ifds[0].dimensions,
            TiffDimensions {
                width: 2,
                height: 3
            }
        );
        assert_eq!(
            metadata.ifds[1].dimensions,
            TiffDimensions {
                width: 4,
                height: 5
            }
        );
        assert_eq!(
            metadata.to_json()["dimensions"],
            json!({"width": 2, "height": 3})
        );
    }

    #[test]
    fn test_extract_and_save_metadata_reports_output_errors() {
        let file = NamedTempFile::new().unwrap();
        let result = extract_and_save_metadata_with_report(
            "tests/fixtures/images/example_image.tif",
            file.path().to_str().unwrap(),
        );
        assert!(result.is_ok());

        let mut contents = String::new();
        std::fs::File::open(file.path())
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert!(contents.contains("\"ifds\""));
    }

    #[test]
    fn test_extract_and_save_metadata_rejects_directory_output() {
        let directory = tempfile::tempdir().unwrap();
        let result = extract_and_save_metadata_with_report(
            "tests/fixtures/images/example_image.tif",
            directory.path().to_str().unwrap(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_tiff_metadata_tags_returns_legacy_array() {
        let file = File::open("tests/fixtures/images/example_image.tif").unwrap();
        let mut decoder = Decoder::new(BufReader::new(file)).unwrap();
        let tags = extract_tiff_metadata_tags(&mut decoder).unwrap();
        assert!(tags.is_array());
        assert!(!tags.as_array().unwrap().is_empty());
    }

    #[test]
    fn test_extract_metadata_preserves_tag_read_diagnostics() {
        let diagnostic = TiffExtractionDiagnostic {
            ifd: 1,
            severity: "error".to_owned(),
            message: "test diagnostic".to_owned(),
        };
        let metadata = ExtractedTiffMetadata {
            filename: "test.tif".to_owned(),
            ifds: vec![TiffIfd {
                index: 0,
                dimensions: TiffDimensions {
                    width: 1,
                    height: 1,
                },
                tags: Vec::new(),
            }],
            diagnostics: vec![diagnostic],
        };

        assert!(metadata.has_diagnostics());
        assert_eq!(
            metadata.to_json()["diagnostics"][0]["message"],
            json!("test diagnostic")
        );
    }
}
