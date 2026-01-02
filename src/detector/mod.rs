// src/detector/mod.rs

mod email;
mod phone;
mod spanish_id;
mod iban;
mod credit_card;
mod ssn;
mod project_code;
mod contract_number;
mod work_order;
mod purchase_order;
mod serial_number;
mod cost_center;

pub use email::EmailDetector;
pub use phone::PhoneDetector;
pub use spanish_id::SpanishIdDetector;
pub use iban::IbanDetector;
pub use credit_card::CreditCardDetector;
pub use ssn::SsnDetector;
pub use project_code::ProjectCodeDetector;
pub use contract_number::ContractNumberDetector;
pub use work_order::WorkOrderDetector;
pub use purchase_order::PurchaseOrderDetector;
pub use serial_number::SerialNumberDetector;
pub use cost_center::CostCenterDetector;

use serde::{Serialize, Deserialize};

/// Unique identifier for a detector
pub type DetectorId = String;

/// Byte span in the original text
#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// Category of detected sensitive data
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    Email,
    Phone,
    Iban,
    NationalId,
    CreditCard,
    ProjectCode,
    ContractNumber,
    WorkOrder,
    PurchaseOrder,
    SerialNumber,
    CostCenter,
    CompanyName,
    ProjectName,
    PersonnelName,
    ClientName,
    Custom(String),
    DocumentNumber,
    RevisedBy,
    ApprovedBy,
    DesignedBy,
}

/// Confidence level of a match
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Confidence {
    Verified,
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
    fn id(&self) -> DetectorId;
    fn category(&self) -> Category;
    fn detect(&self, text: &str) -> Vec<CandidateMatch>;
    fn validate(&self, candidate: &str) -> ValidationResult;
    fn priority(&self) -> u32;
}
