use crate::agent::Agent;
use crate::utils;
use geo::{LineString, Point, Closest};
use std::collections::HashMap;
use crate::ray::Ray;
use geo::euclidean_distance::EuclideanDistance;
use geo::bearing::Bearing;
use geo::closest_point::ClosestPoint;
use rand::Rng;
use rand::seq::SliceRandom;

pub struct Env {
    pub line_strings: Vec<LineString<f64>>,
    pub agent: Agent,
    pub targets: Vec<Point<f64>>,
    pub original_targets: Vec<Point<f64>>,
    pub last_state: Vec<f64>,
    pub action_space: Vec<f64>,
    pub starting_points: Vec<Point<f64>>,
    pub prev_target_dist: f64,
    scalex: f64,
    scaley: f64,
}

impl Env {
    pub fn new(path: String) -> Self {
        let (line_strings, targets, scalex, scaley) = utils::import_geometry(path);
        let mut rng = rand::thread_rng();
        let starting_points = vec![
            //Point::new(0.12, 0.58),
            Point::new(0.5, 0.5),
            //Point::new(0.9, 0.1),
        ];
        let mut agent = Agent::new(
            starting_points.choose(&mut rand::thread_rng()).unwrap().clone(), 
            rng.gen_range(-3.14, 3.14)
        );
        let original_targets = targets.to_vec();
        let action_space = vec![
            0.0f64.to_radians(),
            -5.0f64.to_radians(),
            5.0f64.to_radians(),
            //30.0f64.to_radians(),
            //-30.0f64.to_radians(),
        ];
        Env {
            line_strings,
            agent,
            targets,
            original_targets,
            last_state: vec![],
            scalex,
            scaley,
            action_space,
            prev_target_dist: 0.0,
            starting_points,
        }
    }

    pub fn get_line_strings_as_lines(&self) -> Vec<HashMap<&str, f64>> {
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
        res
    }

    pub fn get_targets_as_points(&self) -> Vec<HashMap<&str, f64>> {
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
        res
    }

    pub fn step(&mut self, action: i32) -> (Vec<f64>, f64, bool) {
        let mut move_forward = false;
        let mut reward = (self.agent.speed * -1.0);
        let direction_change = self.action_space.get(action as usize).unwrap().clone();
        if direction_change == 0.0 {
            move_forward = true
        }
        let step_ray = Ray::new(direction_change, self.agent.speed, self.agent.direction, self.agent.position);
        if utils::intersects(&step_ray, &self.line_strings.iter().collect()) {
            let state = self.last_state.iter().copied().collect();
            return (state, -2.0, true);
        }
        let proximity_ray = Ray::new(direction_change, self.agent.speed*3.0, self.agent.direction, self.agent.position);
        if utils::intersects(&proximity_ray, &self.line_strings.iter().collect()) {
            reward = reward + -0.5;
        }
        self.agent.step(direction_change, move_forward);
        self.update_agent();

        let (state ,target_in_sight) = self.get_state();
        let closest_target = utils::closest_of(self.targets.iter(), self.agent.position).unwrap();
        self.agent.closest_target = closest_target;
        if self.last_state.len() > 0 {
            if target_in_sight {//&& (self.last_state.get(0).unwrap() - state.get(0).unwrap()) > 0.0 {
                //reward = reward + self.agent.speed * 3.0;
                //println!("test {}", ((self.agent.fov/2.0) - state.get(1).unwrap().abs()));
                if self.last_state.get(0).unwrap() - state.get(0).unwrap() > 0.0 {
                    reward = reward + ((self.agent.fov/1.9) - state.get(1).unwrap().abs()) / 50.0;
                    let distance_score = self.last_state.get(0).unwrap() - state.get(0).unwrap();
                    reward = reward + distance_score;
                    //println!("distance score: {}", distance_score);
                }
                //println!("reward {}", reward);
            }
        }
        if state[0] < 0.02 {
            reward = 2.0;
            self.agent.age = 1.0;
            self.targets = self.targets.iter().filter(|p| **p != closest_target).cloned().collect();
            if self.targets.len() < 1 {
                self.targets = self.original_targets.to_vec();
            }
            self.agent.targets_count = self.agent.targets_count + 1;
            //self.targets.push(Point::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)));
        }
        self.last_state = state.iter().copied().collect();
        return (state, reward, false);
    }

    pub fn get_agent_rays(&self) -> Vec<HashMap<&str, f64>> {
        let mut res = vec![];
        for ray in self.agent.rays.iter() {
            for line in ray.line_string.lines() {
                let hashmap: HashMap<&str, f64> = [
                    ("start_x", line.start.x),
                    ("start_y", line.start.y),
                    ("end_x", line.end.x),
                    ("end_y", line.end.y),
                    ("length", ray.length),
                    ("angle", ray.angle),
                ]
                    .iter()
                    .cloned()
                    .collect();
                res.push(hashmap);
            }
        }
        res
    }

    pub fn get_state(&mut self) -> (Vec<f64>, bool) {
        let mut state = vec![];
        let mut can_see_target = false;
        let step_ray = Ray::new(0.0, self.agent.speed, self.agent.direction, self.agent.position);
        let closest_target = utils::closest_of(self.targets.iter(), self.agent.position).unwrap();
        let distance_to_target = self.agent.position.euclidean_distance(&closest_target);
        state.push(distance_to_target);
        let mut relative_bearing_to_target = utils::relative_bearing_to_target(self.agent.position, step_ray.line.end_point(), closest_target);
        if relative_bearing_to_target.abs() < self.agent.fov / 2.0 {
            for line in self.line_strings.iter() {
                can_see_target = true;
                let intersections = utils::intersections(&LineString(vec![closest_target.into(), self.agent.position.into()]), line);
                if intersections.len() > 0 {
                    can_see_target = false;
                }
            }
        }
        self.prev_target_dist = distance_to_target;
        state.push(relative_bearing_to_target);
        //state.push(self.agent.age/self.agent.max_age);
        let mut ray_lengths = self.agent.rays.iter().map(|r| r.length / r.max_length).collect();
        state.append(&mut ray_lengths);
        return (state, can_see_target);
    }

    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        let mut agent = Agent::new(
            self.starting_points.choose(&mut rand::thread_rng()).unwrap().clone(), 
            rng.gen_range(-3.14, 3.14)
        );
        self.last_state = vec![];
        agent.cast_rays();
        self.agent = agent;
        self.targets = self.original_targets.to_vec();
        self.update_agent();
    }

    pub fn update_agent(&mut self) {
        let intersecting_line_strings =
            utils::cull_line_strings_precull(&mut self.agent.rays_bb, &self.line_strings, self.agent.position);
        utils::find_intersections_seq(&mut self.agent.rays, &intersecting_line_strings, self.agent.position)
    }
}
#[cfg(test)]
mod tests {
    use geo::Point;
    use geo::bearing::Bearing;

    #[test]
    fn test_bearing() {
        let agent_position = Point::new(0.0, 0.0);
        let agent_step_ray_end = Point::new(-2.0, -2.0);
        let target_position = Point::new(2.0, 0.0);

        let target_bearing = agent_position.bearing(target_position);
        let step_bearing = agent_position.bearing(agent_step_ray_end);
        dbg!(target_bearing);
        dbg!(step_bearing);
        let d = target_bearing - step_bearing;
        dbg!(d);
        if d > 180.0 {
            dbg!(-180.0 + d - 180.0);
        } else if d < -180.0 {
            dbg!(180.0 + d + 180.0);
        }
    }
}