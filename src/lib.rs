use pyo3::prelude::*;

#[pyfunction]
fn fill_field(name: &str) -> PyResult<String> {
    Ok(format!("Filled: {}", name))
}

#[pymodule]
fn pypdfform_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fill_field, m)?)?;
    Ok(())
}
