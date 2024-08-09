use crate::{Result, Error};
use crate::file_types::common::{FileType, determine_file_type};
use crate::utils::validate::validate_args;
use crate::file_types::{geojson, shapefile, geoparquet};
use crate::pg::ops::{check_table_exists, drop_table};

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
    pub srid: Option<usize>,

    /// Mode: overwrite, append, fail. Optional.
    #[arg(short, long)]
    pub mode: Option<String>,
}

pub fn run() -> Result<()> {
    let mut args = Cli::parse();
    validate_args(&args)?;

    // If not provided srid will default to 4326
    if args.srid.is_none() {
        args.srid.get_or_insert(4326);
    }

    let file_type = determine_file_type(&args.input)?;

    // If mode not present, check if table exists
    if args.mode.is_none() {
        check_table_exists(&args.table, &args.schema, &args.uri)?;
    } else if let Some(mode) = &args.mode {
        match mode.as_str() {
            "overwrite" => {
                drop_table(&args.table, &args.schema, &args.uri)?;
            }
            "fail" => {
                check_table_exists(&args.table, &args.schema, &args.uri)?;
            }
            _ => {
                println!("Mode not supported ✘");
                return Err(Error::FailedValidation("Mode not supported ✘".into()));
            }
        }
    };
    
    match file_type {
        FileType::GeoJson => {
            geojson::insert_data(args)?;
        }
        FileType::Shapefile => {
            shapefile::insert_data(args)?;
        }
        FileType::GeoParquet => {
            geoparquet::insert_data(args)?;
        }
    };

    Ok(())
}
