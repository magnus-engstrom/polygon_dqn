use pyo3::prelude::*;

mod env;
use env::Env;

#[pymodule]
fn sandbox_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Env>()?;
    Ok(())
}
