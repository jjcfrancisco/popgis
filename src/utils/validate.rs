use crate::Result;
use super::cli::Cli;
use std::path::Path;

// Validate the file path
pub fn validate_args(args: &Cli) -> Result<()> {

    // Check input file exists
    if !Path::new(&args.input).exists() {
        return Err("Input file does not exist".into());
    }

    // Check URL is not empty
    if args.uri.is_empty() {
        return Err("URL is empty".into());
    }

    // Check that there is a connection to the URL


    Ok(())

}
