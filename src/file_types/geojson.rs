use crate::{Error, Result};

use crate::pg::binary_copy::Wkb;
use geojson::GeoJson;
use postgres::types::Type;
use serde_json;
use std::collections::HashMap;
use wkb::geom_to_wkb;

use crate::pg::binary_copy::{infer_geometry_type, insert_row};
use crate::pg::ops::prepare_postgis;
use crate::utils::cli::Cli;

use super::common::{AcceptedTypes, NameAndType};

pub fn insert_data(args: Cli) -> Result<()> {
    // Determine data types of the input file
    let file_data_types = determine_file_data_types(&args.input)?;
    // Prepare database
    prepare_postgis(&args, &file_data_types)?;

    // Get data types
    let mut types: Vec<Type> = Vec::new();
    for column in file_data_types.iter() {
        types.push(column.data_type.clone());
    }
    // Get geometry type
    let geom_type = infer_geometry_type(&args.table, &args.schema, &args.uri)?;
    // Add geometry type to types
    types.push(geom_type);

    // Read geojson file
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
                insert_row(row, &file_data_types, &types, &args)?;
            }
            println!("Data sucessfully inserted into database ✓");
        }
        _ => println!("Not a feature collection ✘"),
    }

    Ok(())
}

pub fn determine_file_data_types(file_path: &str) -> Result<Vec<NameAndType>> {
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

    let mut names_and_types: Vec<NameAndType> = Vec::new();
    for (name, data_type) in table_config.iter() {
        names_and_types.push(NameAndType {
            name: name.to_string(),
            data_type: data_type.clone(),
        });
    }

    Ok(names_and_types)
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
    fn test_determine_file_data_types() {
        let file_path = "examples/geojson/spain.geojson";
        let data_types = determine_file_data_types(file_path).unwrap();
        assert_eq!(data_types.len(), 3);
        for data_type in data_types {
            match data_type.name.as_str() {
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
        match rows {
            GeoJson::FeatureCollection(fc) => {
                let features = fc.features;
                assert_eq!(features.len(), 19);
            }
            _ => (),
        }
    }
}
