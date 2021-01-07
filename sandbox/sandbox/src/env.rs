use crate::agent::Agent;
use crate::utils;
use geo::{LineString, Point};
use std::collections::HashMap;
use crate::ray::Ray;
use geo::euclidean_distance::EuclideanDistance;
use rand;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use rand::prelude::IteratorRandom;
use std::f32::consts::PI;

pub struct Env {
    pub line_strings: Vec<LineString<f64>>,
    pub targets: Vec<Point<f64>>,
    pub possible_targets: Vec<Point<f64>>,
    pub scalex: f64,
    pub scaley: f64,
    pub xmin: f64,
    pub ymin: f64,
    pub agents: Vec<Agent>,
    pub max_steps: i32,
    pub original_targets: Vec<Point<f64>>,
}

impl Env {
    pub fn new(path: String, agent_count: i32, max_steps: i32) -> Self {
        let (line_strings, targets, scalex, scaley, xmin, ymin) = utils::import_geometry(path);
        let mut agents = vec![];
        for i in 0..agent_count {
            agents.push(Agent::new(targets.choose(&mut rand::thread_rng()).unwrap().clone(), line_strings.clone(), max_steps));
        }
        Env {
            original_targets: targets.iter().copied().collect(),
            line_strings,
            possible_targets: targets.iter().copied().collect(),
            targets,
            scalex,
            scaley,
            xmin,
            ymin,
            agents: agents,
            max_steps,
        }
    }

    pub fn action_space(&self) -> usize {
        self.agents.get(0).unwrap().action_space.len()
    }

    pub fn observation_space(&self) -> usize {
        /*
        relative_bearing_to_target,
        steps_to_target,
        past_position_bearing,
        ray
        */
        3 + self.agents.get(0).unwrap().ray_count as usize
    }

    pub fn get_line_strings_as_lines(&self) -> Vec<HashMap<&str, f64>> {
        let mut res = vec![];
        self.line_strings.iter().for_each(|l| {
            for line in l.lines() {
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
        });
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

    pub fn step(&mut self, action: i32, a: i32) -> (Vec<f64>, f64, bool) {
        let direction_change = self.agents[a as usize].action_space.get(action as usize).unwrap().clone();
        let mut reward = 0.0;

        let step_ray = Ray::new(direction_change, self.agents[a as usize].speed, self.agents[a as usize].direction, self.agents[a as usize].position, false, 0.0);
        if utils::intersects(&step_ray, &self.line_strings.iter().collect()) {
            let state = self.agents[a as usize].last_state.iter().copied().collect();
            reward = -7.0;
            //self.agents[a as usize].add_to_memory(&state, action, reward, true);
            self.agents[a as usize].active = false;
            return (state, reward, true);
        }

        /*
        let proximity_ray = Ray::new(direction_change, self.agents[a as usize].speed*3.0, self.agents[a as usize].direction, self.agents[a as usize].position, false, 0.0);
        if utils::intersects(&proximity_ray, &self.line_strings.iter().collect()) {
            reward = -2.0;
        }
         */

        self.agents[a as usize].age = self.agents[a as usize].age + 1.0;
        self.agents[a as usize].step(action as usize);

        let (mut state, closest_target) = self.get_state(a, step_ray);
        //self.agents[a as usize].closest_target = closest_target;
        reward = reward - state[1] / 3.0;
        //println!("bearing {}", state[0]);
        //println!("bearing reward {}", reward - state[0].abs());
        reward = reward - state[0].abs() / 3.0;
        if state[1]*1000.0 < 10.0 {
            state = self.agents[a as usize].last_state.iter().copied().collect();
            reward = 7.0;
            self.possible_targets.retain(|t| t.x_y() != closest_target.x_y());
            self.agents[a as usize].collect_target(closest_target, self.targets.len() as i32);
        }

        self.agents[a as usize].last_state = state.iter().copied().collect();
        //self.agents[a as usize].add_to_memory(&state, action, reward, false);
        return (state, reward, false);
    }

    pub fn get_state(&mut self, a: i32, mut step_ray: Ray) -> (Vec<f64>, Point<f64>) {
        let step_ray = Ray::new(0.0, self.agents[a as usize].speed, self.agents[a as usize].direction, self.agents[a as usize].position, false, 0.0);
        let mut state = vec![];
        let closest_target = utils::closest_of(self.possible_targets.iter(), self.agents[a as usize].position).unwrap();
        let relative_bearing_to_target = utils::relative_bearing_to_target(self.agents[a as usize].position, step_ray.line.end_point(), closest_target);
        state.push(relative_bearing_to_target / 3.14159);

        let distance_to_target = self.agents[a as usize].position.euclidean_distance(&closest_target);
        let steps_to_target = (distance_to_target / self.agents[a as usize].speed) / 1000.0;
        state.push(steps_to_target);
        state.push(self.agents[a as usize].past_position_bearing / 3.14159);
        let mut ray_lengths = self.agents[a as usize].rays.iter().map(|r| r.length / r.max_length).collect();
        state.append(&mut ray_lengths);
        return (state, closest_target);
    }

    pub fn reset(&mut self, agent_index: i32, epsilon: f64) {
        let mut new_targets = vec![];
        let mut take_targets = self.original_targets.len() as f64;
        if self.original_targets.len() as f64*(epsilon+epsilon) + 10.0 < self.original_targets.len() as f64 {
            take_targets = self.original_targets.len() as f64*(epsilon+epsilon) + 10.0;
        }
        self.original_targets.iter().choose_multiple(&mut rand::thread_rng(), take_targets as usize).iter().for_each(|p| {
            new_targets.push(p.clone().clone());
        });
        self.targets = new_targets.clone();
        self.targets.shuffle(&mut rand::thread_rng());
        let start = self.targets.choose(&mut rand::thread_rng()).unwrap().clone();
        self.possible_targets = self.targets.iter().filter(|t| t.x_y() != start.x_y()).copied().collect();
        self.agents[agent_index as usize] = Agent::new(start, self.line_strings.clone(), self.max_steps + ((1.0 - epsilon) * 1000.0) as i32);
        self.agents[agent_index as usize].cast_rays();
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