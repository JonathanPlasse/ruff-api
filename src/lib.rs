use std::path::Path;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use ruff::linter::{lint_only, LinterResult};
use ruff::settings::configuration::Configuration;
use ruff::settings::flags::{Autofix, Noqa};
use ruff::settings::options::Options;
use ruff::settings::Settings;

use crate::message::Message;

mod message;

fn into_pyerr(e: impl std::fmt::Display) -> PyErr {
    PyException::new_err(e.to_string())
}

#[pyfunction]
fn lint(
    content: &str,
    settings: &str,
    path: Option<&str>,
    noqa: Option<bool>,
    autofix: Option<bool>,
    package: Option<&str>,
) -> PyResult<Vec<Message>> {
    let path = path.map(Path::new).unwrap_or(Path::new("-"));
    let options: Options = toml::from_str(settings).map_err(into_pyerr)?;
    let configuration = Configuration::from_options(options, path).map_err(into_pyerr)?;
    let settings = Settings::from_configuration(configuration, path).map_err(into_pyerr)?;
    let noqa = noqa.map_or(Noqa::Enabled, |v| {
        if v {
            Noqa::Enabled
        } else {
            Noqa::Disabled
        }
    });
    let autofix = autofix.map_or(Autofix::Disabled, |v| {
        if v {
            Autofix::Enabled
        } else {
            Autofix::Disabled
        }
    });
    let package = package.map(Path::new);

    let LinterResult { data, error } = lint_only(
        content,
        path,
        package,
        &settings,
        noqa,
        autofix,
    );
    if let Some(error) = error {
        Err(into_pyerr(error))
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
