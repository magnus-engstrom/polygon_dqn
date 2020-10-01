use geo::{Coordinate, LineString, Point};

use crate::ray::Ray;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub(crate) struct Agent {
    #[pyo3(get, set)]
    pub speed: f64,
    #[pyo3(get, set)]
    pub direction: f64,
    #[pyo3(get, set)]
    pub ray_count: f64,
    #[pyo3(get, set)]
    pub fov: f64,
    #[pyo3(get, set)]
    pub visibility: f64,
    pub position: Point<f64>,
    pub rays: Vec<Ray>,
}

#[pymethods]
impl Agent {
    #[new]
    fn new(position: (f64, f64), direction: f64) -> Self {
        Agent {
            speed: 0.0004,
            direction,
            ray_count: 128.0,
            fov: 0.4,
            visibility: 0.6,
            position: Point::from(position),
            rays: vec![],
        }
    }

    #[getter]
    fn get_rays(&self) -> PyResult<Vec<HashMap<&str, f64>>> {
        let mut res = vec![];
        for ray in self.rays.iter() {
            for line in ray.line_string.lines() {
                let hashmap: HashMap<&str, f64> =
                    [
                        ("start_x", line.start.x),
                        ("start_y", line.start.y),
                        ("end_x", line.end.x),
                        ("end_y", line.end.y),
                        ("length", ray.length),
                        ("angle", ray.angle),
                    ].iter().cloned().collect();
                res.push(hashmap);
            }
        }
        Ok(res)
    }

    pub fn cast_rays(&mut self) {
        self.rays.clear();
        self.rays = Ray::generate_rays(
            self.ray_count,
            self.fov,
            self.visibility,
            self.direction,
            self.position,
        );
    }

    pub fn step(&mut self, direction_change: f64, speed: f64) {
        self.direction += direction_change;
        self.speed = speed;
        self.position = Point::new(
            self.position.x() + self.speed * self.direction.cos(),
            self.position.y() + self.speed * self.direction.sin(),
        );
        self.cast_rays();
    }
}
