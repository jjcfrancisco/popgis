use crate::Result;
use postgres::Statement;
use postgres::types::Type;

use postgres::{Client, NoTls, CopyInWriter};

pub fn create_connection(uri: &str) -> Result<Client> {
    let client = Client::connect(uri, NoTls)?;
    Ok(client)
}

pub fn create_table(client: &mut Client) -> Result<Statement> {
    client.execute("CREATE TABLE IF NOT EXISTS pio (
                    id INT,
                    properties JSONB,
                    geometry geometry);", &[])?;

    let stmt = client.prepare("SELECT geometry FROM pio")?;
    Ok(stmt)
}

pub fn infer_geom_type(stmt: Statement) -> Result<Type> {
    let column = stmt.columns().get(0).expect("Failed to get columns");
    Ok(column.type_().clone())
}

pub fn create_binary_writer<'a>(client: &'a mut Client) -> Result<CopyInWriter<'a>> {
    let writer:CopyInWriter = client.copy_in("COPY pio (id, properties, geometry) FROM stdin BINARY")?;
    Ok(writer)
}

