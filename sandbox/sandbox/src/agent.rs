use geo::{Point, Rect, LineString};
use geo::euclidean_distance::EuclideanDistance;
use crate::utils;
use crate::ray::Ray;
use pyo3::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use geo::map_coords::MapCoordsInplace;
pub struct Agent {
    pub speed: f64,
    pub direction: f64,
    pub ray_count: f64,
    pub fov: f64,
    pub visibility: f64,
    pub position: Point<f64>,
    pub rays: Vec<Ray>,
    pub rays_bb: Rect<f64>,
    pub age: f64,
    pub closest_target: Point<f64>,
    pub max_age: f64,
    pub active: bool,
    pub position_ticker: i32,
    pub past_positions: Vec<Point<f64>>,
    pub past_position_distance: f64,
    pub past_position_bearing: f64,
    pub action_space: Vec<f64>,
    pub prev_state: Vec<f64>,
    pub memory: Vec<Py<PyAny>>,
    pub collected_targets: Vec<Point<f64>>,
    pub prev_target_dist: f64,
    pub last_state: Vec<f64>,
    pub coordinates_path: Vec<Point<f64>>,
    pub env_line_strings: Vec<LineString<f64>>,
    pub bearing_to_target: f64,
    pub targets_found: i32,
}

impl Agent {
    pub(crate) fn new(mut position: Point<f64>, env_line_strings: Vec<LineString<f64>>, max_age: i32) -> Self {
        println!("max steps: {}", max_age);
        let first_target = position.clone();
        position.map_coords_inplace(|&(x, y)| ((x + rand::thread_rng().gen_range(-0.005, 0.005)), (y + rand::thread_rng().gen_range(-0.005, 0.005))));
        Agent {
            speed: 0.007, // 0.0045
            age: 1.0,
            direction: rand::thread_rng().gen_range(-3.14, 3.14),
            ray_count: 29.0,
            fov: 130.0f64.to_radians(),
            visibility: 0.6,
            max_age: max_age as f64,
            position: position,
            rays: vec![],
            rays_bb:Rect::new((f64::NEG_INFINITY,f64::NEG_INFINITY),(f64::INFINITY,f64::INFINITY)),
            collected_targets: vec![first_target],
            targets_found: 0,
            closest_target: Point::new(0.0,0.0),
            active: true,
            position_ticker: 70, // 50
            past_positions: vec![position],
            past_position_distance: 0.0,
            past_position_bearing: 0.0,
            last_state: vec![],
            action_space: vec![
                -10.0f64.to_radians(),
                -3.0f64.to_radians(), // 1
                0.0f64.to_radians(),
                3.0f64.to_radians(), // 1
                10.0f64.to_radians(),
            ],
            prev_state: vec![],
            memory: vec![],
            prev_target_dist: 1.0,
            coordinates_path: vec![position],
            env_line_strings,
            bearing_to_target: 0.0,
        }
    }

    pub fn cast_rays(&mut self) {
        self.rays.clear();
        let (rays, rays_bb) = Ray::generate_rays(
            self.ray_count,
            self.fov,
            self.visibility,
            self.direction,
            self.position,
        );
        self.rays = rays;
        self.rays_bb = rays_bb;
    }


    pub fn get_rays(&self) -> Vec<HashMap<&str, f64>> {
        let mut res = vec![];
        for ray in self.rays.iter() {
            for line in ray.line_string.lines() {
                let hashmap: HashMap<&str, f64> = [
                    ("start_x", line.start.x),
                    ("start_y", line.start.y),
                    ("end_x", line.end.x),
                    ("end_y", line.end.y),
                    ("length", ray.length),
                    ("max_length", ray.max_length),
                    ("angle", ray.angle),
                    ("in_fov", ray.in_fov as i32 as f64),
                    ("angle_adj", ray.angle_adj),
                ]
                    .iter()
                    .cloned()
                    .collect();
                res.push(hashmap);
            }
        }
        res
    }

    pub fn add_to_memory(&mut self, new_state: &Vec<f64>, action: i32, reward: f64, done: bool) {
        if self.prev_state.len() > 0 {
            let gil = Python::acquire_gil();
            let py = gil.python();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("old_state", self.prev_state.clone().to_object(py)), 
                ("action", action.to_object(py)),
                ("new_state", new_state.clone().to_object(py)),
                ("reward", reward.to_object(py)),
                ("done", done.to_object(py))
            ];
            self.memory.push(key_vals.to_object(py));
        }
        //println!("state: {:#?}", new_state);
        self.prev_state = new_state.clone();
    }

    pub fn collect_target(&mut self, target: Point<f64>, n_targets: i32) {
        self.age = 1.0;
        self.max_age = self.max_age + 100.0;
        self.targets_found = self.targets_found + 1;
        self.collected_targets.push(target);
        if self.collected_targets.len() as i32 == n_targets {
            self.collected_targets = vec![];
        }
        self.past_positions = vec![self.position];
        self.position_ticker = 0;        
    }

    pub fn step(&mut self, action: usize) {
        let step_size = self.speed;
        let direction_change = self.action_space.get(action as usize).unwrap(); 
        self.position_ticker = self.position_ticker - 1;
        if self.position_ticker <= 0 {
            self.position_ticker = 50;
            self.past_positions.push(self.position);
        }
        if self.past_positions.len() > 3 {
            self.past_positions = self.past_positions.drain(self.past_positions.len()-3..).collect();
        }
        if self.age > self.max_age {
            self.active = false;
        }
        self.direction += direction_change;
        if self.direction > 3.14159 {
            self.direction = self.direction - 3.14159*2.0;
        }
        if self.direction < -3.14159 {
            self.direction = self.direction + 3.14159*2.0;
        }
        let closest_past_position = utils::closest_of(self.past_positions.iter(), self.position).unwrap();
        let new_position = Point::new(
            self.position.x() + step_size * self.direction.cos(),
            self.position.y() + step_size * self.direction.sin(),
        );
        self.past_position_distance = self.position.euclidean_distance(&closest_past_position);
        self.past_position_bearing = utils::relative_bearing_to_target(
            self.position, 
            new_position, 
            closest_past_position
        );
        self.position = new_position;
        self.coordinates_path.push(self.position);
        self.cast_rays();
        self.update();
    }

    pub fn update(&mut self) {
        let intersecting_line_strings =
            utils::cull_line_strings_precull(&mut self.rays_bb, &self.env_line_strings, self.position);
        utils::find_intersections_seq(&mut self.rays, &intersecting_line_strings, self.position)
    }

    pub fn get_coordinates_path(&self) -> Vec<Point<f64>> {
        return self.coordinates_path.clone()
    }
}
