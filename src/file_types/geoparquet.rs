use std::any::Any;

use crate::{Error, Result};

use geoarrow::io::parquet::read_geoparquet_async;
use geoarrow::io::parquet::GeoParquetReaderOptions;
use geoarrow::table::GeoTable;
use tokio::fs::File;
use postgres::types::Type;
use std::collections::HashMap;
use std::any::TypeId;

use crate::utils::cli::Cli;
use crate::file_types::common::NameAndType;
use crate::pg::ops::prepare_postgis;
use crate::pg::binary_copy::infer_geometry_type;

pub fn insert_data(args: Cli) -> Result<()> {
    // Currently static batch size. In time, this should be dynamic
    let batch_size = 1000;
    // Read geoparquet file using tokio runtime
    let runtime = tokio::runtime::Runtime::new()?;
    let geotable = runtime.block_on(read_geoparquet(&args.input, batch_size))?;
    let file_data_types = determine_file_data_types(&geotable)?;

    // Determine data types of the input file
    // Prepare database
    prepare_postgis(&args, &file_data_types)?;

    // Get data types
    let mut types: Vec<Type> = Vec::new();
    for column in file_data_types.iter() {
        types.push(column.data_type.clone());
    }
    // Get geometry type
    let geom_type = infer_geometry_type(&args.table, &args.schema, &args.uri)?;
    // Add geometry type to types
    types.push(geom_type);

    // Process geotable
    process_geotable(geotable)?;
    Ok(())
}

fn determine_file_data_types(geotable: &GeoTable) -> Result<Vec<NameAndType>> {
    let schema = geotable.schema();
    let mut table_config: HashMap<String, Type> = HashMap::new();
    for field in schema.fields() {
        let name = field.name();
        let data_type = field.data_type();
        if data_type.type_id() == TypeId::of::<f64>() {
            table_config.insert(name.to_string(), Type::FLOAT8);
        } else if data_type.type_id() == TypeId::of::<i64>() {
            table_config.insert(name.to_string(), Type::INT8);
        } else if data_type.type_id() == TypeId::of::<String>() {
            table_config.insert(name.to_string(), Type::VARCHAR);
        } else if data_type.type_id() == TypeId::of::<bool>() {
            table_config.insert(name.to_string(), Type::BOOL);
        }
    }

    let mut names_and_types: Vec<NameAndType> = Vec::new();
    for (name, data_type) in table_config.iter() {
        names_and_types.push(NameAndType {
            name: name.to_string(),
            data_type: data_type.clone(),
        });
    }

    Ok(names_and_types)

}

async fn read_geoparquet(file: &str, batch_size: usize) -> Result<GeoTable> {
    let file = File::open(file).await.unwrap();
    let options = GeoParquetReaderOptions::new(batch_size, Default::default());
    let geotable = read_geoparquet_async(file, options).await?;

    Ok(geotable)
}

pub fn process_geotable(geotable: GeoTable) -> Result<()> {
    let geom_column = geotable.geometry()?;
    let geom_type = geom_column.data_type();
    println!("{:?}", geom_type);
    let chunks = geom_column.geometry_chunks();

    // To polygons
    // for chunk in chunks {
    //     let polys = chunk.as_polygon();
    //     polys.iter().for_each(|poly| {
    //         if poly.is_some() {
    //             let poly = poly.unwrap();
    //             let geo_geom = poly.to_geo_geometry();
    //             println!("{:?}", geo_geom);
    //         }
    //     });
    // }

    Ok(())
}

// Write test for reading geoparquet
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_geoparquet() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let file_path = "examples/geoparquet/example.parquet";
        let batch_size = 1000;
        let result = runtime
            .block_on(read_geoparquet(file_path, batch_size))
            .unwrap();
        assert_eq!(result.len(), 5);
    }
}
