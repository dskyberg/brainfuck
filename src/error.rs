//! Crate errors
use thiserror::Error;

/// Shorthand for standard Result
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Crate errors
#[derive(Error, Debug, PartialEq, PartialOrd)]
pub enum BrainFuckError {
    #[error("Bad token: {0}")]
    BadToken(char),
    #[error("Data is out of range {0}")]
    DataOutOfRange(usize),
    #[error("Data point is out of range")]
    DataPointerOutOfRange,
    #[error("Compile error")]
    FailedToCompile,
    #[error("Builder error: {0}")]
    InterpreterBuildError(String),
    #[error("Bad args. Use --help.")]
    BadArgs,
}
