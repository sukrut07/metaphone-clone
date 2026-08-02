//! Custom error types for the Metaphone library.

use std::fmt;

/// Errors that can occur during Metaphone encoding or configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaphoneError {
    /// The specified maximum code length is invalid (e.g. 0).
    InvalidMaxCodeLength,
}

impl fmt::Display for MetaphoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMaxCodeLength => write!(f, "maximum code length must be greater than 0"),
        }
    }
}

impl std::error::Error for MetaphoneError {}
