use crate::{Result, Error};
use super::cli::Cli;
use std::path::Path;

// Validate the file path
pub fn validate_args(args: &Cli) -> Result<()> {

    // Check input file exists
    if !Path::new(&args.input).exists() {
        return Err(Error::FailedValidation("Input file does not exist ✘".into()));
    }

    // Check URL is not empty
    if args.uri.is_empty() {
        return Err(Error::FailedValidation("URL is empty ✘".into()));
    }

    // Check table is not empty
    if args.table.is_empty() {
        return Err(Error::FailedValidation("Table is empty ✘".into()));
    }

    // Check if srid is 4326 or 3857
    if let Some(srid) = args.srid {
        if srid != 4326 && srid != 3857 {
            return Err(Error::FailedValidation("SRID must be 4326 or 3857 ✘".into()));
        }
    }

    Ok(())

}

#[cfg(test)]
mod tests {
    use super::*;

    // Without schema
    #[test]
    fn test_validate_args() {
        let args = Cli {
            input: "examples/geojson/spain.geojson".to_string(),
            uri: "postgresql://localhost:5432/postgis".to_string(),
            table: "points".to_string(),
            schema: None,
            srid: None,
            mode: None
        };
        assert!(validate_args(&args).is_ok());
    }

    // With schema
    #[test]
    fn test_validate_args_with_schema() {
        let args = Cli {
            input: "examples/shapefile/andalucia.shp".to_string(),
            uri: "postgresql://localhost:5432/postgis".to_string(),
            table: "points".to_string(),
            schema: Some("gis".to_string()),
            srid: Some(4326),
            mode: None
        };
        assert!(validate_args(&args).is_ok());
    }

    // Invalid srid
    #[test]
    fn test_validate_args_invalid_srid() {
        let args = Cli {
            input: "examples/shapefile/andalucia.shp".to_string(),
            uri: "postgresql://localhost:5432/postgis".to_string(),
            table: "points".to_string(),
            schema: Some("gis".to_string()),
            srid: Some(1234),
            mode: None
        };
        assert!(validate_args(&args).is_err());
    }

    // File does not exist
    #[test]
    fn test_validate_args_file_does_not_exist() {
        let args = Cli {
            input: "examples/shapefile/does_not_exist.shp".to_string(),
            uri: "postgresql://localhost:5432/postgis".to_string(),
            table: "points".to_string(),
            schema: Some("gis".to_string()),
            srid: Some(4326),
            mode: None
        };
        assert!(validate_args(&args).is_err());
    }

    // Url is empty
    #[test]
    fn test_validate_args_empty_url() {
        let args = Cli {
            input: "examples/shapefile/andalucia.shp".to_string(),
            uri: "".to_string(),
            table: "points".to_string(),
            schema: Some("gis".to_string()),
            srid: Some(4326),
            mode: None
        };
        assert!(validate_args(&args).is_err());
    }
}
