use crate::Result;
use bytes::BytesMut;
use postgres::types::to_sql_checked;
use postgres::types::{IsNull, ToSql, Type};
use postgres::Statement;
use std::error::Error;

use postgres::binary_copy::BinaryCopyInWriter;
use postgres::CopyInWriter;

use crate::pg::crud::create_connection;
use crate::file_types::common::{AcceptedTypes, NewTableTypes, Rows};

#[derive(Debug)]
pub struct Wkb {
    pub geometry: Vec<u8>,
}

impl ToSql for Wkb {
    fn to_sql(
        &self,
        _: &Type,
        out: &mut BytesMut,
    ) -> std::result::Result<IsNull, Box<dyn Error + Send + Sync>> {
        out.extend_from_slice(&self.geometry);
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "geometry"
    }

    to_sql_checked!();
}

pub fn infer_geom_type(stmt: Statement) -> Result<Type> {
    let column = stmt.columns().get(0).expect("Failed to get columns");
    Ok(column.type_().clone())
}

pub fn insert_rows<'a>(
    rows: &Rows,
    config: &Vec<NewTableTypes>,
    geom_type: Type,
    uri: &str,
    schema: &Option<String>,
    table: &str,
) -> Result<()> {
    // Create connection
    let mut client = create_connection(uri)?;

    // Prepare types for binary copy
    let mut types: Vec<Type> = Vec::new();
    for column in config.iter() {
        types.push(column.data_type.clone());
    }
    types.push(geom_type);

    // Binary copy in writer
    let mut query = String::from("COPY ");
    if let Some(schema) = schema {
        query.push_str(&format!("{}.{}", schema, table));
    } else {
        query.push_str(&table);
    }
    query.push_str(" (");
    for column in config.iter() {
        query.push_str(&format!("{},", column.column_name));
    }
    query.push_str("geom) FROM stdin BINARY");
    let writer: CopyInWriter = client.copy_in(&query)?;

    let mut writer = BinaryCopyInWriter::new(writer, &types);

    // Use to test if types are correct
    // println!("{:?}", types);

    for row in rows.row.iter() {
        // Transform row into vector of ToSql
        let mut tosql: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for column in row.columns.iter() {
            match column {
                AcceptedTypes::Int(value) => {
                    tosql.push(value);
                }
                AcceptedTypes::Float(value) => {
                    tosql.push(value);
                }
                AcceptedTypes::Double(value) => {
                    tosql.push(value);
                }
                AcceptedTypes::Text(value) => {
                    tosql.push(value);
                }
                AcceptedTypes::Bool(value) => {
                    tosql.push(value);
                }
                AcceptedTypes::Geometry(value) => {
                    tosql.push(value);
                }
            }
        }

        // Convert the vector to a slice of references
        let vec_slice: &[&(dyn ToSql + Sync)] = &tosql;

        // Write row to database
        writer
            .write(vec_slice)
            .expect("Failed to insert row into database");
    }

    // Finish writing
    writer.finish()?;

    Ok(())
}
