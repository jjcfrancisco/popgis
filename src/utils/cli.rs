use crate::Result;
use crate::utils::{determine_file_type, FileType};
use crate::utils::validate::validate_args;
use crate::utils::shp::{read_shapefile, determine_data_types};
use crate::pg::{create_connection, create_table};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long)]
    pub uri: String,

    #[arg(short, long)]
    pub table: String,

    #[arg(short, long)]
    pub schema: Option<String>,
}

pub fn run() -> Result<()> {
    let args = Cli::parse();
    validate_args(&args)?;

    let file_type = determine_file_type(&args.input)?;
    let data = match file_type {
        FileType::Shapefile => {
            read_shapefile(&args.input)?
        }
        FileType::GeoJson => {
            unimplemented!()
        }
    };
    let config = determine_data_types(&args.input)?;
    let mut client = create_connection(&args.uri)?;
    create_table(&args.table, args.schema, &config, &mut client)?;
    // Insert data into table using Rows

    Ok(())
}
