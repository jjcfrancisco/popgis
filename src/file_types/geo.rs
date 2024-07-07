use crate::Result;
use geo::Coord;
use shapefile::Shape;

pub fn to_geo(shape: &Shape) -> Result<geo::Geometry<f64>> {
    match shape {
        Shape::Point(p) => Ok(geo::Point::new(p.x, p.y).into()),
        Shape::Polyline(p) => {
            let mut coords: Vec<Coord> = Vec::new();
            for part in p.parts().iter() {
                for point in part.iter() {
                    coords.push(Coord::from((point.x, point.y)));
                }
            }
            Ok(geo::LineString::new(coords).into())
        }
        Shape::Polygon(p) => {
            let mut outer_placeholder: Vec<(f64, f64)> = Vec::new();
            let mut inner_rings: Vec<geo::LineString> = Vec::new();

            for ring_type in p.rings() {
                match ring_type {
                    //Gather all outer rings
                    shapefile::PolygonRing::Outer(out) => {
                        out.iter().for_each(|p| outer_placeholder.push((p.x, p.y)))
                    }
                    //Gather all inner rings
                    shapefile::PolygonRing::Inner(inn) => {
                        let mut inner_ring: Vec<(f64, f64)> = Vec::new();
                        inn.iter().for_each(|p| inner_ring.push((p.x, p.y)));
                        let ls = geo::LineString::from(inner_ring);
                        inner_rings.push(ls);
                    }
                }
            }

            let outer_ring = geo::LineString::from(outer_placeholder);
            if inner_rings.is_empty() {
                let poly = geo::Polygon::new(outer_ring, vec![]);
                Ok(geo::Geometry::from(poly))
            } else {
                let poly = geo::Polygon::new(outer_ring, inner_rings);
                Ok(geo::Geometry::from(poly))
            }
        }
        _ => Err("Unsupported shape type".into()),
    }
}
