// src/lib.rs

//! # anonymize
//!
//! Deterministic text anonymization engine.

mod error;
mod normalizer;
mod detector;
mod conflict_resolver;
mod replacement_engine;
mod audit_report;
mod engine;
pub mod utils;
pub mod web;
pub mod document_processor; // New: document processing

pub use error::AnonymizeError;
pub use normalizer::{normalize, NormalizedText};
pub use detector::{
    Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult,
    EmailDetector, PhoneDetector, SpanishIdDetector, IbanDetector, CreditCardDetector,
    SsnDetector, ProjectCodeDetector, ContractNumberDetector, WorkOrderDetector,
    PurchaseOrderDetector, SerialNumberDetector, CostCenterDetector,
};
pub use engine::{Anonymizer, AnonymizationOutput};
pub use audit_report::AuditReport;

pub type Result<T> = std::result::Result<T, AnonymizeError>;
