use crate::Result;
use postgres::types::Type;
use shapefile::dbase::FieldValue;

use crate::utils::to_geo;
use crate::utils::{AcceptedTypes, NewTableTypes, Row, Rows};

pub fn determine_data_types(file_path: &str) -> Result<Vec<NewTableTypes>> {
    let mut table_config: Vec<NewTableTypes> = Vec::new();
    let mut reader = shapefile::Reader::from_path(file_path)?;
    for shape_record in reader.iter_shapes_and_records() {
        let (_, record) = shape_record.unwrap();
        for (column_name, data_type) in record.into_iter() {
            match data_type {
                FieldValue::Numeric(_) => {
                    table_config.push(NewTableTypes {
                        column_name,
                        data_type: Type::INT8,
                    });
                }
                FieldValue::Float(_) => {
                    table_config.push(NewTableTypes {
                        column_name,
                        data_type: Type::FLOAT8,
                    });
                }
                FieldValue::Double(_) => {
                    table_config.push(NewTableTypes {
                        column_name,
                        data_type: Type::FLOAT8,
                    });
                }
                FieldValue::Integer(_) => {
                    table_config.push(NewTableTypes {
                        column_name,
                        data_type: Type::INT8,
                    });
                }
                FieldValue::Character(_) => {
                    table_config.push(NewTableTypes {
                        column_name,
                        data_type: Type::TEXT,
                    });
                }
                FieldValue::Logical(_) => {
                    table_config.push(NewTableTypes {
                        column_name,
                        data_type: Type::BOOL,
                    });
                }
                _ => println!("Type currently not supported"),
            }
        }
        break;
    }
    Ok(table_config)
}

pub fn read_shapefile(file_path: &str) -> Result<Rows> {
    // Create a new vector that can hold any data type from below
    let mut rows = Rows::new();
    let mut reader = shapefile::Reader::from_path(file_path)?;
    for shape_record in reader.iter_shapes_and_records() {
        let mut row = Row::new();
        let (shape, record) = shape_record?;
        for (_, data_type) in record.into_iter() {
            match data_type {
                FieldValue::Numeric(value) => {
                    if let Some(value) = value {
                        row.add(AcceptedTypes::Int(value as i64));
                    }
                }
                FieldValue::Float(value) => {
                    if let Some(value) = value {
                        row.add(AcceptedTypes::Double(value as f64));
                    }
                }
                FieldValue::Double(value) => {
                    row.add(AcceptedTypes::Double(value));
                }
                FieldValue::Integer(value) => {
                    row.add(AcceptedTypes::Int(value as i64));
                }
                FieldValue::Character(value) => {
                    if let Some(value) = value {
                        row.add(AcceptedTypes::Text(value));
                    }
                }
                FieldValue::Logical(value) => {
                    if let Some(value) = value {
                        row.add(AcceptedTypes::Bool(value));
                    }
                }
                _ => println!("Type currently not supported"),
            }
        }

        // Transform shape to geometry then push
        let geometry = to_geo(&shape)?;
        row.add(AcceptedTypes::Geometry(geometry));
        rows.add(row);
    }

    Ok(rows)
}
