use std::collections::HashMap;

use pyo3::prelude::*;
use sandbox::env::Env as REnv;
use sandbox::utils;
#[pyclass]
pub(crate) struct Env {
    pub env: REnv,
}

#[pymethods]
impl Env {
    #[new]
    fn new(path: String) -> Self {
        let env = REnv::new(path);
        Env {
            env
        }
    }

    #[getter(action_space)]
    fn get_action_space(&self) -> PyResult<Vec<f64>> {
        Ok(self.env.action_space.clone())
    }

    #[getter(lines)]
    fn get_line_strings_as_lines(&self) -> PyResult<Vec<HashMap<&str, f64>>> {
        Ok(self.env.get_line_strings_as_lines())
    }

    #[getter(targets)]
    fn get_targets_as_points(&self) -> PyResult<Vec<HashMap<&str, f64>>> {
        Ok(self.env.get_targets_as_points())
    }

    #[getter(ray_count)]
    fn get_ray_count(&self) -> PyResult<f64> {
        Ok(self.env.agent.ray_count)
    }

    #[getter(agent_age)]
    fn get_agent_age(&self) -> PyResult<f64> {
        Ok(self.env.agent.age)
    }


    #[getter(agent_position)]
    fn get_agent_position(&self) -> PyResult<(f64, f64)> {
        Ok(self.env.agent.position.x_y())
    }

    #[getter(agent_closest_target)]
    fn get_agent_closest_target(&self) -> PyResult<(f64, f64)> {
        Ok(self.env.agent.closest_target.x_y())
    }

    pub fn step(&mut self, action: i32) -> (Vec<f64>, f64, bool) {
        self.env.step(action)
    }
    pub fn get_agent_rays(&self) -> PyResult<Vec<HashMap<&str, f64>>> {
        Ok(self.env.get_agent_rays())
    }

    pub fn get_agent_targets_count(&self) -> PyResult<i32> {
        Ok(self.env.agent.targets_count)
    }

    pub fn get_state(&mut self) -> Vec<f64> {
        let (state, _) = &mut self.env.get_state();
        return state.clone()
    }

    pub fn reset(&mut self) {
        self.env.reset()
    }

    pub fn update_agent(&mut self) {
        self.env.update_agent();
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