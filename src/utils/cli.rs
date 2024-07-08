use crate::Result;
use crate::file_types::common::{FileType, determine_file_type};
use crate::utils::validate::validate_args;
use crate::file_types::shapefile::{read_shapefile, determine_data_types};
use crate::pg::crud::{create_table, create_schema};
use crate::pg::binary_copy::{infer_geom_type, insert_rows};

use clap::Parser;

/// A blazing fast way to insert GeoJSON & ShapeFiles into a PostGIS database 
#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {

    /// Input file path, either shapefile or geojson
    #[arg(short, long)]
    pub input: String,

    /// PostgreSQL URI
    #[arg(short, long)]
    pub uri: String,

    /// Table name
    #[arg(short, long)]
    pub table: String,

    /// Schema name to create table in. Optional.
    #[arg(short, long)]
    pub schema: Option<String>,
}

pub fn run() -> Result<()> {
    let args = Cli::parse();
    validate_args(&args)?;

    let file_type = determine_file_type(&args.input)?;
    let rows = match file_type {
        FileType::Shapefile => {
            read_shapefile(&args.input)?
        }
        FileType::GeoJson => {
            unimplemented!()
        }
    };
    let config = determine_data_types(&args.input)?;
    // If schema present, create schema
    if let Some(schema) = &args.schema {
        create_schema(&schema, &args.uri)?;
    }
    let stmt = create_table(&args.table, &args.schema, &config, &args.uri)?;
    let geom_type = infer_geom_type(stmt)?;
    insert_rows(&rows, &config, geom_type, &args.uri, &args.schema, &args.table)?;

    Ok(())
}
