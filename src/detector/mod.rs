// src/detector/mod.rs

mod email;
pub use email::EmailDetector;

/// Byte span in the original text
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// Category of detected sensitive data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    // Personal identifiers
    Email,
    Phone,
    Iban,
    NationalId,
    CreditCard,
    BankAccount,
    
    // Industrial/Corporate identifiers
    ProjectCode,
    ProjectNumber,
    ContractNumber,
    WorkOrder,
    PurchaseOrder,
    SerialNumber,
    CostCenter,
    
    // Dictionary-based entities
    CompanyName,
    ProjectName,
    PersonnelName,
    ClientName,
    
    // Document metadata
    DocumentNumber,
    RevisedBy,
    ApprovedBy,
    DesignedBy,
}

/// A candidate match found by a detector
#[derive(Debug, Clone)]
pub struct CandidateMatch {
    pub span: Span,
    pub value: String,
    pub category: Category,
}

/// Trait that all detectors must implement
pub trait Detector {
    /// Unique identifier for this detector
    fn id(&self) -> &str;
    
    /// Category of data this detector finds
    fn category(&self) -> Category;
    
    /// Find all matches in the text
    fn detect(&self, text: &str) -> Vec<CandidateMatch>;
}