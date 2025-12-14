// src/error.rs

use thiserror::Error;

/// Errors that can occur during anonymization
#[derive(Debug, Error)]
pub enum AnonymizeError {
    /// Invalid UTF-8 encoding in input
    #[error("Invalid UTF-8 at byte position {position}")]
    InvalidUtf8 {
        position: usize,
    },

    /// Input exceeds maximum allowed size
    #[error("Input exceeds maximum size: {size} bytes > {max} bytes")]
    InputTooLarge {
        size: usize,
        max: usize,
    },

    /// Configuration error
    #[error("Configuration error: {message}")]
    ConfigError {
        message: String,
    },

    /// Invalid regex pattern in detector
    #[error("Invalid regex pattern in detector '{detector}': {message}")]
    InvalidPattern {
        detector: String,
        message: String,
    },

    /// IO error (file read/write)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}