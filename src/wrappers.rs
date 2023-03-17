use pyo3::prelude::*;
use std::fmt::{Display, Formatter};

#[pyclass]
pub struct Message(ruff::message::Message);

#[pymethods]
impl Message {
    #[getter]
    fn kind(&self) -> DiagnosticKind {
        self.0.kind.clone().into()
    }

    #[getter]
    fn location(&self) -> Location {
        self.0.location.into()
    }

    #[getter]
    fn end_location(&self) -> Location {
        self.0.end_location.into()
    }

    #[getter]
    fn fix(&self) -> Option<Fix> {
        self.0.fix.clone().map(Fix)
    }

    #[getter]
    fn filename(&self) -> String {
        self.0.filename.clone()
    }

    #[getter]
    fn source(&self) -> Option<Source> {
        self.0.source.clone().map(Source)
    }

    #[getter]
    fn noqa_row(&self) -> usize {
        self.0.noqa_row
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }
}

impl From<ruff::message::Message> for Message {
    fn from(message: ruff::message::Message) -> Self {
        Self(message)
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct DiagnosticKind(ruff_diagnostics::DiagnosticKind);

#[pymethods]
impl DiagnosticKind {
    #[getter]
    fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    fn body(&self) -> String {
        self.0.body.clone()
    }

    #[getter]
    fn suggestion(&self) -> Option<String> {
        self.0.suggestion.clone()
    }

    #[getter]
    fn fixable(&self) -> bool {
        self.0.fixable
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }
}

impl From<ruff_diagnostics::DiagnosticKind> for DiagnosticKind {
    fn from(kind: ruff_diagnostics::DiagnosticKind) -> Self {
        Self(kind)
    }
}

impl Display for DiagnosticKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[pyclass]
pub struct Location(ruff::message::Location);

#[pymethods]
impl Location {
    #[getter]
    fn row(&self) -> usize {
        self.0.row()
    }

    #[getter]
    fn column(&self) -> usize {
        self.0.column()
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }
}

impl From<ruff::message::Location> for Location {
    fn from(location: ruff::message::Location) -> Self {
        Self(location)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[pyclass]
pub struct Fix(ruff_diagnostics::Fix);

#[pymethods]
impl Fix {
    #[getter]
    fn content(&self) -> String {
        self.0.content.clone()
    }

    #[getter]
    fn location(&self) -> Location {
        self.0.location.into()
    }

    #[getter]
    fn end_location(&self) -> Location {
        self.0.end_location.into()
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }
}

impl Display for Fix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[pyclass]
pub struct Source(ruff::message::Source);

#[pymethods]
impl Source {
    #[getter]
    fn contents(&self) -> String {
        self.0.contents.clone()
    }

    #[getter]
    fn range(&self) -> (usize, usize) {
        self.0.range
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
