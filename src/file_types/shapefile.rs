use std::collections::HashMap;

use crate::Result;
use postgres::types::Type;
use shapefile::dbase::FieldValue;

use crate::file_types::common::{AcceptedTypes, NewTableTypes, Row, Rows};
use crate::file_types::geo::to_geo;
use crate::pg::binary_copy::Wkb;
use wkb::geom_to_wkb;

pub fn determine_data_types(file_path: &str) -> Result<Vec<NewTableTypes>> {
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
                        return Err("Column contains mixed data types ✘".into());
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
                        return Err("Column contains mixed data types ✘".into());
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
                        return Err("Column contains mixed data types ✘".into());
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
                        return Err("Column contains mixed data types ✘".into());
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
                        return Err("Column contains mixed data types ✘".into());
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
                        return Err("Column contains mixed data types ✘".into());
                    } else {
                        table_config.insert(column_name, Type::BOOL);
                    }
                }
                _ => println!("Type currently not supported ✘"),
            }
        }
    }

    let mut data_types: Vec<NewTableTypes> = Vec::new();
    for (column_name, data_type) in table_config.iter() {
        data_types.push(NewTableTypes {
            column_name: column_name.clone(),
            data_type: data_type.clone(),
        });
    }

    Ok(data_types)
}

pub fn read_shapefile(file_path: &str) -> Result<Rows> {
    let mut rows = Rows::new();
    let mut reader = shapefile::Reader::from_path(file_path)?;
    for shape_record in reader.iter_shapes_and_records() {
        let mut row = Row::new();
        let (shape, record) = shape_record?;
        for (_, data_type) in record.into_iter() {
            match data_type {
                FieldValue::Numeric(value) => {
                    row.add(AcceptedTypes::Float(value));
                }
                FieldValue::Float(value) => {
                    row.add(AcceptedTypes::Double(value));
                }
                FieldValue::Double(value) => {
                    row.add(AcceptedTypes::Float(Some(value)));
                }
                FieldValue::Integer(value) => {
                    row.add(AcceptedTypes::Int(Some(value)));
                }
                FieldValue::Character(value) => {
                    row.add(AcceptedTypes::Text(value));
                }
                FieldValue::Logical(value) => {
                    row.add(AcceptedTypes::Bool(value));
                }
                _ => println!("Type currently not supported ✘"),
            }
        }

        let geom = to_geo(&shape)?;
        let wkb = geom_to_wkb(&geom).expect("Failed to insert node into database ✘");
        row.add(AcceptedTypes::Geometry(Some(Wkb { geometry: wkb })));
        rows.add(row);
    }

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_data_types() {
        let file_path = "examples/shapefile/andalucia.shp";
        let data_types = determine_data_types(file_path).unwrap();
        assert_eq!(data_types.len(), 2);
        for data_type in data_types {
            if data_type.column_name == "x" || data_type.column_name == "y" {
                assert_eq!(data_type.data_type, Type::FLOAT8);
            }
        }
    }

    #[test]
    fn test_read_shapefile() {
        let file_path = "examples/shapefile/andalucia.shp";
        let rows = read_shapefile(file_path).unwrap();
        assert_eq!(rows.row.len(), 36);
    }
}
