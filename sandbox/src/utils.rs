use crate::ray::Ray;
use geo::bounding_rect::BoundingRect;
use geo::euclidean_distance::EuclideanDistance;
use geo::intersects::Intersects;
use geo::map_coords::MapCoordsInplace;
use geo::{Geometry, GeometryCollection, LineString, Point, Polygon};
use geojson::{quick_collection, GeoJson};
use line_intersection::{LineInterval, LineRelation};
use std::fs;

fn load_json(path: String) -> GeometryCollection<f64> {
    let geojson_str = fs::read_to_string(path).unwrap();
    let geojson = geojson_str.parse::<GeoJson>().unwrap();
    quick_collection(&geojson).unwrap()
}

pub fn import_line_strings(path: String) -> (Vec<LineString<f64>>, f64, f64) {
    let collection = load_json(path);
    let mut lines = collection_as_line_strings(collection);
    let (scalex, scaley, xmin, ymin, _xmax, _ymax) = calculate_scales(&lines);
    scale_line_strings(scalex, scaley, xmin, ymin, &mut lines);
    (lines, scalex, scaley)
}

pub fn scale_line_strings(
    scalex: f64,
    scaley: f64,
    xmin: f64,
    ymin: f64,
    lines: &mut Vec<LineString<f64>>,
) {
    for line in lines.iter_mut() {
        line.map_coords_inplace(|&(x, y)| ((x - xmin) / scalex, (y - ymin) / scaley));
    }
}

pub fn collection_as_line_strings(mut collection: GeometryCollection<f64>) -> Vec<LineString<f64>> {
    let mut lines: Vec<LineString<_>> = vec![];
    for i in collection.iter_mut() {
        match i {
            Geometry::Point(_) => {}
            Geometry::Line(_) => {}
            Geometry::LineString(ref x) => lines.push(x.clone()),
            Geometry::Polygon(ref x) => lines.push(x.exterior().clone()),
            Geometry::MultiPoint(_) => {}
            Geometry::MultiLineString(_) => {}
            Geometry::MultiPolygon(_) => {}
            Geometry::GeometryCollection(_) => {}
            Geometry::Rect(_) => {}
            Geometry::Triangle(_) => {}
        }
    }
    lines
}

pub fn calculate_scales(lines: &Vec<LineString<f64>>) -> (f64, f64, f64, f64, f64, f64) {
    let mut xmin: f64 = 999.9;
    let mut ymin: f64 = 999.9;
    let mut xmax: f64 = 0.0;
    let mut ymax: f64 = 0.0;
    for line in lines.iter() {
        for point in line.points_iter() {
            xmin = xmin.min(point.x());
            ymin = ymin.min(point.y());
            xmax = xmax.max(point.x());
            ymax = ymax.max(point.y());
        }
    }
    let scalex = 1.0 * (xmax - xmin);
    let scaley = 1.0 * (ymax - ymin);
    (scalex, scaley, xmin, ymin, xmax, ymax)
}

pub fn cull_line_strings<'a>(
    rays: &Vec<Ray>,
    line_strings: &'a Vec<LineString<f64>>,
    origin_position: Point<f64>,
) -> Vec<&'a LineString<f64>> {
    let polygon = Polygon::new(
        LineString::from(vec![
            origin_position.x_y(), // origin
            rays[0].line_string.0[1].x_y(),
            rays[(rays.len() as f64 / 2.0).floor() as usize]
                .line_string
                .0[1]
                .x_y(),
            rays[rays.len() - 1].line_string.0[1].x_y(),
            origin_position.x_y(), // origin
        ]),
        vec![],
    );

    let mut intersecting_line_strings = vec![];
    for line_string in line_strings.iter() {
        if polygon.intersects(line_string) {
            intersecting_line_strings.push(line_string)
        }
    }
    intersecting_line_strings
}

pub fn cull_line_strings_precull<'a>(
    rays: &Vec<Ray>,
    line_strings: &'a Vec<LineString<f64>>,
    origin_position: Point<f64>,
) -> Vec<&'a LineString<f64>> {
    let polygon = Polygon::new(
        LineString::from(vec![
            origin_position.x_y(), // origin
            rays[0].line_string.0[1].x_y(),
            rays[(rays.len() as f64 / 2.0).floor() as usize]
                .line_string
                .0[1]
                .x_y(),
            rays[rays.len() - 1].line_string.0[1].x_y(),
            origin_position.x_y(), // origin
        ]),
        vec![],
    );

    let bbox = polygon.bounding_rect().unwrap();

    let max_x = bbox.max().x;
    let max_y = bbox.max().y;
    let min_x = bbox.min().x;
    let min_y = bbox.min().y;

    let mut intersecting_line_strings = vec![];
    for line_string in line_strings.iter() {
        let mut check = false;
        for line in line_string.lines() {
            if line.start.x > max_x && line.end.x > max_x {
                continue;
            }
            if line.start.y > max_y && line.end.y > max_y {
                continue;
            }
            if line.start.x < min_x && line.end.x < min_x {
                continue;
            }
            if line.start.y < min_y && line.end.y < min_y {
                continue;
            }
            check = true;
        }

        if check && polygon.intersects(line_string) {
            intersecting_line_strings.push(line_string)
        }
    }
    intersecting_line_strings
}

pub fn find_intersections_seq(
    rays: &mut Vec<Ray>,
    line_strings: &Vec<&LineString<f64>>,
    origin_position: Point<f64>,
) {
    rays.iter_mut()
        .for_each(|ray| find_intersections(ray, line_strings, origin_position));
}

pub fn find_intersections(
    ray: &mut Ray,
    line_strings: &Vec<&LineString<f64>>,
    origin_position: Point<f64>,
) {
    for line in line_strings.iter() {
        let intersections = intersections(&ray.line_string, line);
        for intersection in intersections.iter() {
            let length = intersection.euclidean_distance(&origin_position);
            if length < ray.length {
                ray.length = length;
                ray.line_string = LineString(vec![ray.line_string.0[0], intersection.0])
            }
        }
    }
}

fn intersections(linestring1: &LineString<f64>, linestring2: &LineString<f64>) -> Vec<Point<f64>> {
    let mut intersections = vec![];
    if linestring1.0.is_empty() || linestring2.0.is_empty() {
        return vec![];
    }
    for a in linestring1.lines() {
        for b in linestring2.lines() {
            let a_li = LineInterval::line_segment(a);
            let b_li = LineInterval::line_segment(b);
            match a_li.relate(&b_li) {
                LineRelation::DivergentIntersecting(x) => intersections.push(x),
                _ => {}
            }
        }
    }
    intersections
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::utils;
    use geo::Point;
    use test::Bencher;

    #[bench]
    fn test_culling_obstacles(b: &mut Bencher) {
        let position = Point::new(0.5, 0.5);
        let mut rays = Ray::generate_rays(180.0, 0.4, 0.3, 0.1, position);
        let (line_strings, _scalex, _scaley) =
            utils::import_line_strings("data/obstacles.json".into());
        b.iter(|| utils::cull_line_strings(&mut rays, &line_strings, position));
    }

    #[bench]
    fn test_culling_obstacles_preculling(b: &mut Bencher) {
        let position = Point::new(0.5, 0.5);
        let mut rays = Ray::generate_rays(180.0, 0.4, 0.3, 0.1, position);
        let (line_strings, _scalex, _scaley) =
            utils::import_line_strings("data/obstacles.json".into());
        b.iter(|| utils::cull_line_strings_precull(&mut rays, &line_strings, position));
    }

    #[bench]
    fn test_culling_polygons(b: &mut Bencher) {
        let position = Point::new(0.5, 0.5);
        let mut rays = Ray::generate_rays(180.0, 0.4, 0.3, 0.1, position);
        let (line_strings, _scalex, _scaley) =
            utils::import_line_strings("data/polygons.json".into());
        b.iter(|| utils::cull_line_strings(&mut rays, &line_strings, position));
    }
}
