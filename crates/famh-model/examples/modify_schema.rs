use famh_model::{Numeric, v1};
use serde_json::json;

use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file =
        File::open("/home/kevin/repos/famdo/testdata/v1/examples/rectangle_example_sem.json")?;
    let reader = BufReader::new(file);

    let mut schema: v1::FaMetadataHeader = v1::FaMetadataHeader::from_reader(reader)?;

    schema.general_section.file_name = Some("test_famh_model_creation.tiff".to_string());

    let method = schema
        .method_specific
        .scanning_electron_microscopy
        .get_or_insert_with(Default::default);

    method.accelerating_voltage = Some(v1::LegacyNumberWithUnit {
        value: Some(Numeric::Float(799.5)),
        unit: Some("kV".to_string()),
        extra: Default::default(),
    });

    schema
        .extra
        .insert("Edited by".to_string(), json!("load_schema example"));
    method.extra.insert("Operator".to_string(), json!("Kevin"));

    let out = File::create("/home/kevin/repos/famdo/testdata/test.json")?;
    let writer = BufWriter::new(out);
    schema.to_writer_pretty(writer)?;

    println!("{:#?}", schema);
    Ok(())
}
