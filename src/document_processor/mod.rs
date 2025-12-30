// src/document_processor/mod.rs

pub mod docx;

use crate::Result;

/// Tipo de documento detectado
#[derive(Debug, Clone, Copy)]
pub enum DocumentType {
    Docx,
}

impl DocumentType {
    /// Detectar tipo por extensión
    pub fn from_filename(filename: &str) -> Option<Self> {
        let lower = filename.to_lowercase();
        if lower.ends_with(".docx") {
            Some(DocumentType::Docx)
        } else {
            None 
        }
    }
}

/// Resultado del procesamiento de un documento
pub struct ProcessedDocument {
    pub content: Vec<u8>,
    pub content_type: String,
    pub filename: String,
}

/// Procesar documento según tipo
pub fn process_document(
    file_bytes: &[u8],
    filename: &str,
    anonymizer: &crate::Anonymizer,
) -> Result<ProcessedDocument> {
    let doc_type = DocumentType::from_filename(filename)
        .ok_or_else(|| crate::AnonymizeError::ConfigError {
            message: format!("Tipo de archivo no soportado: {}", filename),
        })?;

    match doc_type {
        DocumentType::Docx => docx::process_docx(file_bytes, filename, anonymizer),
    }
}