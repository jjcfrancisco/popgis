use crate::{Result, Error};
use postgres::types::Type;
use postgres::Statement;

use postgres::{Client, NoTls};

use crate::file_types::common::NewTableTypes;

pub fn create_connection(uri: &str) -> Result<Client> {
    let client = Client::connect(uri, NoTls)?;
    Ok(client)
}

pub fn create_schema(schema_name: &str, uri: &str) -> Result<()> {
    let mut client = create_connection(uri)?;
    client.batch_execute(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name))?;
    Ok(())
}

pub fn get_stmt(table_name: &str, schema_name: &Option<String>, uri: &str) -> Result<Statement> {
    let mut client = create_connection(uri)?;
    let stmt = if let Some(schema) = schema_name {
        client.prepare(&format!("SELECT geom FROM {}.{}", schema, table_name))?
    } else {
        client.prepare(&format!("SELECT geom FROM {}", table_name))?
    };
    Ok(stmt)
}

pub fn create_table(
    table_name: &str,
    schema_name: &Option<String>,
    config: &[NewTableTypes],
    uri: &str,
    srid: i32,
) -> Result<()> {
    let mut query = String::from("CREATE TABLE IF NOT EXISTS ");
    if let Some(schema) = schema_name {
        query.push_str(&format!("{}.{} ", schema, table_name));
    } else {
        query.push_str(table_name);
    }
    query.push('(');
    for column in config.iter() {
        match column.data_type {
            Type::INT8 => {
                query.push_str(&format!("{} INT,", column.column_name));
            }
            Type::FLOAT8 => {
                query.push_str(&format!("{} DOUBLE PRECISION,", column.column_name));
            }
            Type::TEXT => {
                query.push_str(&format!("{} TEXT,", column.column_name));
            }
            Type::BOOL => {
                query.push_str(&format!("{} BOOL,", column.column_name));
            }
            _ => println!("Type currently not supported ✘"),
        }
    }
    query.push_str(&format!("geom Geometry(Geometry, {})", srid));
    query.push_str(");");

    // Debugging
    if std::env::var("DEBUG").is_ok() {
        println!("DEBUG || {}", query);
    }

    // If schema, println with schema
    if let Some(schema) = schema_name {
        println!("\nSchema '{}' created ✓", schema);
        println!("Table '{}' created ✓", table_name);
    } else {
        println!("Table '{}' created ✓", table_name);
    }

    let mut client = create_connection(uri)?;
    client.execute(&query, &[])?;

    Ok(())

}

pub fn can_append(table_name: &str, schema_name: &Option<String>, uri: &str) -> Result<()> {
    let mut client = create_connection(uri)?;
    let query = if let Some(schema) = schema_name {
        format!(
            "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_schema = '{}' AND table_name = '{}')",
            schema, table_name
        )
    } else {
        format!(
            "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = '{}')",
            table_name
        )
    };
    let exists: bool = client.query_one(&query, &[])?.get(0);
    // If exists, return Ok
    if exists {
        return Ok(());
    } else {
        return Err(Error::CannotAppend("Cannot append to a table that does NOT exist ✘".into()));
    }
}

pub fn check_table_exists(
    table_name: &str,
    schema_name: &Option<String>,
    uri: &str,
) -> Result<()> {
    let mut client = create_connection(uri)?;
    let query = if let Some(schema) = schema_name {
        format!(
            "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_schema = '{}' AND table_name = '{}')",
            schema, table_name
        )
    } else {
        format!(
            "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = '{}')",
            table_name
        )
    };
    let exists: bool = client.query_one(&query, &[])?.get(0);
    // If exists, throw error
    if exists {
        return Err(Error::TableExists("Table already exists ✘".into()));
    } else {
        return Ok(());
    }
}

pub fn drop_table(table_name: &str, schema_name: &Option<String>, uri: &str) -> Result<()> {
    let mut client = create_connection(uri)?;
    let query = if let Some(schema) = schema_name {
        format!("DROP TABLE IF EXISTS {}.{}", schema, table_name)
    } else {
        format!("DROP TABLE IF EXISTS {}", table_name)
    };
    client.execute(&query, &[])?;
    Ok(())
}
