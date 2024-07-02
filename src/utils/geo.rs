use crate::Result;
use shapefile;
use shapefile::dbase::FieldValue;
use postgres::types::Type;

struct NewTableTypes {
    column_name: String,
    data_type: Type,
}

pub fn read_shapefile(file_path: &str) -> Result<()> {
    let mut reader = shapefile::Reader::from_path(file_path)?;
    for shape_record in reader.iter_shapes_and_records() {
        let (_, record) = shape_record?;
        for (column_name, data_type) in record.into_iter() {
            match data_type {
                FieldValue::Numeric(value) => {
                    Type::INT8;
                    println!("Column name: {}, Numeric: {:?}", column_name, value)
                },
                FieldValue::Float(value) => {
                    println!("Column name: {}, String: {:?}", column_name, value)
                },
                FieldValue::Double(value) => {
                    println!("Column name: {}, String: {:?}", column_name, value)
                },
                FieldValue::Integer(value) => {
                    println!("Column name: {}, String: {:?}", column_name, value)
                },
                FieldValue::Character(value) => {
                    println!("Column name: {}, String: {:?}", column_name, value)
                },
                FieldValue::Logical(value) => {
                    println!("Column name: {}, String: {:?}", column_name, value)
                },
                _ => println!("Type currently not supported"),
            }
        }
        break
    }
    Ok(())
}
