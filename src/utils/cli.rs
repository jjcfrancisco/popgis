use crate::format::common::{determine_file_type, FileType};
use crate::format::shapefile;
use crate::format::{geojson, osmpbf};
use crate::pg::binary_copy::{infer_geom_type, insert_rows};
use crate::pg::crud::{check_table_exists, create_schema, create_table, drop_table, get_stmt};
use crate::utils::validate::validate_args;
use crate::{Error, Result};

use clap::Parser;

/// A blazing fast way to insert GeoJSON, ShapeFiles & OsmPBF into a PostGIS database
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

    /// Mode: overwrite or fail. Optional.
    #[arg(short, long, default_value = "fail")]
    pub mode: Option<String>,

    /// Reproject: reproject to 4326 or 3857. Optional.
    #[arg(short, long)]
    pub reproject: Option<i32>,
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
        FileType::Shapefile => (
            shapefile::read_shapefile(&args)?,
            shapefile::determine_data_types(&args.input)?,
        ),
        FileType::GeoJson => (
            geojson::read_geojson(&args)?,
            geojson::determine_data_types(&args.input)?,
        ),
        FileType::Osmpbf => {
            args.srid = Some(4326); // OsmPbf files are always in 4326
            (osmpbf::read_osmpbf(&args)?, osmpbf::determine_data_types()?)
        }
    };

    // If mode not present, check if table exists
    let create = if args.mode.is_none() {
        check_table_exists(&args.table, &args.schema, &args.uri)?;
        true
    } else if let Some(mode) = &args.mode {
        match mode.as_str() {
            "overwrite" => {
                drop_table(&args.table, &args.schema, &args.uri)?;
                true
            }
            "fail" => {
                check_table_exists(&args.table, &args.schema, &args.uri)?;
                true
            }
            _ => {
                println!("Mode not supported ✘");
                return Err(Error::FailedValidation("❌ Mode not supported".into()));
            }
        }
    } else {
        false
    };

    if create {
        // If schema present, create schema
        if let Some(schema) = &args.schema {
            create_schema(schema, &args.uri)?;
        }
        //
        if let Some(reproject) = args.reproject {
            create_table(&args.table, &args.schema, &config, &args.uri, reproject)?
        } else if args.reproject.is_none() {
            create_table(
                &args.table,
                &args.schema,
                &config,
                &args.uri,
                args.srid.unwrap(),
            )?
        }
    }

    let stmt = get_stmt(&args.table, &args.schema, &args.uri)?;
    let geom_type = infer_geom_type(stmt)?;
    insert_rows(
        &rows,
        &config,
        geom_type,
        &args.uri,
        &args.schema,
        &args.table,
    )?;

    Ok(())
}
