use crate::{Error, Result};

use arrow_array::cast::AsArray;
use arrow_array::{StringArray, BooleanArray};
use arrow_data::ArrayData;
use arrow_schema::DataType;
use geoarrow::array::AsGeometryArray;
use geoarrow::io::parquet::read_geoparquet_async;
use geoarrow::io::parquet::GeoParquetReaderOptions;
use geoarrow::table::GeoTable;
use geoarrow::trait_::GeometryArrayAccessor;
use postgres::types::Type;
use tokio::fs::File;

use crate::file_types::common::NameAndType;
use crate::pg::binary_copy::infer_geometry_type;
use crate::pg::ops::prepare_postgis;
use crate::utils::cli::Cli;

pub fn insert_data(args: Cli) -> Result<()> {
    // Currently static batch size. In time, this should be dynamic
    let batch_size = 500;
    // Read geoparquet file using tokio runtime
    let runtime = tokio::runtime::Runtime::new()?;
    let geotable = runtime.block_on(read_geoparquet(&args.input, batch_size))?;
    let mut non_geometry_types = determine_non_geometry_types(&geotable)?;

    // Prepare database
    prepare_postgis(&args, &non_geometry_types)?;

    // Determine geometry type
    let names_and_types = determine_geometry_type(&args, &mut non_geometry_types)?;

    // Process geotable
    process_geotable(geotable)?;

    Ok(())
}

fn determine_geometry_type<'a>(
    args: &Cli,
    names_and_types: &'a mut Vec<NameAndType>,
) -> Result<&'a mut Vec<NameAndType>> {
    // Get geometry type
    let geom_type = infer_geometry_type(&args.table, &args.schema, &args.uri)?;
    // Add geometry type to types
    names_and_types.push(NameAndType {
        name: "geometry".to_string(),
        data_type: geom_type,
    });

    Ok(names_and_types)
}

fn determine_non_geometry_types(geotable: &GeoTable) -> Result<Vec<NameAndType>> {
    let schema = geotable.schema();
    let fields = schema.fields();

    let mut names_and_types: Vec<NameAndType> = Vec::new();
    for field in fields {
        let name = field.name();
        let data_type = field.data_type();
        match data_type {
            DataType::Utf8 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::VARCHAR,
                });
            }
            DataType::Float16 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::FLOAT8,
                });
            }
            DataType::Float32 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::FLOAT8,
                });
            }
            DataType::Float64 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::FLOAT8,
                });
            }
            DataType::Int8 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::INT8,
                });
            }
            DataType::Int16 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::INT8,
                });
            }
            DataType::Int32 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::INT8,
                });
            }
            DataType::Int64 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::INT8,
                });
            }
            DataType::UInt8 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::INT8,
                });
            }
            DataType::UInt16 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::INT8,
                });
            }
            DataType::UInt32 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::INT8,
                });
            }
            DataType::UInt64 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::INT8,
                });
            }
            DataType::Null => {
                // Do nothing
            }
            DataType::Binary => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::BYTEA,
                });
            }
            DataType::Boolean => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::BOOL,
                });
            }
            DataType::Date32 => {
                names_and_types.push(NameAndType {
                    name: name.to_string(),
                    data_type: Type::DATE,
                });
            }
            #[allow(unused)]
            DataType::List(l) => {
                if name == "geometry" {
                    // This may be a geometry column which is process elsewhere
                    continue;
                }
                // Unsure. Do nothing for now.
            }
            _ => println!("Data type '{:?}' not supported ✘", data_type),
        }
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
    let mut columns: Vec<ArrayData> = Vec::new();

    // Process non-geometry columns
    geotable.batches().into_iter().for_each(|batch| {
        let fields = batch.schema_ref().fields();
        for field in fields {
            let col_name = field.name();
            if let Some(column) = batch.column_by_name(col_name) {
                let column_type = column.data_type();
                match column_type {
                    DataType::Utf8 => {
                        if let Some(utf8_array) = column.as_any().downcast_ref::<StringArray>() {
                            utf8_array.into_iter().for_each(|utf8| {
                                if let Some(value) = utf8 {
                                    // Is data
                                } else {
                                    // Is null
                                }
                            });
                        }
                    }
                    DataType::Boolean => {
                        if let Some(bool_array) = column.as_any().downcast_ref::<BooleanArray>() {
                            bool_array.into_iter().for_each(|boolean| {
                                if let Some(value) = boolean {
                                    // Is data
                                } else {
                                    // Is null
                                }
                            });
                        }
                    }
                    DataType::Int8 => {
                        let int8_array = column.as_primitive::<arrow_array::types::Int8Type>();
                        int8_array.into_iter().for_each(|int8| {
                            if int8.is_some() {
                                // Is data
                            } else {
                                // Is null
                            }
                        });
                    }
                    DataType::Int16 => {
                        let int16_array = column.as_primitive::<arrow_array::types::Int16Type>();
                        int16_array.into_iter().for_each(|int16| {
                            if int16.is_some() {
                                // Is data
                            } else {
                                // Is null
                            }
                        });
                    }
                    DataType::Int32 => {
                        let int32_array = column.as_primitive::<arrow_array::types::Int32Type>();
                        int32_array.into_iter().for_each(|int32| {
                            if int32.is_some() {
                                // Is data
                            } else {
                                // Is null
                            }
                        });
                    }
                    DataType::Float16 => {
                        let float16_array = column.as_primitive::<arrow_array::types::Float16Type>();
                        float16_array.into_iter().for_each(|float16| {
                            if float16.is_some() {
                                // Is data
                            } else {
                                // Is null
                            }
                        });
                    }
                    DataType::Float32 => {
                        let float32_array = column.as_primitive::<arrow_array::types::Float32Type>();
                        float32_array.into_iter().for_each(|float32| {
                            if float32.is_some() {
                                // Is data
                            } else {
                                // Is null
                            }
                        });
                    }
                    DataType::Float64 => {
                        let float64_array = column.as_primitive::<arrow_array::types::Float64Type>();
                        float64_array.into_iter().for_each(|float64| {
                            if float64.is_some() {
                                // Is data
                            } else {
                                // Is null
                            }
                        });
                    }
                    DataType::Date32 => {
                        let date32_array = column.as_primitive::<arrow_array::types::Date32Type>();
                        date32_array.into_iter().for_each(|date32| {
                            if date32.is_some() {
                                // Is data
                            } else {
                                // Is null
                            }
                        });
                    }
                    DataType::Date64 => {
                        let date64_array = column.as_primitive::<arrow_array::types::Date64Type>();
                        date64_array.into_iter().for_each(|date64| {
                            if date64.is_some() {
                                // Is data
                            } else {
                                // Is null
                            }
                        });
                    }
                    _ => {
                        // Handle other types or do something else
                    }
                }
            }
        }
    });

    // Process geometry column
    if geotable.geometry().is_ok() {
        let geometry_column = geotable.geometry().unwrap();
        let geometry_type = geometry_column.data_type();
        let geometry_chunks = geometry_column.geometry_chunks();
        for geometry_chunk in geometry_chunks.into_iter() {
            match geometry_type {
                geoarrow::datatypes::GeoDataType::Point(_) => {
                    let geoarrow_point = geometry_chunk.as_point();
                    for point in geoarrow_point.iter_geo() {
                        // Do something with point
                    }
                }
                geoarrow::datatypes::GeoDataType::MultiPoint(_) => {
                    let geoarrow_multipoint = geometry_chunk.as_multi_point();
                    for multipoint in geoarrow_multipoint.iter_geo() {
                        // Do something with multipoint
                    }
                }
                geoarrow::datatypes::GeoDataType::LineString(_) => {
                    let geoarrow_line = geometry_chunk.as_line_string();
                    for line in geoarrow_line.iter_geo() {
                        // Do something with line
                    }
                }
                geoarrow::datatypes::GeoDataType::MultiLineString(_) => {
                    let geoarrow_multiline = geometry_chunk.as_multi_line_string();
                    for multiline in geoarrow_multiline.iter_geo() {
                        // Do something with multiline
                    }
                }
                geoarrow::datatypes::GeoDataType::Polygon(_) => {
                    let geoarrow_poly = geometry_chunk.as_polygon();
                    for poly in geoarrow_poly.iter_geo() {
                        // Do something with poly
                    }
                }
                geoarrow::datatypes::GeoDataType::MultiPolygon(_) => {
                    let geoarrow_multipoly = geometry_chunk.as_multi_polygon();
                    for multipoly in geoarrow_multipoly.iter_geo() {
                        // Do something with multipoly
                    }
                }
                _ => println!("Geometry type not supported ✘"),
            }
        }
    }

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
