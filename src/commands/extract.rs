use std::fs::File;
use std::io::BufReader;
use tiff::decoder::Decoder;

pub fn exctract_metadata(
    image_path: &str,
    out_path: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let file = File::open(image_path)?;
    let mut decoder = Decoder::new(BufReader::new(file))?;
    let (width, height) = decoder.dimensions()?;
    println!("Dimensions: {} x {}", width, height);

    for tag_result in decoder.tag_iter() {
        match tag_result {
            Ok((tag, idf_value)) => {
                println!("{:?}: {:?}", tag, idf_value);
            }
            Err(err) => {
                eprintln!("Error reading tag: {}", err);
            }
        }
    }
    Ok(true)
}
