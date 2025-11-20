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

    let mut tags = Vec::new();
    let tiff_tags = extract_tiff_metadata_tags(&decoder)?;

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

pub fn extract_tiff_metadata_tags(decoder: &Decoder) -> Result<Value, Box<dyn std::error::Error>> {
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
