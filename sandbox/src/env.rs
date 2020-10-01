use crate::agent::Agent;
use crate::utils;
use geo::LineString;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub(crate) struct Env {
    pub line_strings: Vec<LineString<f64>>,
    #[pyo3(get, set)]
    scalex: f64,
    #[pyo3(get, set)]
    scaley: f64,
}

#[pymethods]
impl Env {
    #[new]
    fn new(path: String) -> Self {
        let (line_strings, scalex, scaley) = utils::import_line_strings(path);
        Env {
            line_strings,
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

    pub fn update_agent(&self, agent: &mut Agent) {
        let intersecting_line_strings =
            utils::cull_line_strings(&mut agent.rays, &self.line_strings, agent.position);
        utils::find_intersections_seq(&mut agent.rays, &intersecting_line_strings, agent.position)
    }
}
