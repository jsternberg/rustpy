use pyo3::prelude::*;
use pyo3::types::{PyDict};

#[pyfunction]
fn num_elements(arg0: &PyDict) -> usize {
    arg0.len()
}

#[pymodule]
fn ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(num_elements, m)?)?;
    Ok(())
}