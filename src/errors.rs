use std::io;
use std::path::PathBuf;

use thiserror::Error;

/// An error that may occur when parsing apt sources.
#[derive(Debug, Error)]
pub enum SourceError {
    #[error("I/O error occurred: {0}")]
    Io(io::Error),
    #[error("missing field in apt source list: '{field}'")]
    MissingField { field: &'static str },
    #[error("invalid field in apt source list: '{value}' is invalid for '{field}'")]
    InvalidValue { field: &'static str, value: String },
    #[error("entry did not exist in sources")]
    EntryNotFound,
    #[error("failed to write changes to {:?}: {}", path, why)]
    EntryWrite { path: PathBuf, why: io::Error },
    #[error("source file was not found")]
    FileNotFound,
    #[error("failed to parse source list at {:?}: {}", path, why)]
    SourcesList {
        path: PathBuf,
        why: Box<SourcesListError>,
    },
    #[error("failed to open / read source list at {:?}: {}", path, why)]
    SourcesListOpen { path: PathBuf, why: io::Error },
    #[error("Syntax Error: {}", why)]
    SyntaxError { why: String },
}

#[derive(Debug, Error)]
pub enum SourcesListError {
    #[error("parsing error on line {}: {}", line, why)]
    BadLine { line: usize, why: SourceError },
}

impl From<io::Error> for SourceError {
    fn from(why: io::Error) -> Self {
        SourceError::Io(why)
    }
}

/// Equivalent to `Result<T, SourceError>`.
pub type SourceResult<T> = Result<T, SourceError>;
