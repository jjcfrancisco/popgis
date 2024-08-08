use crate::{Error, Result};

use postgres::types::Type;
use shapefile::dbase::FieldValue;
use std::collections::HashMap;

use crate::file_types::common::{AcceptedTypes, NameAndType};
use crate::file_types::geo::to_geo;
use crate::pg::binary_copy::{infer_geometry_type, insert_row, Wkb};
use crate::pg::ops::prepare_postgis;
use crate::utils::cli::Cli;

use wkb::geom_to_wkb;

pub fn insert_data(args: Cli) -> Result<()> {
    // Determine data types of the input file
    let file_data_types = determine_file_data_types(&args.input)?;
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

    let mut reader = shapefile::Reader::from_path(&args.input)?;
    println!("Inserting data into database...");
    for shape_record in reader.iter_shapes_and_records() {
        let mut row: Vec<AcceptedTypes> = Vec::new();
        let (shape, record) = shape_record?;
        for (_, data_type) in record.into_iter() {
            match data_type {
                FieldValue::Numeric(value) => {
                    row.push(AcceptedTypes::Float(value));
                }
                FieldValue::Float(value) => {
                    row.push(AcceptedTypes::Double(value));
                }
                FieldValue::Double(value) => {
                    row.push(AcceptedTypes::Float(Some(value)));
                }
                FieldValue::Integer(value) => {
                    row.push(AcceptedTypes::Int(Some(value)));
                }
                FieldValue::Character(value) => {
                    row.push(AcceptedTypes::Text(value));
                }
                FieldValue::Logical(value) => {
                    row.push(AcceptedTypes::Bool(value));
                }
                _ => println!("Type currently not supported ✘"),
            }
        }

        let geom = to_geo(&shape)?;
        let wkb = geom_to_wkb(&geom).expect("Failed to insert node into database ✘");
        row.push(AcceptedTypes::Geometry(Some(Wkb { geometry: wkb })));
        insert_row(row, &file_data_types, &types, &args)?;
    }

    println!("Data sucessfully inserted into database ✓");

    Ok(())
}

pub fn determine_file_data_types(file_path: &str) -> Result<Vec<NameAndType>> {
    let mut table_config: HashMap<String, Type> = HashMap::new();
    let mut reader = shapefile::Reader::from_path(file_path)?;
    for shape_record in reader.iter_shapes_and_records() {
        let (_, record) = shape_record.unwrap();
        for (column_name, data_type) in record.into_iter() {
            match data_type {
                FieldValue::Numeric(_) => {
                    if table_config.contains_key(&column_name)
                        && table_config[&column_name] == Type::FLOAT8
                    {
                        continue;
                    } else if table_config.contains_key(&column_name)
                        && table_config[&column_name] != Type::INT8
                    {
                        return Err(Error::MixedDataTypes(
                            "Column contains mixed data types ✘".to_string(),
                        ));
                    } else {
                        table_config.insert(column_name, Type::FLOAT8);
                    }
                }
                FieldValue::Float(_) => {
                    if table_config.contains_key(&column_name)
                        && table_config[&column_name] == Type::FLOAT8
                    {
                        continue;
                    } else if table_config.contains_key(&column_name)
                        && table_config[&column_name] != Type::INT8
                    {
                        return Err(Error::MixedDataTypes(
                            "Column contains mixed data types ✘".to_string(),
                        ));
                    } else {
                        table_config.insert(column_name, Type::FLOAT8);
                    }
                }
                FieldValue::Double(_) => {
                    if table_config.contains_key(&column_name)
                        && table_config[&column_name] == Type::FLOAT8
                    {
                        continue;
                    } else if table_config.contains_key(&column_name)
                        && table_config[&column_name] != Type::INT8
                    {
                        return Err(Error::MixedDataTypes(
                            "Column contains mixed data types ✘".to_string(),
                        ));
                    } else {
                        table_config.insert(column_name, Type::FLOAT8);
                    }
                }
                FieldValue::Integer(_) => {
                    if table_config.contains_key(&column_name)
                        && table_config[&column_name] == Type::INT8
                    {
                        continue;
                    } else if table_config.contains_key(&column_name)
                        && table_config[&column_name] != Type::FLOAT8
                    {
                        return Err(Error::MixedDataTypes(
                            "Column contains mixed data types ✘".to_string(),
                        ));
                    } else {
                        table_config.insert(column_name, Type::INT8);
                    }
                }
                FieldValue::Character(_) => {
                    if table_config.contains_key(&column_name)
                        && table_config[&column_name] == Type::TEXT
                    {
                        continue;
                    } else if table_config.contains_key(&column_name)
                        && table_config[&column_name] != Type::INT8
                    {
                        return Err(Error::MixedDataTypes(
                            "Column contains mixed data types ✘".to_string(),
                        ));
                    } else {
                        table_config.insert(column_name, Type::TEXT);
                    }
                }
                FieldValue::Logical(_) => {
                    if table_config.contains_key(&column_name)
                        && table_config[&column_name] == Type::BOOL
                    {
                        continue;
                    } else if table_config.contains_key(&column_name)
                        && table_config[&column_name] != Type::INT8
                    {
                        return Err(Error::MixedDataTypes(
                            "Column contains mixed data types ✘".to_string(),
                        ));
                    } else {
                        table_config.insert(column_name, Type::BOOL);
                    }
                }
                _ => println!("Type currently not supported ✘"),
            }
        }
    }

    let mut data_types: Vec<NameAndType> = Vec::new();
    for (column_name, data_type) in table_config.iter() {
        data_types.push(NameAndType {
            name: column_name.clone(),
            data_type: data_type.clone(),
        });
    }

    Ok(data_types)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_file_data_types() {
        let file_path = "examples/shapefile/andalucia.shp";
        let data_types = determine_file_data_types(file_path).unwrap();
        assert_eq!(data_types.len(), 2);
        for data_type in data_types {
            if data_type.name == "x" || data_type.name == "y" {
                assert_eq!(data_type.data_type, Type::FLOAT8);
            }
        }
    }
}
