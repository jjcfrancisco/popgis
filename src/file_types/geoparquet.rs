use crate::{Error, Result};

use geoarrow::table::GeoTable;
use tokio::fs::File;

use geoarrow::io::parquet::read_geoparquet_async;
use geoarrow::io::parquet::GeoParquetReaderOptions;

async fn read_geoparquet(file: &str, batch_size: usize) -> Result<GeoTable> {
    let file = File::open(file).await.unwrap();
    let options = GeoParquetReaderOptions::new(batch_size, Default::default());
    let geotable = read_geoparquet_async(file, options).await?;

    Ok(geotable)
}

pub fn process_geotable() -> Result<()> {
    let runtime = tokio::runtime::Runtime::new()?;
    let geotable = runtime.block_on(read_geoparquet("../../data/saporo.parquet", 1000))?;
    let geom_column = geotable.geometry()?;
    let geom_type = geom_column.data_type();
    println!("{:?}", geom_type);
    let chunks = geom_column.geometry_chunks();

    // To polygons
    // for chunk in chunks {
    //     let polys = chunk.as_polygon();
    //     polys.iter().for_each(|poly| {
    //         if poly.is_some() {
    //             let poly = poly.unwrap();
    //             let geo_geom = poly.to_geo_geometry();
    //             println!("{:?}", geo_geom);
    //         }
    //     });
    // }

    Ok(())
}

// Write test for reading geoparquet
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_geoparquet() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let file_path = "examples/geoparquet/example.parquet";
        let batch_size = 1000;
        let result = runtime.block_on(read_geoparquet(file_path, batch_size)).unwrap();
        assert_eq!(result.len(), 5);
    }
}
