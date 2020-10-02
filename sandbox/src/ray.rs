use geo::{Coordinate, Line, LineString, Point, Rect};
use crate::utils;

pub struct Ray {
    pub angle: f64,
    pub length: f64,
    pub max_length: f64,
    pub line_string: LineString<f64>,
    pub line: Line<f64>,
}

impl Ray {
    pub fn new(angle: f64, length: f64, center_angle: f64, position: Point<f64>) -> Ray {
        Ray {
            angle,
            length,
            max_length: 0.0,
            line_string: LineString(vec![
                Coordinate {
                    x: position.x(),
                    y: position.y(),
                },
                Coordinate {
                    x: position.x() + length * (center_angle + angle).cos(),
                    y: position.y() + length * (center_angle + angle).sin(),
                },
            ]),
            line: Line::new(
                Coordinate {
                    x: position.x(),
                    y: position.y(),
                },
                Coordinate {
                    x: position.x() + length * (center_angle + angle).cos(),
                    y: position.y() + length * (center_angle + angle).sin(),
                },
            ),
        }
    }

    pub fn generate_rays(
        ray_count: f64,
        fov: f64,
        length: f64,
        direction: f64,
        position: Point<f64>,
    ) -> (Vec<Ray>, Rect<f64>) {
        let mut min_x = position.x();
        let mut min_y = position.y();
        let mut max_x = position.x();
        let mut max_y = position.y();

        let mut rays = vec![];
        for i in 0..ray_count as i32 {
            let x = i as f64 / ray_count - 0.5;
            let angle = x.atan2(fov);
            let ray = Ray::new(angle, length, direction, position);
            let (tmp_min_x, tmp_min_y, tmp_max_x, tmp_max_y) = utils::min_max_point(&ray.line.end, min_x, min_y, max_x, max_y);
            min_x = tmp_min_x;
            min_y = tmp_min_y;
            max_x = tmp_max_x;
            max_y = tmp_max_y;
            rays.push(ray)
        }
        (rays, Rect::new((min_x, min_y),(max_x, max_y)))
    }
}
