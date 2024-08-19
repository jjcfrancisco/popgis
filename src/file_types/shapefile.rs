use crate::{Error, Result};

use postgres::types::Type;
use shapefile::dbase::FieldValue;
use std::collections::HashMap;
use wkb::geom_to_wkb;

use crate::file_types::common::{AcceptedTypes, NameAndType};
use crate::file_types::geo::to_geo;
use crate::pg::binary_copy::{infer_geometry_type, insert_row, prepare_query, Wkb};
use crate::pg::ops::prepare_postgis;
use crate::utils::cli::Cli;

pub fn insert_data(args: Cli) -> Result<()> {
    // Determine data types of the input file
    let mut non_geometry_types = determine_non_geometry_types(&args)?;
    // Prepare database
    prepare_postgis(&args, &non_geometry_types)?;
    // Determine geometry type
    let names_and_types = determine_geometry_types(&args, &mut non_geometry_types)?; 

    // Get data types
    let mut types: Vec<Type> = Vec::new();
    for column in names_and_types.iter() {
        types.push(column.data_type.clone());
    }

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
        let query = prepare_query(&args, names_and_types);
        insert_row(row, query, &types, &args)?;
    }

    println!("Data sucessfully inserted into database ✓");

    Ok(())
}

fn determine_geometry_types<'a>(
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

fn determine_non_geometry_types(args: &Cli) -> Result<Vec<NameAndType>> {
    let mut table_config: HashMap<String, Type> = HashMap::new();
    let mut reader = shapefile::Reader::from_path(&args.input)?;
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

    let mut names_and_types: Vec<NameAndType> = Vec::new();
    for (column_name, data_type) in table_config.iter() {
        names_and_types.push(NameAndType {
            name: column_name.clone(),
            data_type: data_type.clone(),
        });
    }

    Ok(names_and_types)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_file_data_types() {
        let args = Cli {
            input: "examples/shapefile/andalucia.shp".to_string(),
            uri: "postgresql://postgres:password@localhost:5432/postgres".to_string(),
            schema: None,
            table: "andalucia".to_string(),
            srid: None,
            mode: None,
        };
        let data_types = determine_non_geometry_types(&args).unwrap();
        assert_eq!(data_types.len(), 2);
        for data_type in data_types {
            if data_type.name == "x" || data_type.name == "y" {
                assert_eq!(data_type.data_type, Type::FLOAT8);
            }
        }
    }
}
