use crate::format::common::{AcceptedTypes, NewTableTypes, Row, Rows};
use crate::pg::binary_copy::Wkb;
use crate::utils::cli::Cli;
use crate::Result;
use postgres::types::Type;
use wkb::geom_to_wkb;

use geo::{Geometry, LineString, Point, Polygon};
use osmpbf::{Element, ElementReader};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct OsmPbf {
    tags: Vec<String>,
    geometry: geo::Geometry<f64>,
}

fn build_nodes(file_path: &str) -> Result<HashMap<i64, OsmPbf>> {
    let reader = ElementReader::from_path(file_path)?;
    let mut nodes = HashMap::<i64, OsmPbf>::new();
    _ = reader.for_each(|element| match element {
        Element::Node(node) => {
            nodes.insert(
                node.id(),
                OsmPbf {
                    tags: node
                        .tags()
                        .map(|(key, value)| format!("{}={}", key, value))
                        .collect(),
                    geometry: Geometry::Point(Point::new(node.lon(), node.lat())),
                },
            );
        }
        Element::DenseNode(dense_node) => {
            nodes.insert(
                dense_node.id(),
                OsmPbf {
                    tags: dense_node
                        .tags()
                        .map(|(key, value)| format!("{}={}", key, value))
                        .collect(),
                    geometry: Geometry::Point(Point::new(dense_node.lon(), dense_node.lat())),
                },
            );
        }
        _ => {}
    });

    Ok(nodes)
}

fn build_line(way: &osmpbf::Way, nodes: &HashMap<i64, OsmPbf>) -> OsmPbf {
    let mut points: Vec<(f64, f64)> = Vec::new();
    way.refs().for_each(|node_id| {
        if let Some(node) = nodes.get(&node_id) {
            if let Geometry::Point(point) = node.geometry {
                points.push((point.x(), point.y()));
            }
        }
    });
    OsmPbf {
        tags: way
            .tags()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect(),
        geometry: Geometry::LineString(LineString::from(points)),
    }
}

fn build_polygon(way: &osmpbf::Way, nodes: &HashMap<i64, OsmPbf>) -> OsmPbf {
    let mut points: Vec<(f64, f64)> = Vec::new();
    way.refs().for_each(|node_id| {
        if let Some(node) = nodes.get(&node_id) {
            if let Geometry::Point(point) = node.geometry {
                points.push((point.x(), point.y()));
            }
        }
    });

    OsmPbf {
        tags: way
            .tags()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect(),
        geometry: Geometry::Polygon(Polygon::new(LineString::from(points), vec![])),
    }
}

fn build_polys_and_lines(file_path: &str, nodes: &HashMap<i64, OsmPbf>) -> Result<Vec<OsmPbf>> {
    let reader = ElementReader::from_path(file_path)?;
    let mut all = Vec::<OsmPbf>::new();
    _ = reader.for_each(|element| match element {
        Element::Way(way) => {
            // If the way is closed, it's a polygon
            if way.refs().next() == way.refs().last() {
                all.push(build_polygon(&way, nodes));
            } else {
                all.push(build_line(&way, nodes));
            }
        }
        #[allow(unused_variables)]
        Element::Relation(relation) => {
            // relation.members().into_iter().for_each(|member| {
            //     if let Some(node) = nodes.get(&member.member_id) {
            //         println!("{:?}", node);
            //     }
            // });
        }
        _ => {}
    });

    Ok(all)
}

fn osmpbf_to_rows(all: &[OsmPbf]) -> Result<Rows> {
    let mut rows = Rows::new();

    for item in all.iter() {
        let mut row = Row::new();
        // Tags
        row.add(AcceptedTypes::Array(Some(item.tags.clone())));
        // Geom
        let wkb =
            geom_to_wkb(&item.geometry.clone()).expect("❌ Could not convert geometry to WKB");
        row.add(AcceptedTypes::Geometry(Some(Wkb { geometry: wkb })));
        rows.add(row);
    }

    Ok(rows)
}

pub fn determine_data_types() -> Result<Vec<NewTableTypes>> {
    let data_types: Vec<NewTableTypes> = vec![NewTableTypes {
        column_name: "tags".to_string(),
        data_type: Type::TEXT_ARRAY,
    }];

    Ok(data_types)
}

pub fn read_osmpbf(args: &Cli) -> Result<Rows> {
    let nodes = build_nodes(&args.input)?;
    let all = build_polys_and_lines(&args.input, &nodes)?;
    let rows = osmpbf_to_rows(&all)?;

    Ok(rows)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_build_nodes() {
        let nodes = build_nodes("examples/osmpbf/monaco-latest.osm.pbf").unwrap();
        assert_eq!(nodes.len(), 30798);
    }

    #[test]
    fn test_build_polys_and_lines() {
        let nodes = build_nodes("examples/osmpbf/monaco-latest.osm.pbf").unwrap();
        let all = build_polys_and_lines("examples/osmpbf/monaco-latest.osm.pbf", &nodes).unwrap();
        assert_eq!(all.len(), 4944);
    }
}
