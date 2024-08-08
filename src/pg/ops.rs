use crate::{file_types::common::NameAndType, Error, Result};
use postgres::types::Type;

use crate::utils::cli::Cli;
use postgres::{Client, NoTls};

pub fn prepare_postgis(args: &Cli, config: &[NameAndType]) -> Result<()> {
    // If schema present, create schema
    if let Some(schema) = &args.schema {
        let schema_exists = create_schema(schema, &args.uri)?;
        if !schema_exists {
            println!("Schema '{}' already exists ■", schema);
        } else {
            println!("\nSchema '{}' created ✓", schema);
        }
    }
    create_table(&args.table, &args.schema, &config, &args.uri, &args.srid)?;
    println!("Table '{}' created ✓", args.table);

    Ok(())
}

pub fn create_connection(uri: &str) -> Result<Client> {
    let client = Client::connect(uri, NoTls)?;
    Ok(client)
}

pub fn create_schema(schema_name: &str, uri: &str) -> Result<bool> {
    let mut client = create_connection(uri)?;
    let query = "SELECT EXISTS(SELECT 1 FROM information_schema.schemata WHERE schema_name = $1)";
    let row = client.query_one(query, &[&schema_name])?;
    let exists: bool = row.get(0);
    if exists {
        Ok(false)
    } else {
        client.batch_execute(&format!("CREATE SCHEMA {}", schema_name))?;
        Ok(true)
    }
}

pub fn create_table(
    table_name: &str,
    schema_name: &Option<String>,
    config: &[NameAndType],
    uri: &str,
    srid: &Option<usize>,
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
                query.push_str(&format!("{} INT,", column.name));
            }
            Type::FLOAT8 => {
                query.push_str(&format!("{} DOUBLE PRECISION,", column.name));
            }
            Type::TEXT => {
                query.push_str(&format!("{} TEXT,", column.name));
            }
            Type::BOOL => {
                query.push_str(&format!("{} BOOL,", column.name));
            }
            _ => println!("Type currently not supported ✘"),
        }
    }

    // If no srid, default to 4326
    if let Some(srid) = srid {
        query.push_str(&format!("geom Geometry(Geometry, {})", srid));
    } else {
        query.push_str("geom Geometry(Geometry, 4326)");
    }
    query.push_str(");");

    // Debugging
    if std::env::var("DEBUG").is_ok() {
        println!("DEBUG || {}", query);
    }

    let mut client = create_connection(uri)?;
    client.execute(&query, &[])?;

    Ok(())
}

pub fn check_table_exists(table_name: &str, schema_name: &Option<String>, uri: &str) -> Result<()> {
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
        Err(Error::TableExists("Table already exists ✘".into()))
    } else {
        Ok(())
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
