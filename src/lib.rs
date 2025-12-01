use pyo3::{pyfunction, wrap_pyfunction, Bound, PyResult};
use pyo3::types::PyModule;
use pyo3::prelude::*;
use crate::client::PyDut;

mod client;
mod config;
mod rfmetrics;
mod testcase;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[pyfunction]
fn init_logger() -> PyResult<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[pymodule]
fn iq_dump(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_logger, m)?)?;
    m.add_class::<PyDut>()?;
    Ok(())
}

