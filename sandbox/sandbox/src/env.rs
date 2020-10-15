use crate::agent::Agent;
use crate::utils;
use geo::{LineString, Point, Closest};
use std::collections::HashMap;
use crate::ray::Ray;
use geo::euclidean_distance::EuclideanDistance;
use geo::bearing::Bearing;
use geo::closest_point::ClosestPoint;

pub struct Env {
    pub line_strings: Vec<LineString<f64>>,
    pub agent: Agent,
    pub targets: Vec<Point<f64>>,
    pub last_state: Vec<f64>,
    scalex: f64,
    scaley: f64,
}

impl Env {
    pub fn new(path: String) -> Self {
        let (line_strings, targets, scalex, scaley) = utils::import_geometry(path);
        let agent = Agent::new((0.5, 0.5), 0.1);
        Env {
            line_strings,
            agent,
            targets,
            last_state: vec![],
            scalex,
            scaley,
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
            let state = self.last_state.iter().copied().collect();
            return (state, -2.0, true);
        }
        self.agent.step(direction_change);
        self.update_agent();

        let state = self.get_state();
        self.last_state = state.iter().copied().collect();
        return (state, 0.25, false);
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

    pub fn get_state(&self) -> Vec<f64> {
        let mut state = vec![];
        let step_ray = Ray::new(0.0, self.agent.speed, self.agent.direction, self.agent.position);
        let closest_target = utils::closest_of(self.targets.iter(), self.agent.position).unwrap();
        let distance_to_target = self.agent.position.euclidean_distance(&closest_target);
        state.push(distance_to_target);
        let relative_bearing_to_target = utils::relative_bearing_to_target(self.agent.position, step_ray.line.end_point(), closest_target);
        state.push(relative_bearing_to_target);
        let mut ray_lengths = self.agent.rays.iter().map(|r| r.length / r.max_length).collect();
        state.append(&mut ray_lengths);
        return state;
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