use pyo3::{pyfunction, wrap_pyfunction, Bound, PyResult};
use pyo3::types::PyModule;
use pyo3::prelude::*;
use walkdir::WalkDir;
use crate::client::PyDut;
use crate::rfmetrics::FileParser;

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

#[pyfunction]
fn parse_dir(dir: String) -> PyResult<()> {
    let mut file_list = FileParser::new(Vec::new());
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|ext| ext == "txt").unwrap_or(false)) {
        file_list.add_file(entry.path().display().to_string());
    }
    file_list.sort_file()
        .parse_and_write().unwrap();
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
    m.add_function(wrap_pyfunction!(parse_dir, m)?)?;
    m.add_class::<PyDut>()?;
    Ok(())
}

