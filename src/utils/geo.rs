use crate::Result;
use shapefile;

pub fn read_shapefile(file_path: &str) -> Result<()> {
    let mut reader = shapefile::Reader::from_path(file_path)?;
    for shape_record in reader.iter_shapes_and_records() {
        let (shape, record) = shape_record?;
    }
    Ok(())
}
