// src/lib.rs

//! # anonymize
//!
//! Deterministic text anonymization engine.



// Declare the error module
mod error;

pub use error::AnonymizeError;
pub type Result<T> = std::result::Result<T, AnonymizeError>;

pub mod normalizer;

pub use normalizer::{normalize, NormalizedText};