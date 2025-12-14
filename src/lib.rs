// src/lib.rs

//! # anonymize
//!
//! Deterministic text anonymization engine.
//!
//! This library provides formal, rule-based anonymization of sensitive data
//! in text documents. No AI, no heuristics, no guessing.

// Declare modules
mod error;
mod normalizer;
mod detector;

// Re-export public API
pub use error::AnonymizeError;
pub use normalizer::{normalize, NormalizedText};
pub use detector::{Detector, CandidateMatch, Category, Span, EmailDetector};

// Custom Result type for this crate
pub type Result<T> = std::result::Result<T, AnonymizeError>;