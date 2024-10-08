use crate::utils::cli::Cli;
use crate::{Error, Result};

use geojson::GeoJson;
use postgres::types::Type;
use proj::{Proj, Transform};
use serde_json;
use std::collections::HashMap;
use wkb::geom_to_wkb;

use crate::format::common::{AcceptedTypes, Row, Rows};
use crate::pg::binary_copy::Wkb;

use super::common::NewTableTypes;

pub fn determine_data_types(file_path: &str) -> Result<Vec<NewTableTypes>> {
    let mut table_config: HashMap<String, Type> = HashMap::new();
    let geojson_str = std::fs::read_to_string(file_path)?;
    let geojson = geojson_str.parse::<GeoJson>().unwrap();

    match geojson {
        GeoJson::FeatureCollection(fc) => {
            let features = fc.features;
            // Id not used
            // table_config.push(NewTableTypes {
            //     column_name: "id".to_string(),
            //     data_type: Type::INT8,
            // });
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
                                        "❌ Column contains mixed data types".to_string(),
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
                                        "❌ Column contains mixed data types".to_string(),
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
                                        "❌ Column contains mixed data types".to_string(),
                                    ));
                                } else {
                                    table_config.insert(key, Type::BOOL);
                                }
                            }
                            // If null
                            serde_json::Value::Null => continue,
                            _ => println!("❌ Type currently not supported"),
                        }
                    }
                }
            }
        }
        _ => println!("❌ Not a feature collection"),
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

pub fn read_geojson(args: &Cli) -> Result<Rows> {
    let mut rows = Rows::new();
    let geojson_str = std::fs::read_to_string(&args.input)?;
    let geojson = geojson_str.parse::<GeoJson>().unwrap();

    match geojson {
        GeoJson::FeatureCollection(fc) => {
            let features = fc.features;
            for feature in features {
                let mut row = Row::new();
                // Id not used
                // let id = feature.id.unwrap();
                // match id {
                //     geojson::feature::Id::Number(id) => {
                //         let as_i64 = id.as_i64().unwrap();
                //         row.add(AcceptedTypes::Int(Some(as_i64 as i32)));
                //     }
                //     _ => (),
                // }
                for (_, value) in feature.properties.unwrap().into_iter() {
                    match value {
                        serde_json::Value::Number(num) => {
                            row.add(AcceptedTypes::Float(num.as_f64()));
                        }
                        serde_json::Value::String(string) => {
                            row.add(AcceptedTypes::Text(Some(string)));
                        }
                        serde_json::Value::Bool(boolean) => {
                            row.add(AcceptedTypes::Bool(Some(boolean)));
                        }
                        serde_json::Value::Null => {
                            row.add(AcceptedTypes::Text(None));
                        }
                        _ => println!("❌ Type currently not supported"),
                    }
                }
                let gj_geom = feature.geometry.unwrap();
                let mut geom: geo::Geometry<f64> = gj_geom
                    .value
                    .try_into()
                    .expect("❌ Failed to convert geojson::Geometry to geo::Geometry");
                // Reproject
                geom = if args.reproject.is_some() {
                    let from = format!("EPSG:{}", args.srid.unwrap());
                    let to = format!("EPSG:{}", args.reproject.unwrap());
                    let proj = Proj::new_known_crs(&from, &to, None)?;
                    geom.transform(&proj)?;
                    geom
                } else {
                    geom
                };
                let wkb = geom_to_wkb(&geom).expect("❌ Could not convert geometry to WKB");
                // Check length of row
                row.add(AcceptedTypes::Geometry(Some(Wkb { geometry: wkb })));
                rows.add(row);
            }
        }
        _ => println!("❌ Not a feature collection"),
    }

    Ok(rows)
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
        let args = Cli {
            input: file_path.to_string(),
            srid: Some(4326),
            reproject: Some(3857),
            mode: None,
            uri: "postgresql://localhost:5432/postgis".to_string(),
            schema: None,
            table: "spain".to_string(),
        };
        let rows = read_geojson(&args).unwrap();
        assert_eq!(rows.row.len(), 19);
    }
}
