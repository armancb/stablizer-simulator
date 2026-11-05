//! Error type shared by the whole crate.

use std::fmt;

/// `Index` is an out-of-range qubit or row; `Value` is any other invalid input
/// or state. The Python bindings map them to `IndexError` and `ValueError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Index(String),
    Value(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Index(m) | Error::Value(m) => f.write_str(m),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn value_err<T>(msg: impl Into<String>) -> Result<T> {
    Err(Error::Value(msg.into()))
}

pub(crate) fn index_err<T>(msg: impl Into<String>) -> Result<T> {
    Err(Error::Index(msg.into()))
}
