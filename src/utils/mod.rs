use crate::Result;
use std::path::Path;
use postgres::types::Type;
use geo::Coord;
use shapefile::Shape;
use crate::pg::binary_copy::Wkb;

pub mod cli;
pub mod shp;
mod validate;

// Struct to hold column name and data type
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
    pub rows: Vec<Row>,
}

impl Row {
    pub fn new() -> Self {
        Row { columns: Vec::new() }
    }
    pub fn add(&mut self, column: AcceptedTypes) {
        self.columns.push(column);
    }
}

impl Rows {
    pub fn new() -> Self {
        Rows { rows: Vec::new() }
    }
    pub fn add(&mut self, row: Row) {
        self.rows.push(row);
    }
}

// Enum to hold accepted data types
#[derive(Debug)]
pub enum AcceptedTypes {
    Int(i64),
    // Float(f64),
    Double(f64),
    Text(String),
    Bool(bool),
    Geometry(Wkb),
}

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

pub fn to_geo(shape: &Shape) -> Result<geo::Geometry<f64>> {
    match shape {
        Shape::Point(p) => Ok(geo::Point::new(p.x, p.y).into()),
        Shape::Polyline(p) => {
            let mut coords: Vec<Coord> = Vec::new();
            for part in p.parts().iter() {
                for point in part.iter() {
                    coords.push(Coord::from((point.x, point.y)));
                }
            }
            Ok(geo::LineString::new(coords).into())
        }
        Shape::Polygon(p) => {
            let mut outer_placeholder: Vec<(f64, f64)> = Vec::new();
            let mut inner_rings: Vec<geo::LineString> = Vec::new();

            for ring_type in p.rings() {
                match ring_type {
                    //Gather all outer rings
                    shapefile::PolygonRing::Outer(out) => {
                        out.iter().for_each(|p| outer_placeholder.push((p.x, p.y)))
                    }
                    //Gather all inner rings
                    shapefile::PolygonRing::Inner(inn) => {
                        let mut inner_ring: Vec<(f64, f64)> = Vec::new();
                        inn.iter().for_each(|p| inner_ring.push((p.x, p.y)));
                        let ls = geo::LineString::from(inner_ring);
                        inner_rings.push(ls);
                    }
                }
            }

            let outer_ring = geo::LineString::from(outer_placeholder);
            if inner_rings.is_empty() {
                let poly = geo::Polygon::new(outer_ring, vec![]);
                Ok(geo::Geometry::from(poly))
            } else {
                let poly = geo::Polygon::new(outer_ring, inner_rings);
                Ok(geo::Geometry::from(poly))
            }
        }
        _ => Err("Unsupported shape type".into()),
    }
}
