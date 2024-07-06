use crate::Result;
use postgres::Statement;
use postgres::types::Type;

use postgres::{Client, NoTls, CopyInWriter};

use crate::utils::NewTableTypes;

pub fn create_connection(uri: &str) -> Result<Client> {
    let client = Client::connect(uri, NoTls)?;
    Ok(client)
}

// pub fn create_table(client: &mut Client) -> Result<Statement> {
//     client.execute("CREATE TABLE IF NOT EXISTS pio (
//                     id INT,
//                     properties JSONB,
//                     geometry geometry);", &[])?;
//
//     let stmt = client.prepare("SELECT geometry FROM pio")?;
//     Ok(stmt)
// }

pub fn infer_geom_type(stmt: Statement) -> Result<Type> {
    let column = stmt.columns().get(0).expect("Failed to get columns");
    Ok(column.type_().clone())
}

pub fn create_binary_writer<'a>(client: &'a mut Client) -> Result<CopyInWriter<'a>> {
    let writer:CopyInWriter = client.copy_in("COPY pio (id, properties, geometry) FROM stdin BINARY")?;
    Ok(writer)
}

pub fn create_table(table_name: &str, schema_name: Option<String>, config: &Vec<NewTableTypes>, client: &mut Client) -> Result<()> {
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
                query.push_str(&format!(" {} INT,", column.column_name));
            },
            Type::FLOAT8 => {
                query.push_str(&format!(" {} DOUBLE,", column.column_name));
            },
            Type::TEXT => {
                query.push_str(&format!(" {} TEXT,", column.column_name));
            },
            Type::BOOL => {
                query.push_str(&format!(" {} BOOL,", column.column_name));
            },
            _ => println!("Type currently not supported"),
        }
    }
    query.push_str("geom GEOMETRY");
    query.push_str(");");
    println!("{}", query);
    client.execute(&query, &[])?;
    Ok(())
}
