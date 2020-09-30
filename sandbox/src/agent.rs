use geo::{LineString, Point, Coordinate};
use crate::utils;
use pyo3::prelude::*;
use crate::ray::Ray;

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
    fn get_rays(&self) -> PyResult<Vec<Vec<(f64, f64, f64, f64, f64, f64)>>> {
        let as_tuples = self.rays.iter().map(|ray| ray.line_string.lines().map(|line| (line.start.x, line.start.y, line.end.x, line.end.y, ray.length, ray.angle)).collect()).collect();
        Ok(as_tuples)
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