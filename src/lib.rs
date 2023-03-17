use std::path::Path;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use ruff::linter::{lint_only, LinterResult};
use ruff::settings::flags::{Autofix, Noqa};
use ruff::settings::Settings;
use wrappers::Message;

mod wrappers;

#[pyfunction]
fn lint(content: &str) -> PyResult<Vec<Message>> {
    let LinterResult { data, error } = lint_only(
        content,
        Path::new("-"),
        None,
        &Settings::default(),
        Noqa::Enabled,
        Autofix::Disabled,
    );
    if let Some(error) = error {
        Err(PyException::new_err(error.to_string()))
    } else {
        Ok(data.into_iter().map(Message::from).collect())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ruff_api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(lint, m)?)?;
    Ok(())
}
