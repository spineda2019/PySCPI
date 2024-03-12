use pyo3::prelude::*;

#[pymodule]
fn scpi_sender_pylib(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}
