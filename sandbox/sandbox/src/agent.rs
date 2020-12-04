use geo::{Point, Rect};
use geo::euclidean_distance::EuclideanDistance;
use crate::utils;
use crate::state_transition::StateTransition;
use crate::ray::Ray;
use pyo3::prelude::*;

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
    pub targets_count: i32,
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
}

impl Agent {
    pub(crate) fn new(position: Point<f64>, direction: f64) -> Self {
        Agent {
            speed: 0.0045,
            age: 1.0,
            direction,
            ray_count: 49.0,
            fov: 0.8,
            visibility: 0.6,
            max_age: 400.0,
            position: position,
            rays: vec![],
            rays_bb:Rect::new((f64::NEG_INFINITY,f64::NEG_INFINITY),(f64::INFINITY,f64::INFINITY)),
            targets_count: 0,
            closest_target: Point::new(0.0,0.0),
            active: true,
            position_ticker: 50,
            past_positions: vec![position],
            past_position_distance: 0.0,
            past_position_bearing: 0.0,
            action_space: vec![
                -10.0f64.to_radians(),
                -1.0f64.to_radians(),
                0.0f64.to_radians(),
                1.0f64.to_radians(),
                10.0f64.to_radians(),
            ],
            prev_state: vec![],
            memory: vec![],
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
        self.prev_state = new_state.clone();
    }

    pub fn step(&mut self, action: usize, full_move: bool) {
        let mut step_size = self.speed;
        let direction_change = self.action_space.get(action as usize).unwrap(); 
        if !full_move {
            step_size = self.speed / 3.0;
        }
        if self.age > self.max_age {
            self.active = false;
        }
        self.direction += direction_change;
        if self.direction > 3.14 {
            self.direction = self.direction - 6.28;
        }
        if self.direction < -3.14 {
            self.direction = self.direction + 6.28;
        }
        self.position_ticker = self.position_ticker - 1;
        if self.position_ticker <= 0 {
            self.position_ticker = 50;
            self.past_positions.push(self.position);
        }
        if self.past_positions.len() > 5 {
            self.past_positions = self.past_positions.drain(self.past_positions.len()-5..).collect();
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
        self.cast_rays();
    }
}
