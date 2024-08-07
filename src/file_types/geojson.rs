use crate::{Error, Result};

use crate::pg::binary_copy::Wkb;
use geojson::GeoJson;
use postgres::types::Type;
use serde_json;
use std::collections::HashMap;
use wkb::geom_to_wkb;

use crate::pg::binary_copy::{infer_geom_type, insert_row};
use crate::pg::crud::{get_stmt, prepare_postgis};
use crate::utils::cli::Cli;

use super::common::{AcceptedTypes, NewTableTypes};

pub fn insert_data(args: Cli) -> Result<()> {
    // Reads through the geojson file and determines the data types
    // Fix - it should only read one time
    //
    // Example:
    //
    // let data_types, geojson_str = read_geojson(&args.uri)?;
    let data_types = determine_data_types(&args.input)?;

    // Creates the necessary schema and table in PostGIS
    prepare_postgis(&args, &data_types)?;

    // Infer the geometry type
    let stmt = get_stmt(&args.table, &args.schema, &args.uri)?;
    let geom_type = infer_geom_type(stmt)?;

    // Prepare types for binary copy
    // This is unnecessary -> refactor soon
    let mut types: Vec<Type> = Vec::new();
    for column in data_types.iter() {
        types.push(column.data_type.clone());
    }
    types.push(geom_type);

    let geojson = read_geojson(&args.input)?;
    match geojson {
        GeoJson::FeatureCollection(fc) => {
            let features = fc.features;
            println!("Inserting data into database...");
            for feature in features {
                let mut row: Vec<AcceptedTypes> = Vec::new();
                for (_, value) in feature.properties.unwrap().into_iter() {
                    match value {
                        serde_json::Value::Number(num) => {
                            row.push(AcceptedTypes::Float(num.as_f64()));
                        }
                        serde_json::Value::String(string) => {
                            row.push(AcceptedTypes::Text(Some(string)));
                        }
                        serde_json::Value::Bool(boolean) => {
                            row.push(AcceptedTypes::Bool(Some(boolean)));
                        }
                        serde_json::Value::Null => {
                            row.push(AcceptedTypes::Text(None));
                        }
                        _ => println!("Type currently not supported ✘"),
                    }
                }
                let gj_geom = feature.geometry.unwrap();
                let geom: geo::Geometry<f64> = gj_geom
                    .value
                    .try_into()
                    .expect("Failed to convert geojson::Geometry to geo::Geometry ✘");
                let wkb = geom_to_wkb(&geom).expect("Could not convert geometry to WKB ✘");
                row.push(AcceptedTypes::Geometry(Some(Wkb { geometry: wkb })));
                insert_row(row, &data_types, &types, &args)?;
            }
            println!("Data sucessfully inserted into database ✓");
        }
        _ => println!("Not a feature collection ✘"),
    }

    Ok(())
}

pub fn determine_data_types(file_path: &str) -> Result<Vec<NewTableTypes>> {
    let mut table_config: HashMap<String, Type> = HashMap::new();
    let geojson_str = std::fs::read_to_string(file_path)?;
    let geojson = geojson_str.parse::<GeoJson>().unwrap();

    match geojson {
        GeoJson::FeatureCollection(fc) => {
            let features = fc.features;
            if !features.is_empty() {
                let properties = features.first();
                if properties.is_some() {
                    for (key, value) in properties.unwrap().properties.clone().unwrap().into_iter()
                    {
                        if key == "geom" || key == "geometry" {
                            continue;
                        }
                        match value {
                            serde_json::Value::Number(_) => {
                                if table_config.contains_key(&key)
                                    && table_config[&key] == Type::FLOAT8
                                {
                                    continue;
                                } else if table_config.contains_key(&key)
                                    && table_config[&key] != Type::INT8
                                {
                                    return Err(Error::MixedDataTypes(
                                        "Column contains mixed data types ✘".to_string(),
                                    ));
                                } else {
                                    table_config.insert(key, Type::FLOAT8);
                                }
                            }
                            serde_json::Value::String(_) => {
                                if table_config.contains_key(&key)
                                    && table_config[&key] == Type::TEXT
                                {
                                    continue;
                                } else if table_config.contains_key(&key)
                                    && table_config[&key] != Type::INT8
                                {
                                    return Err(Error::MixedDataTypes(
                                        "Column contains mixed data types ✘".to_string(),
                                    ));
                                } else {
                                    table_config.insert(key, Type::TEXT);
                                }
                            }
                            serde_json::Value::Bool(_) => {
                                if table_config.contains_key(&key)
                                    && table_config[&key] == Type::BOOL
                                {
                                    continue;
                                } else if table_config.contains_key(&key)
                                    && table_config[&key] != Type::INT8
                                {
                                    return Err(Error::MixedDataTypes(
                                        "Column contains mixed data types ✘".to_string(),
                                    ));
                                } else {
                                    table_config.insert(key, Type::BOOL);
                                }
                            }
                            // If null
                            serde_json::Value::Null => continue,
                            _ => println!("Type currently not supported ✘"),
                        }
                    }
                }
            }
        }
        _ => println!("Not a feature collection ✘"),
    }

    let mut data_types: Vec<NewTableTypes> = Vec::new();
    for (column_name, data_type) in table_config {
        data_types.push(NewTableTypes {
            column_name,
            data_type,
        });
    }

    Ok(data_types)
}

pub fn read_geojson(file_path: &str) -> Result<GeoJson> {
    let geojson_str = std::fs::read_to_string(file_path)?;
    let geojson = geojson_str.parse::<GeoJson>().unwrap();
    Ok(geojson)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_data_types() {
        let file_path = "examples/geojson/spain.geojson";
        let data_types = determine_data_types(file_path).unwrap();
        assert_eq!(data_types.len(), 3);
        for data_type in data_types {
            match data_type.column_name.as_str() {
                "source" => assert_eq!(data_type.data_type, Type::TEXT),
                "id" => assert_eq!(data_type.data_type, Type::TEXT),
                "name" => assert_eq!(data_type.data_type, Type::TEXT),
                _ => (),
            }
        }
    }

    #[test]
    fn test_read_geojson() {
        let file_path = "examples/geojson/spain.geojson";
        let rows = read_geojson(file_path).unwrap();
        assert_eq!(rows.row.len(), 19);
    }
}
