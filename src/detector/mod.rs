// src/detector/mod.rs

mod email;
mod phone;
mod dni;

pub use email::EmailDetector;
pub use phone::PhoneDetector;
pub use dni::DniDetector;


use serde::{Serialize, Deserialize};

/// Unique identifier for a detector
pub type DetectorId = String;


/// Byte span in the original text
#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub struct Span {
    pub start: usize, // Byte offset, inclusive
    pub end: usize,   // Byte offset, exclusive
}

/// Category of detected sensitive data
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    // Personal
    Email,
    Phone,
    Iban,
    NationalId,
    CreditCard,
    // Industrial/Corporate
    ProjectCode,
    ContractNumber,
    WorkOrder,
    PurchaseOrder,
    SerialNumber,
    CostCenter,
    // Dictionary-based
    CompanyName,
    ProjectName,
    PersonnelName,
    ClientName,
    // User-defined
    Custom(String),
    // Document metadata
    DocumentNumber,
    RevisedBy,
    ApprovedBy,
    DesignedBy,
}

/// Confidence level of a match
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Confidence {
    /// Pattern + validation passed (e.g., IBAN with valid checksum)
    Verified,
    /// Pattern matched, no validation available or applicable
    PatternOnly,
}

/// Result of a validation check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationResult {
    Valid,
    Invalid,
    NotApplicable,
}

/// A candidate match found by a detector
#[derive(Debug, Clone)]
pub struct CandidateMatch {
    pub span: Span,
    pub detector_id: DetectorId,
    pub category: Category,
    pub priority: u32,
    pub confidence: Confidence,
    pub raw_value: String,
    pub normalized_value: Option<String>,
}

/// Trait that all detectors must implement
pub trait Detector: Send + Sync {
    /// Unique identifier for this detector type
    fn id(&self) -> DetectorId;

    /// Human-readable category for audit reports
    fn category(&self) -> Category;

    /// Find all candidate matches in the text
    fn detect(&self, text: &str) -> Vec<CandidateMatch>;

    /// Validate a candidate (e.g., checksum verification)
    /// Returns NotApplicable if validation is not applicable
    fn validate(&self, candidate: &str) -> ValidationResult;

    /// Priority for conflict resolution (higher = preferred)
    fn priority(&self) -> u32;
}