use std::collections::HashMap;
use pyo3::prelude::*;
use sandbox::env::Env as REnv;
use sandbox::utils;
use geo::algorithm::map_coords::MapCoordsInplace;
use geojson::{Feature, GeoJson, Geometry, FeatureCollection};
use geo;
use serde_json::{Map};
use line_intersection::{LineInterval, LineRelation};
use rand;
use rand::seq::SliceRandom;
use rand::Rng;
use std::cmp;

#[pyclass]
pub(crate) struct Env {
    pub env: REnv,
}

#[pymethods]
impl Env {
    #[new]
    fn new(path: String, agent_count: i32) -> Self {
        let env = REnv::new(path, agent_count);
        Env {
            env
        }
    }

    fn action_space(&self, agent_index: i32) -> PyResult<Vec<f64>> {
        Ok(self.env.agents[agent_index as usize].action_space.clone())
    }

    #[getter(lines)]
    fn get_line_strings_as_lines(&self) -> PyResult<Vec<HashMap<&str, f64>>> {
        Ok(self.env.get_line_strings_as_lines())
    }

    #[getter(targets)]
    fn get_targets_as_points(&self) -> PyResult<Vec<HashMap<&str, f64>>> {
        Ok(self.env.get_targets_as_points())
    }

    fn ray_count(&self, agent_index: i32) -> PyResult<f64> {
        Ok(self.env.agents[agent_index as usize].ray_count + 2.0)
    }

    fn agent_age(&self, agent_index: i32) -> PyResult<f64> {
        Ok(self.env.agents[agent_index as usize].age)
    }

    fn agent_memory_size(&self, agent_index: i32) -> PyResult<i32> {
        Ok(self.env.agents[agent_index as usize].memory.len() as i32)
    }

    pub fn agent_active(&self, agent_index: i32) -> PyResult<bool> {
        Ok(self.env.agents[agent_index as usize].active)
    }

    fn agent_collected_targets(&self, agent_index: i32) -> PyResult<Vec<(f64, f64)>> {
        Ok(self.env.agents[agent_index as usize].collected_targets.iter().map(|t| t.x_y()).collect())
    }

    pub fn agent_position(&self, agent_index: i32) -> PyResult<(f64, f64)> {
        Ok(self.env.agents[agent_index as usize].position.x_y())
    }

    pub fn agent_past_position(&self, agent_index: i32) -> PyResult<(f64, f64)> {
        Ok(utils::closest_of(
            self.env.agents[agent_index as usize].past_positions.iter(), 
            self.env.agents[agent_index as usize].position).unwrap().x_y())
    }

    pub fn agent_closest_target(&self, agent_index: i32) -> PyResult<(f64, f64)> {
        Ok(self.env.agents[agent_index as usize].closest_target.x_y())
    }

    pub fn agent_coordinates_path(&self, a: i32) -> PyResult<String> {
        let mut points_path_raw = self.env.agents[a as usize].get_coordinates_path().clone();
        let mut points_path_final = vec![points_path_raw[0 as usize]];
        let mut features = vec![];
        let mut smoothing_point_index = points_path_raw.len()-1;
        loop {
            let p1 = points_path_final[points_path_final.len()-1 as usize];
            let smoothing_point = points_path_raw[smoothing_point_index as usize];
            for (i, p2) in points_path_raw[..smoothing_point_index].iter_mut().enumerate().rev() {
                println!("index {}", i);
                let check_line: geo::LineString<f64> = vec![p1.x_y(), p2.x_y()].into();
                let mut intersections = vec![];
                for l in self.env.line_strings.iter() {
                    for a in l.lines() {
                        for b in check_line.lines() {
                            let a_li = LineInterval::line_segment(a);
                            let b_li = LineInterval::line_segment(b);
                            match a_li.relate(&b_li) {
                                LineRelation::DivergentIntersecting(x) => intersections.push(x),
                                _ => {}
                            }
                        }
                    }
                    if intersections.len() > 0 {
                        break;
                    }
                }   
                if i < 10 {
                    points_path_raw.drain(0..i);
                    break;
                }             
                if intersections.len() < 1 {
                    if smoothing_point_index > 0 && smoothing_point_index < i {
                        println!("inserting smoothing point");
                        points_path_final.push(smoothing_point);
                    }
                    points_path_final.push(p2.clone());
                    points_path_raw.drain(0..i);
                    smoothing_point_index = rand::thread_rng().gen_range(0, points_path_raw.len()-1);
                    break;
                }
            }
            if points_path_raw.len() < 10 {
                break;
            }
        }
        points_path_final.push(points_path_raw[points_path_raw.len()-1 as usize]);
        for point in points_path_final.iter_mut() {
            point.map_coords_inplace(|&(x, y)| ((x * self.env.scalex + self.env.xmin), (y * self.env.scaley + self.env.ymin)));
            let geometry = Geometry::new(
                geojson::Value::from(&point.clone())
            );
            features.push(Feature {
                bbox: None,
                geometry: Some(geometry),
                id: None,
                properties: Some(Map::new()),
                foreign_members: None,
            });
        }
        let feature_collection = FeatureCollection {
            bbox: None,
            features: features,
            foreign_members: None,
        };

        let serialized = GeoJson::from(feature_collection).to_string();
        Ok(serialized)
    }

    pub fn agent_memory(&self, agent_index: i32) -> PyResult<Vec<Py<PyAny>>> {
        Ok(self.env.agents[agent_index as usize].memory.clone())
    }

    pub fn step(&mut self, action: i32, agent_index: i32) -> (Vec<f64>, f64, bool) {
        self.env.step(action, agent_index)
    }
    pub fn agent_rays(&self, agent_index: i32) -> PyResult<Vec<HashMap<&str, f64>>> {
        Ok(self.env.agents[agent_index as usize].get_rays())
    }


    pub fn agent_targets_count(&self, agent_index: i32) -> PyResult<i32> {
        Ok(self.env.agents[agent_index as usize].collected_targets.len() as i32 - 1)
    }

    pub fn get_state(&mut self, agent_index: i32) -> Vec<f64> {
        let (state, _) = &mut self.env.get_state(agent_index);
        return state.clone()
    }

    pub fn reset(&mut self, agent_index: i32) {
        self.env.reset(agent_index)
    }

    // pub fn update_agent(&mut self) {
    //     self.env.update_agent();
    // }
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
