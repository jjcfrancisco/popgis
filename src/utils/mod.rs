use crate::Result;
use std::path::Path;

pub mod cli;
pub mod geo;
mod validate;

// Create enum of supported file types
pub enum FileType {
    Shapefile,
    GeoJson,
}

fn determine_file_type(input_file: &str) -> Result<FileType> {
    let file_extension = Path::new(input_file)
        .extension()
        .expect("No file extension found");
    let file_extension_str = file_extension
        .to_str()
        .expect("Could not convert file extension to string");
    match file_extension_str {
        "shp" => Ok(FileType::Shapefile),
        "json" => Ok(FileType::GeoJson),
        _ => Err("Unsupported file type".into()),
    }
}

fn determine_column_types() -> Result<()> {
    Ok(())
}
