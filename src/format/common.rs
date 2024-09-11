use crate::{Error, Result};

use postgres::types::Type;
use std::path::Path;

use crate::pg::binary_copy::Wkb;

// Struct to hold column name and data type
#[derive(Debug)]
pub struct NewTableTypes {
    pub column_name: String,
    pub data_type: Type,
}

#[derive(Debug)]
pub struct Row {
    pub columns: Vec<AcceptedTypes>,
}

#[derive(Debug)]
pub struct Rows {
    pub row: Vec<Row>,
}

impl Row {
    pub fn new() -> Self {
        Row {
            columns: Vec::new(),
        }
    }
    pub fn add(&mut self, column: AcceptedTypes) {
        self.columns.push(column);
    }
}

impl Rows {
    pub fn new() -> Self {
        Rows { row: Vec::new() }
    }
    pub fn add(&mut self, row: Row) {
        self.row.push(row);
    }
}

// Enum to hold accepted data types
#[derive(Debug)]
pub enum AcceptedTypes {
    Int(Option<i32>),
    Float(Option<f64>),
    Double(Option<f32>),
    Text(Option<String>),
    Bool(Option<bool>),
    Array(Option<Vec<String>>),
    Geometry(Option<Wkb>),
}

// Create enum of supported file types
#[derive(Debug, PartialEq)]
pub enum FileType {
    Shapefile,
    GeoJson,
    Osmpbf,
}

pub fn determine_file_type(input_file: &str) -> Result<FileType> {
    let file_extension = Path::new(input_file)
        .extension()
        .expect("❌ No file extension found");
    let file_extension_str = file_extension
        .to_str()
        .expect("❌ Could not convert file extension to string");
    match file_extension_str {
        "shp" => Ok(FileType::Shapefile),
        "geojson" => Ok(FileType::GeoJson),
        "pbf" => Ok(FileType::Osmpbf),
        _ => Err(Error::UnsupportedFileExtension(
            "❌ Unsupported file type".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_file_type() {
        let shapefile = "examples/shapefile/andalucia.shp";
        let geojson = "examples/geojson/spain.geojson";
        assert_eq!(determine_file_type(shapefile).unwrap(), FileType::Shapefile);
        assert_eq!(determine_file_type(geojson).unwrap(), FileType::GeoJson);
    }
}
