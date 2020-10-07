use crate::agent::Agent;
use crate::utils;
use geo::{LineString, Point};
use pyo3::prelude::*;
use std::collections::HashMap;
use crate::ray::Ray;
use geo::euclidean_distance::EuclideanDistance;

#[pyclass]
pub(crate) struct Env {
    pub line_strings: Vec<LineString<f64>>,
    pub agent: Agent,
    pub targets: Vec<Point<f64>>,
    #[pyo3(get, set)]
    scalex: f64,
    #[pyo3(get, set)]
    scaley: f64,
}

#[pymethods]
impl Env {
    #[new]
    fn new(path: String) -> Self {
        let (line_strings, targets, scalex, scaley) = utils::import_geometry(path);
        let agent = Agent::new((0.5, 0.5), 0.1);
        Env {
            line_strings,
            agent,
            targets,
            scalex,
            scaley,
        }
    }


    #[getter(lines)]
    fn get_line_strings_as_lines(&self) -> PyResult<Vec<HashMap<&str, f64>>> {
        let mut res = vec![];
        for line_string in self.line_strings.iter() {
            for line in line_string.lines() {
                let hashmap: HashMap<&str, f64> = [
                    ("start_x", line.start.x),
                    ("start_y", line.start.y),
                    ("end_x", line.end.x),
                    ("end_y", line.end.y),
                ]
                .iter()
                .cloned()
                .collect();
                res.push(hashmap);
            }
        }
        Ok(res)
    }

    #[getter(targets)]
    fn get_targets_as_points(&self) -> PyResult<Vec<HashMap<&str, f64>>> {
        let mut res = vec![];
        for target in self.targets.iter() {
            let hashmap: HashMap<&str, f64> = [
                ("x", target.x()),
                ("y", target.y()),
            ]
            .iter()
            .cloned()
            .collect();
            res.push(hashmap);
        }
        Ok(res)
    }

    pub fn step(&mut self, action: i32) -> (Vec<f64>, f64, bool) {
        let direction_change = match action {
            0 => -90.0f64.to_radians(),
            1 => -45.0f64.to_radians(),
            2 => 0.0f64.to_radians(),
            3 => 45.0f64.to_radians(),
            4 => 90.0f64.to_radians(),
            _ => panic!("Action should be between 0-4"),
        };
        let step_ray = Ray::new(direction_change, self.agent.speed, self.agent.direction, self.agent.position);
        if utils::intersects(&step_ray, &self.line_strings.iter().collect()) {
            let mut distance_to_closest_target = f64::INFINITY;
            let mut x = 0.0;
            let mut y = 0.0;
            for target in self.targets.iter() {
                let distance = self.agent.position.euclidean_distance(target);
                if distance < distance_to_closest_target {
                    distance_to_closest_target = distance;
                    x = target.x();
                    y = target.y();
                }
            }
            let bearing = (y - self.agent.position.y()).atan2(x - self.agent.position.x());

            let state = self.agent.rays.iter().map(|r| r.length / r.max_length).collect();
            let reward = -2.0;
            return (state, reward, true);
        }
        self.agent.step(direction_change);
        self.update_agent();

        let state = self.agent.rays.iter().map(|r| r.length / r.max_length).collect();
        let reward = -2.0;
        return (state, reward, true);
    }

    pub fn reset(&mut self) {
        let mut agent = Agent::new((0.5, 0.5), 0.1);
        agent.cast_rays();
        self.agent = agent;
        self.update_agent();
    }

    pub fn update_agent(&mut self) {
        let intersecting_line_strings =
            utils::cull_line_strings_precull(&mut self.agent.rays_bb, &self.line_strings, self.agent.position);
        utils::find_intersections_seq(&mut self.agent.rays, &intersecting_line_strings, self.agent.position)
    }
}
