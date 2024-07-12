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

#[cfg(test)]
mod tests {
    use super::*;
    use shapefile::{Point, Polyline};

    #[test]
    fn test_to_geo_point() {
        let shape = Shape::Point(shapefile::Point::new(1.0, 2.0));
        let geo = to_geo(&shape).unwrap();
        assert_eq!(geo, geo::Geometry::Point(geo::Point::new(1.0, 2.0)));
    }

    #[test]
    fn test_to_geo_line() {
        let first_part = vec![
            Point::new(1.0, 1.0),
            Point::new(2.0, 2.0),
        ];

        let second_part = vec![
            Point::new(3.0, 1.0),
            Point::new(5.0, 6.0),
        ];

        let poly = Polyline::with_parts(vec![first_part, second_part]);
        let shape = Shape::Polyline(poly);
        let geo = to_geo(&shape).unwrap();
        let expected = geo::Geometry::LineString(geo::LineString::from(vec![
            Coord::from((1.0, 1.0)),
            Coord::from((2.0, 2.0)),
            Coord::from((3.0, 1.0)),
            Coord::from((5.0, 6.0)),
        ]));
        assert_eq!(geo, expected);
    }

    #[test]
    fn test_to_geo_poly() {

        let first_part = vec![
            Point::new(1.0, 1.0),
            Point::new(2.0, 2.0),
        ];

        let second_part = vec![
            Point::new(3.0, 1.0),
            Point::new(5.0, 6.0),
        ];

        let third_part = vec![
            Point::new(17.0, 15.0),
            Point::new(18.0, 19.0),
            Point::new(20.0, 19.0),
        ];
        let poly = Polyline::with_parts(vec![first_part, second_part, third_part]);
        let shape = Shape::Polyline(poly);
        let geo = to_geo(&shape).unwrap();
        let expected = geo::Geometry::LineString(geo::LineString::from(vec![
            Coord::from((1.0, 1.0)),
            Coord::from((2.0, 2.0)),
            Coord::from((3.0, 1.0)),
            Coord::from((5.0, 6.0)),
            Coord::from((17.0, 15.0)),
            Coord::from((18.0, 19.0)),
            Coord::from((20.0, 19.0)),
        ]));
        assert_eq!(geo, expected);

    }
}
