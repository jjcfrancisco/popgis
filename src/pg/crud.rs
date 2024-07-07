use crate::Result;
use postgres::types::Type;
use postgres::Statement;

use postgres::{Client, NoTls};

use crate::utils::NewTableTypes;

pub fn create_connection(uri: &str) -> Result<Client> {
    let client = Client::connect(uri, NoTls)?;
    Ok(client)
}

pub fn create_table(
    table_name: &str,
    schema_name: &Option<String>,
    config: &Vec<NewTableTypes>,
    uri: &str,
) -> Result<Statement> {
    let mut query = String::from("CREATE TABLE IF NOT EXISTS ");
    if let Some(schema) = schema_name {
        query.push_str(&format!("{}.{} ", schema, table_name));
    } else {
        query.push_str(&table_name);
    }
    query.push_str("(");
    for column in config.iter() {
        match column.data_type {
            Type::INT8 => {
                query.push_str(&format!("{} INT,", column.column_name));
            }
            Type::FLOAT8 => {
                query.push_str(&format!("{} DOUBLE,", column.column_name));
            }
            Type::TEXT => {
                query.push_str(&format!("{} TEXT,", column.column_name));
            }
            Type::BOOL => {
                query.push_str(&format!("{} BOOL,", column.column_name));
            }
            _ => println!("Type currently not supported"),
        }
    }
    query.push_str("geom Geometry(Geometry, 4326)");
    query.push_str(");");
    println!("{}", query);

    let mut client = create_connection(uri)?;
    client.execute(&query, &[])?;

    let stmt = if let Some(schema) = schema_name {
        client.prepare(&format!("SELECT geom FROM {}.{}", schema, table_name))?
    } else {
        client.prepare(&format!("SELECT geom FROM {}", table_name))?
    };

    Ok(stmt)
}
