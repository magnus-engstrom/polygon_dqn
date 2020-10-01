#![feature(test)]
extern crate test;

use pyo3::prelude::*;

mod agent;
mod env;
mod ray;
mod utils;

use crate::agent::Agent;
use crate::env::Env;

#[pymodule]
fn sandbox(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Env>()?;
    m.add_class::<Agent>()?;
    Ok(())
}
