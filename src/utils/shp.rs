use crate::Result;
use postgres::types::Type;
use shapefile::dbase::FieldValue;

use crate::pg::binary_copy::Wkb;
use crate::utils::to_geo;
use crate::utils::{AcceptedTypes, NewTableTypes, Row, Rows};
use wkb::geom_to_wkb;

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
                        data_type: Type::FLOAT8,
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
    let mut rows = Rows::new();
    let mut reader = shapefile::Reader::from_path(file_path)?;
    for shape_record in reader.iter_shapes_and_records() {
        let mut row = Row::new();
        let (shape, record) = shape_record?;
        for (_, data_type) in record.into_iter() {
            match data_type {
                FieldValue::Numeric(value) => {
                    if let Some(value) = value {
                        row.add(AcceptedTypes::Float(value));
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

        let geom = to_geo(&shape)?;
        let wkb = geom_to_wkb(&geom).expect("Failed to insert node into database");
        row.add(AcceptedTypes::Geometry(Wkb { geometry: wkb }));
        rows.add(row);
    }

    Ok(rows)
}
