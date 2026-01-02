// src/document_processor/docx.rs

use crate::{Anonymizer, Result, AnonymizeError};
use super::ProcessedDocument;
use docx_rs::*;
use std::io::Cursor;

/// Process DOCX file
pub fn process_docx(
    file_bytes: &[u8],
    original_filename: &str,
    anonymizer: &Anonymizer,
) -> Result<ProcessedDocument> {
    // Read DOCX document (Updated API: accepts &[u8] directly)
    let docx = read_docx(file_bytes)
        .map_err(|e| AnonymizeError::ConfigError {
            message: format!("Error reading DOCX: {}", e),
        })?;

    // Extract all text from the document
    let text = extract_text_from_docx(&docx);

    // Anonymize text with the existing engine
    let output = anonymizer.anonymize(&text)?;

    // Create new document with anonymized text
    let new_docx = create_anonymized_docx(&output.text);

    // Serialize to bytes (pack needs Write + Seek)
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    new_docx
        .build()
        .pack(&mut cursor)
        .map_err(|e| AnonymizeError::ConfigError {
            message: format!("Error creating DOCX: {}", e),
        })?;

    // Generate filename
    let new_filename = generate_output_filename(original_filename);

    Ok(ProcessedDocument {
        content: buffer,
        content_type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
        filename: new_filename,
        audit_report: output.report,
    })
}

/// Extract text from a DOCX document (simplified)
fn extract_text_from_docx(docx: &Docx) -> String {
    let mut text = String::new();
    
    for child in &docx.document.children {
        match child {
            DocumentChild::Paragraph(para) => {
                for run in &para.children {
                    if let ParagraphChild::Run(r) = run {
                        for run_child in &r.children {
                            if let RunChild::Text(t) = run_child {
                                text.push_str(&t.text);
                            }
                        }
                    }
                }
                text.push('\n');
            }
            // Simplified tables - only extract basic text
            DocumentChild::Table(_table) => {
                // Table structure is complex and varies by version
                // For now, add placeholder
                text.push_str("[TABLE]\n");
            }
            _ => {}
        }
    }
    
    text
}

/// Create DOCX document with anonymized text
fn create_anonymized_docx(anonymized_text: &str) -> Docx {
    let mut docx = Docx::new();
    
    // Split anonymized text into paragraphs
    for line in anonymized_text.lines() {
        if !line.trim().is_empty() {
            docx = docx.add_paragraph(
                Paragraph::new()
                    .add_run(Run::new().add_text(line))
            );
        } else {
            // Empty line = empty paragraph
            docx = docx.add_paragraph(Paragraph::new());
        }
    }
    
    docx
}

/// Generate output filename
fn generate_output_filename(original: &str) -> String {
    let stem = original
        .trim_end_matches(".docx")
        .trim_end_matches(".DOCX");
    format!("{}_anonymized.docx", stem)
}