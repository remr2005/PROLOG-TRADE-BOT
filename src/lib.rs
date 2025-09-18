#[cfg(feature = "python")]
use pyo3::prelude::*;

pub fn sum_as_string_rs(a: usize, b: usize) -> String {
    (a + b).to_string()
}

#[cfg(feature = "python")]
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok(sum_as_string_rs(a, b))
}

#[cfg(feature = "python")]
#[pymodule]
fn my_project(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
