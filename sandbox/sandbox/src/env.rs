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
            original_targets: targets.clone(),
            line_strings,
            targets,
            scalex,
            scaley,
            xmin,
            ymin,
            agents: agents,
            max_steps,
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
        let direction_change = self.agents[a as usize].action_space.get(action as usize).unwrap().clone();
        let mut reward = 0.0;

        let step_ray = Ray::new(direction_change, self.agents[a as usize].speed, self.agents[a as usize].direction, self.agents[a as usize].position, false, 0.0);
        if utils::intersects(&step_ray, &self.line_strings.par_iter().collect()) {
            let state = self.agents[a as usize].last_state.iter().copied().collect();
            reward = -7.0;
            self.agents[a as usize].add_to_memory(&state, action, reward, true);
            self.agents[a as usize].active = false;
            return (state, reward, true);
        }

        let proximity_ray = Ray::new(direction_change, self.agents[a as usize].speed*3.0, self.agents[a as usize].direction, self.agents[a as usize].position, false, 0.0);
        if utils::intersects(&proximity_ray, &self.line_strings.par_iter().collect()) {
            reward = -2.0;
        } 

        self.agents[a as usize].age = self.agents[a as usize].age + 1.0;
        self.agents[a as usize].step(action as usize);
        
        let (mut state, closest_target) = self.get_state(a, step_ray);
        self.agents[a as usize].closest_target = closest_target;
        reward = reward - state[1] / 3.0;
        //println!("bearing {}", state[0]);
        //println!("bearing reward {}", reward - state[0].abs());
        reward = reward - state[0].abs() / 3.0;
        if state[1]*self.agents[a as usize].max_age < 10.0 {
            state = self.agents[a as usize].last_state.iter().copied().collect();
            reward = 7.0;
            self.agents[a as usize].collect_target(closest_target, self.targets.len() as i32);
        }

        self.agents[a as usize].last_state = state.iter().copied().collect();
        self.agents[a as usize].add_to_memory(&state, action, reward, false);
        return (state, reward, false);
    }

    // pub fn step_old(&mut self, action: i32, a: i32) -> (Vec<f64>, f64, bool) {
    //     let mut full_move = false;
    //     let mut reward = -0.08;
    //     let direction_change = self.agents[a as usize].action_space.get(action as usize).unwrap().clone();
    //     if direction_change.abs() < 3.0f64.to_radians() {
    //         full_move = true
    //     } 
    //     let step_ray = Ray::new(direction_change, self.agents[a as usize].speed, self.agents[a as usize].direction, self.agents[a as usize].position, false, 0.0);
    //     if utils::intersects(&step_ray, &self.line_strings.par_iter().collect()) {
    //         let state = self.agents[a as usize].last_state.iter().copied().collect();
    //         reward = -4.0;
    //         self.agents[a as usize].add_to_memory(&state, action, reward, true);
    //         self.agents[a as usize].active = false;
    //         return (state, reward, true);
    //     }
    //     let prev_past_position_dist = self.agents[a as usize].past_position_distance;
    //     self.agents[a as usize].step(action as usize, full_move);
    //     let (state ,target_in_sight) = self.get_state(a);
    //     let mut possible_targets = vec![];
    //     for target in self.targets.iter() {
    //         if !self.agents[a as usize].collected_targets.iter().any(|&t| t==target.clone()) {
    //             possible_targets.push(target);
    //         }
    //     }
    //     let closest_target = utils::closest_of(possible_targets.iter(), self.agents[a as usize].position).unwrap();
    //     let mut distance_to_target = self.agents[a as usize].position.euclidean_distance(&closest_target);
    //     self.agents[a as usize].closest_target = closest_target;
    //     // if self.agents[a as usize].last_state.len() > 0 {
    //     // //     let target_turn_value = self.agents[a as usize].bearing_to_target.abs() - state[0 as usize].abs();
    //     // //     if target_turn_value >= 0.0 || self.agents[a as usize].bearing_to_target.abs() <= 2.0f64.to_radians() {
    //     // //         reward = reward + 0.16;
    //     // //         if target_in_sight && target_turn_value > 0.0 {
    //     // //             reward = reward + target_turn_value;
    //     // //             //println!("Turn bonus {}", target_turn_value);
    //     // //         }
    //     // //     }
    //     if self.agents[a as usize].last_state.len() > 0 {
    //         let distance_target_value = self.agents[a as usize].prev_target_dist - distance_to_target;
    //         if distance_target_value <= 0.0 {
    //             self.agents[a as usize].age = self.agents[a as usize].age + 1.0;
    //         } else {
    //             reward = reward + 1.0;
    //         }
    //     }

    //     // let mut distance_to_target_scaled = distance_to_target / 10.0;
    //     // if distance_to_target_scaled > 0.99 {
    //     //     distance_to_target_scaled = 0.99;
    //     // }
    //     //reward = reward + 1.0; //(1.0-distance_to_target_scaled).log10();
    //             //let distance_score = 1.0 - (distance_to_target / self.agents[a as usize].prev_target_dist);
    //             //println!("Distance bonus {}", distance_target_value);
    //             // if target_in_sight {
    //             //     reward = reward + distance_target_value * 10.0;
                    
    //             // }
    //             //reward = reward + 0.16; //(distance_score / 3.0); // not / 3.0
    //        // } //else {
    //     //         self.agents[a as usize].age = self.agents[a as usize].age + 1.0;
    //     //     } 
    //     // }
    //     // if self.agents[a as usize].last_state.len() > 0 && !target_in_sight {
    //     //     let distance_target_value = self.agents[a as usize].prev_target_dist - distance_to_target;
    //     //     if distance_target_value < 0.0 {
    //     //         self.agents[a as usize].age = self.agents[a as usize].age + 1.0;
    //     //     } else {
    //     //         reward = reward + 0.1;
    //     //     }
    //     // }
    //     //     if distance_target_value > 0.0 {
    //     self.agents[a as usize].bearing_to_target = state[0 as usize];
    //     // if state.get(0).unwrap().abs() / 3.14 < 0.5 {
    //     //     reward = reward + (1.0 - (state.get(0).unwrap().abs() / 3.14)) / 5.0;
    //     //     if target_in_sight {
    //     //         reward = reward + 0.01;
    //     //     }
    //     // }
    //     // if !target_in_sight {
    //     //     self.agents[a as usize].age = self.agents[a as usize].age + 1.0;
    //     // }
    //     for i in 0..self.agents[a as usize].action_space.len() {
    //         let pr = Ray::new(self.agents[a as usize].action_space.get(i).unwrap().clone(), self.agents[a as usize].speed*2.0, self.agents[a as usize].direction, self.agents[a as usize].position, false, 0.0);
    //         if utils::intersects(&pr, &self.line_strings.par_iter().collect()) {
    //             reward = reward - 1.5;
    //             break;
    //         }
    //     }
    //     if !target_in_sight && prev_past_position_dist - self.agents[a as usize].past_position_distance > 0.0 {
    //         //let mut backtrack_penalty =  0.0; //(1.0 - (distance_to_target / self.agents[a as usize].prev_target_dist));
    //         //let backtrack_penalty = self.agents[a as usize].past_position_bearing.abs() / 3.0;
    //         reward = reward - 0.1; //backtrack_penalty / 5.0; // 5.0
    //     }
    //     // // reward = reward - self.agents[a as usize].age / self.agents[a as usize].max_age;
    //     //println!("distance to target: {}", distance_to_target);
    //     if distance_to_target < 0.1 {
    //         reward = 4.0;
    //         distance_to_target = 1.0;
    //         self.agents[a as usize].collect_target(closest_target, self.targets.len() as i32);
    //     }
    //     if target_in_sight {
    //         self.agents[a as usize].past_positions = vec![self.agents[a as usize].position]; 
    //     }
    //     self.agents[a as usize].prev_target_dist = distance_to_target;
    //     self.agents[a as usize].last_state = state.iter().copied().collect();
    //     self.agents[a as usize].add_to_memory(&state, action, reward, false);
        
    //     return (state, reward, false);
    // }

    pub fn get_state(&mut self, a: i32, mut step_ray: Ray) -> (Vec<f64>, Point<f64>) {
        let step_ray = Ray::new(0.0, self.agents[a as usize].speed, self.agents[a as usize].direction, self.agents[a as usize].position, false, 0.0);
        let mut state = vec![];
        let mut possible_targets = vec![];
        for target in self.targets.iter() {
            if !self.agents[a as usize].collected_targets.iter().any(|&t| t==target.clone()) {
                possible_targets.push(target);
            }
        }
        let closest_target = utils::closest_of(possible_targets.iter(), self.agents[a as usize].position).unwrap();
        let relative_bearing_to_target = utils::relative_bearing_to_target(self.agents[a as usize].position, step_ray.line.end_point(), closest_target);
        state.push(relative_bearing_to_target / 3.14159);

        let distance_to_target = self.agents[a as usize].position.euclidean_distance(&closest_target);
        let steps_to_target = (distance_to_target / self.agents[a as usize].speed) / self.agents[a as usize].max_age;
        state.push(steps_to_target);
        state.push(self.agents[a as usize].past_position_bearing / 3.14159);
        let mut ray_lengths = self.agents[a as usize].rays.par_iter().map(|r| r.length / r.max_length).collect();
        state.append(&mut ray_lengths);
        return (state, closest_target);
    }

    // pub fn get_state(&mut self, a: i32) -> (Vec<f64>, bool) {
    //     let mut state = vec![];
    //     let mut can_see_target = false;
    //     let step_ray = Ray::new(0.0, self.agents[a as usize].speed, self.agents[a as usize].direction, self.agents[a as usize].position, false, 0.0);
    //     let mut possible_targets = vec![];
    //     for target in self.targets.iter() {
    //         if !self.agents[a as usize].collected_targets.iter().any(|&t| t==target.clone()) {
    //             possible_targets.push(target);
    //         }
    //     }
    //     let closest_target = utils::closest_of(possible_targets.iter(), self.agents[a as usize].position).unwrap();
    //     let mut distance_to_target = self.agents[a as usize].position.euclidean_distance(&closest_target) / 10.0;
    //     if distance_to_target > 0.99 {
    //         distance_to_target = 0.99;
    //     }
    //     let relative_bearing_to_target = utils::relative_bearing_to_target(self.agents[a as usize].position, step_ray.line.end_point(), closest_target);
    //     // if relative_bearing_to_target.abs() <= self.agents[a as usize].fov{
    //     //     can_see_target = true;
    //     //     for line in self.line_strings.iter() {
    //     //         let intersections = utils::intersections(&LineString(vec![closest_target.into(), self.agents[a as usize].position.into()]), line);
    //     //         if intersections.len() > 0 {
    //     //             can_see_target = false;
    //     //             break;
    //     //         }
    //     //     }
    //     // }
    //     state.push(relative_bearing_to_target / 3.14159);
    //     //state.push(distance_to_target);
    //     // if can_see_target {
    //     //     state.push(1.0);
    //     // } else {
    //     //     state.push(0.0);
    //     // }
    //     //state.push(self.agents[a as usize].past_position_distance);
    //     state.push(self.agents[a as usize].past_position_bearing / 3.14159);
    //     // // state.push(self.agents[a as usize].age / self.agents[a as usize].max_age);
    //     let mut ray_lengths = self.agents[a as usize].rays.par_iter().map(|r| r.length / r.max_length).collect();
    //     state.append(&mut ray_lengths);
    //     return (state, can_see_target);
    // }

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
        println!("### target count: {} ###",  self.targets.len());
        self.targets.shuffle(&mut rand::thread_rng());
        let start = self.targets.choose(&mut rand::thread_rng()).unwrap().clone();
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