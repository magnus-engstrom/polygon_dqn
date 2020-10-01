use crate::agent::Agent;
use crate::utils;
use geo::LineString;
use pyo3::prelude::*;

#[pyclass]
pub(crate) struct Env {
    //#[pyo3(get, set)]
    //pub line_strings: Arc<Mutex<Vec<LineString<f64>>>>,
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

    #[getter]
    fn get_line_strings(&self) -> PyResult<Vec<Vec<(f64, f64, f64, f64)>>> {
        let as_tuples = self
            .line_strings
            .iter()
            .map(|line_string| {
                line_string
                    .lines()
                    .map(|line| (line.start.x, line.start.y, line.end.x, line.end.y))
                    .collect()
            })
            .collect();
        Ok(as_tuples)
    }

    pub fn update_agent(&self, agent: &mut Agent) {
        let intersecting_line_strings =
            utils::cull_line_strings(&mut agent.rays, &self.line_strings, agent.position);
        utils::find_intersections_seq(&mut agent.rays, &intersecting_line_strings, agent.position)
    }

    /*
    #[setter]
    fn set_line_strings(&mut self, value: i32) -> PyResult<()> {
        self.num = value;
        Ok(())
    }

     */
}
