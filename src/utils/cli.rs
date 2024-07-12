use crate::Result;
use crate::file_types::common::{FileType, determine_file_type};
use crate::utils::validate::validate_args;
use crate::file_types::shapefile;
use crate::file_types::geojson;
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

    /// Srid, if not provided, will default to 4326
    #[arg(long)]
    pub srid: Option<i32>,
}

pub fn run() -> Result<()> {
    let mut args = Cli::parse();
    validate_args(&args)?;

    // If not provided srid will default to 4326
    if args.srid.is_none() {
        args.srid.get_or_insert(4326);
    }

    let file_type = determine_file_type(&args.input)?;
    let (rows, config) = match file_type {
        FileType::Shapefile => {
            (shapefile::read_shapefile(&args.input)?, shapefile::determine_data_types(&args.input)?)
        }
        FileType::GeoJson => {
            (geojson::read_geojson(&args.input)?, geojson::determine_data_types(&args.input)?)
        }
    };
    // If schema present, create schema
    if let Some(schema) = &args.schema {
        create_schema(&schema, &args.uri)?;
    }
    let stmt = if let Some(srid) = args.srid {
        create_table(&args.table, &args.schema, &config, &args.uri, srid)?
    } else {
        create_table(&args.table, &args.schema, &config, &args.uri, 4326)?
    };
    let geom_type = infer_geom_type(stmt)?;
    insert_rows(&rows, &config, geom_type, &args.uri, &args.schema, &args.table)?;

    Ok(())
}
