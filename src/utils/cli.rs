use crate::Result;
use crate::utils::{determine_file_type, FileType};
use crate::utils::validate::validate_args;
use crate::utils::geo::read_shapefile;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long)]
    pub uri: String,
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
            println!("GeoJson");
        }
    };

    Ok(())
}
