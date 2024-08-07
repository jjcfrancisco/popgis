use crate::Result;
use bytes::BytesMut;
use postgres::types::to_sql_checked;
use postgres::types::{IsNull, ToSql, Type};
use postgres::Statement;
use std::error::Error;

use postgres::binary_copy::BinaryCopyInWriter;
use postgres::CopyInWriter;

use crate::pg::ops::create_connection;
use crate::file_types::common::{AcceptedTypes, NameAndType};
use crate::utils::cli::Cli;

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

pub fn infer_geometry_type(table_name: &str, schema_name: &Option<String>, uri: &str) -> Result<Type> {
    let mut client = create_connection(uri)?;
    let stmt = if let Some(schema) = schema_name {
        client.prepare(&format!("SELECT geom FROM {}.{}", schema, table_name))?
    } else {
        client.prepare(&format!("SELECT geom FROM {}", table_name))?
    };
    let column = stmt.columns().first().expect("Failed to get columns ✘");
    Ok(column.type_().clone())
}

pub fn insert_row(
    row: Vec<AcceptedTypes>,
    config: &[NameAndType],
    types: &Vec<Type>,
    args: &Cli,
) -> Result<()> {
    // Create connection
    let mut client = create_connection(&args.uri)?;

    // Binary copy in writer
    let schema = &args.schema;
    let table = &args.table;
    let mut query = String::from("COPY ");
    if let Some(schema) = schema {
        query.push_str(&format!("{}.{}", schema, table));
    } else {
        query.push_str(table);
    }
    query.push_str(" (");
    for column in config.iter() {
        query.push_str(&format!("{},", column.name));
    }
    query.push_str("geom) FROM stdin BINARY");
    let writer: CopyInWriter = client.copy_in(&query)?;

    let mut writer = BinaryCopyInWriter::new(writer, &types);

    // Use to test if types are correct
    if std::env::var("DEBUG").is_ok() {
        println!("DEBUG || {:?}", types);
    }

    // Transform row into vector of ToSql
    let mut tosql: Vec<&(dyn ToSql + Sync)> = Vec::new();
    for column in row.iter() {
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
        .expect("Failed to insert row into database ✘");

    // Finish writing
    writer.finish()?;

    Ok(())
}
