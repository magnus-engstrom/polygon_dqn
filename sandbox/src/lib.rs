use pyo3::prelude::*;

mod utils;
mod env;
mod agent;
mod ray;

use crate::env::Env;
use crate::agent::Agent;


#[pymodule]
fn sandbox(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Env>()?;
    m.add_class::<Agent>()?;
    Ok(())
}
