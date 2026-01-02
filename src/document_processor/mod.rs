// src/document_processor/mod.rs

pub mod docx;

use crate::{Result, AuditReport};

/// Detected document type
#[derive(Debug, Clone, Copy)]
pub enum DocumentType {
    Docx,
}

impl DocumentType {
    /// Detect type by extension
    pub fn from_filename(filename: &str) -> Option<Self> {
        let lower = filename.to_lowercase();
        if lower.ends_with(".docx") {
            Some(DocumentType::Docx)
        } else {
            None 
        }
    }
}

/// Result of document processing
pub struct ProcessedDocument {
    pub content: Vec<u8>,
    pub content_type: String,
    pub filename: String,
    pub audit_report: AuditReport,
}

/// Process document according to type
pub fn process_document(
    file_bytes: &[u8],
    filename: &str,
    anonymizer: &crate::Anonymizer,
) -> Result<ProcessedDocument> {
    let doc_type = DocumentType::from_filename(filename)
        .ok_or_else(|| crate::AnonymizeError::ConfigError {
            message: format!("Unsupported file type: {}", filename),
        })?;

    match doc_type {
        DocumentType::Docx => docx::process_docx(file_bytes, filename, anonymizer),
    }
}