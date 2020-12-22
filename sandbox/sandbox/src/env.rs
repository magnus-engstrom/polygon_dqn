use crate::agent::Agent;
use crate::utils;
use geo::{LineString, Point};
use std::collections::HashMap;
use crate::ray::Ray;
use geo::euclidean_distance::EuclideanDistance;
use rand;
use rand::seq::SliceRandom;
use rayon::prelude::*;

pub struct Env {
    pub line_strings: Vec<LineString<f64>>,
    pub targets: Vec<Point<f64>>,
    pub scalex: f64,
    pub scaley: f64,
    pub xmin: f64,
    pub ymin: f64,
    pub agents: Vec<Agent>,
}

impl Env {
    pub fn new(path: String, agent_count: i32) -> Self {
        let (line_strings, targets, scalex, scaley, xmin, ymin) = utils::import_geometry(path);
        let mut agents = vec![];
        for i in 0..agent_count {
            agents.push(Agent::new(targets.choose(&mut rand::thread_rng()).unwrap().clone(), line_strings.clone()));
        }
        Env {
            line_strings,
            targets,
            scalex,
            scaley,
            xmin,
            ymin,
            agents: agents,
        }
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
        let mut full_move = false;
        let mut reward = -0.06;
        let direction_change = self.agents[a as usize].action_space.get(action as usize).unwrap().clone();
        if direction_change.abs() < 3.0f64.to_radians() {
            full_move = true
        }
        let step_ray = Ray::new(direction_change, self.agents[a as usize].speed, self.agents[a as usize].direction, self.agents[a as usize].position, false);
        if utils::intersects(&step_ray, &self.line_strings.par_iter().collect()) {
            let state = self.agents[a as usize].last_state.iter().copied().collect();
            self.agents[a as usize].add_to_memory(&state, action, reward, true);
            self.agents[a as usize].active = false;
            return (state, -3.0, true);
        }
        let prev_past_position_dist = self.agents[a as usize].past_position_distance;
        self.agents[a as usize].step(action as usize, full_move);
        let (state ,target_in_sight) = self.get_state(a);
        let mut possible_targets = vec![];
        for target in self.targets.iter() {
            if !self.agents[a as usize].collected_targets.iter().any(|&t| t==target.clone()) {
                possible_targets.push(target);
            }
        }
        let closest_target = utils::closest_of(possible_targets.iter(), self.agents[a as usize].position).unwrap();
        let mut distance_to_target = self.agents[a as usize].position.euclidean_distance(&closest_target);
        self.agents[a as usize].closest_target = closest_target;
        reward = reward + (1.0 - (state.get(0).unwrap().abs() / 3.14)) / 5.0;
        if self.agents[a as usize].last_state.len() > 0 && self.agents[a as usize].prev_target_dist - distance_to_target > 0.0 {
            let distance_score = 1.0 - (distance_to_target / self.agents[a as usize].prev_target_dist);
            reward = reward + (distance_score); 
        } else {
            self.agents[a as usize].age = self.agents[a as usize].age + 1.0;
        }  
        for i in 0..self.agents[a as usize].action_space.len() {
            let pr = Ray::new(self.agents[a as usize].action_space.get(i).unwrap().clone(), self.agents[a as usize].speed*3.0, self.agents[a as usize].direction, self.agents[a as usize].position, false);
            if utils::intersects(&pr, &self.line_strings.par_iter().collect()) {
                reward = -0.2;
                break;
            }
        }
        if !target_in_sight && prev_past_position_dist - self.agents[a as usize].past_position_distance > 0.0 {
            let mut backtrack_penalty =  (1.0 - (distance_to_target / self.agents[a as usize].prev_target_dist)) * 2.0;
            backtrack_penalty = backtrack_penalty + (1.0 - (self.agents[a as usize].past_position_bearing.abs() / 3.14)) / 4.0;
            if backtrack_penalty > 0.0 {
                reward = reward + backtrack_penalty * -1.0;
                //println!("backtrack penalty {}", reward)
            }
        }
        if distance_to_target < 0.02 {
            reward = 3.5;
            distance_to_target = 1.0;
            self.agents[a as usize].collect_target(closest_target, self.targets.len() as i32);
        }
        self.agents[a as usize].prev_target_dist = distance_to_target;
        self.agents[a as usize].last_state = state.iter().copied().collect();
        self.agents[a as usize].add_to_memory(&state, action, reward, false);
        return (state, reward, !self.agents[a as usize].active);
    }

    pub fn get_state(&mut self, a: i32) -> (Vec<f64>, bool) {
        let mut state = vec![];
        let mut can_see_target = false;
        let step_ray = Ray::new(0.0, self.agents[a as usize].speed, self.agents[a as usize].direction, self.agents[a as usize].position, false);
        let mut possible_targets = vec![];
        for target in self.targets.iter() {
            if !self.agents[a as usize].collected_targets.iter().any(|&t| t==target.clone()) {
                possible_targets.push(target);
            }
        }
        let closest_target = utils::closest_of(possible_targets.iter(), self.agents[a as usize].position).unwrap();
        let distance_to_target = self.agents[a as usize].position.euclidean_distance(&closest_target);
        let relative_bearing_to_target = utils::relative_bearing_to_target(self.agents[a as usize].position, step_ray.line.end_point(), closest_target);
        if relative_bearing_to_target.abs() <= self.agents[a as usize].fov * 1.3 {
            can_see_target = true;
            for line in self.line_strings.iter() {
                let intersections = utils::intersections(&LineString(vec![closest_target.into(), self.agents[a as usize].position.into()]), line);
                if intersections.len() > 0 {
                    can_see_target = false;
                    break;
                }
            }
        }
        state.push(relative_bearing_to_target);
        state.push(distance_to_target);
        if can_see_target {
            state.push(1.0);
        } else {
            state.push(0.0);
        }
        state.push(self.agents[a as usize].past_position_distance);
        state.push(self.agents[a as usize].past_position_bearing / 3.14);
        let mut ray_lengths = self.agents[a as usize].rays.par_iter().map(|r| r.length / r.max_length).collect();
        state.append(&mut ray_lengths);
        return (state, can_see_target);
    }

    pub fn reset(&mut self, agent_index: i32) {
        let start = self.targets.choose(&mut rand::thread_rng()).unwrap().clone();
        self.agents[agent_index as usize] = Agent::new(start, self.line_strings.clone());
        self.agents[agent_index as usize].cast_rays();
        //let mut agent = Agent::new(start, self.line_strings.clone());
        //agent.cast_rays();
        //self.agent = agent;
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