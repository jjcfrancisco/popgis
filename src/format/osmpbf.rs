use crate::utils::cli::Cli;
use crate::Result;

use geo::{MapCoords, Point};
use osmpbf::{Element, ElementReader};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct OsmPbf {
    tags: Vec<(String, String)>,
    geometry: geo::Geometry<f64>,
}

fn build_nodes(file_path: &str) -> Result<HashMap<i64, OsmPbf>> {
    let reader = ElementReader::from_path(&file_path)?;
    let mut nodes = HashMap::<i64, OsmPbf>::new();
    _ = reader.for_each(|element| match element {
        Element::Node(node) => {
            nodes.insert(
                node.id(),
                OsmPbf {
                    tags: node
                        .tags()
                        .into_iter()
                        .map(|(key, value)| (key.to_string(), value.to_string()))
                        .collect(),
                    geometry: geo::Geometry::Point(Point::new(node.lon(), node.lat())),
                },
            );
        }
        Element::DenseNode(dense_node) => {
            nodes.insert(
                dense_node.id(),
                OsmPbf {
                    tags: dense_node
                        .tags()
                        .into_iter()
                        .map(|(key, value)| (key.to_string(), value.to_string()))
                        .collect(),
                    geometry: geo::Geometry::Point(Point::new(dense_node.lon(), dense_node.lat())),
                },
            );
        }
        _ => {}
    });

    Ok(nodes)
}

fn build_polygon(way: &osmpbf::Way, nodes: &HashMap<i64, OsmPbf>) -> OsmPbf {
    let mut points: Vec<(f64, f64)> = Vec::new();
    way.refs().for_each(|node_id| {
        if let Some(node) = nodes.get(&node_id) {
            match node.geometry {
                geo::Geometry::Point(point) => {
                    points.push((point.x(), point.y()));
                }
                _ => {}
            }
        }
    });

    OsmPbf {
        tags: way
            .tags()
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect(),
        geometry: geo::Geometry::Polygon(geo::Polygon::new(geo::LineString::from(points), vec![])),
    }
}

fn build_line(way: &osmpbf::Way, nodes: &HashMap<i64, OsmPbf>) -> OsmPbf {
    let mut points: Vec<(f64, f64)> = Vec::new();
    way.refs().for_each(|node_id| {
        if let Some(node) = nodes.get(&node_id) {
            match node.geometry {
                geo::Geometry::Point(point) => {
                    points.push((point.x(), point.y()));
                }
                _ => {}
            }
        }
    });
    OsmPbf {
        tags: way
            .tags()
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect(),
        geometry: geo::Geometry::LineString(geo::LineString::from(points)),
    }
}

fn build_polys_and_lines(file_path: &str, nodes: &HashMap<i64, OsmPbf>) -> Result<Vec<OsmPbf>> {
    let reader = ElementReader::from_path(&file_path)?;
    let mut all = Vec::<OsmPbf>::new();
    _ = reader.for_each(|element| match element {
        Element::Way(way) => {
            // If the way is closed, it's a polygon
            if way.refs().next() == way.refs().last() {
                all.push(build_polygon(&way, &nodes));
            } else {
                all.push(build_line(&way, &nodes));
            }
        }
        Element::Relation(relation) => {}
        _ => {}
    });

    Ok(all)
}

pub fn read_osmpbf(args: &Cli) -> Result<()> {

    let nodes = build_nodes(&args.input)?;
    let all = build_polys_and_lines(&args.input, &nodes)?;

    Ok(())
}
