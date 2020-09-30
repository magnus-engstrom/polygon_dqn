use geo::{Coordinate, LineString, Point, Line};

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
                }
            ),
        }
    }

    pub fn generate_rays(
        ray_count: f64,
        fov: f64,
        length: f64,
        direction: f64,
        position: Point<f64>,
    ) -> Vec<Ray> {
        let mut rays = vec![];
        for i in 0..ray_count as i32 {
            let x = i as f64 / ray_count - 0.5;
            let angle = x.atan2(fov);
            rays.push(Ray::new(angle, length, direction, position))
        }
        rays
    }
}
