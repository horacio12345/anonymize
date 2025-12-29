// src/document_processor/docx.rs

use crate::{Anonymizer, Result, AnonymizeError};
use super::ProcessedDocument;
use docx_rs::*;

/// Procesar archivo DOCX
pub fn process_docx(
    file_bytes: &[u8],
    original_filename: &str,
    anonymizer: &Anonymizer,
) -> Result<ProcessedDocument> {
    let docx = read_docx(file_bytes)?;
    let text = extract_text_from_docx(&docx);
    let output = anonymizer.anonymize(&text)?;
    let new_docx = create_anonymized_docx(&output.text);

    // FIX: Usar Cursor para implementar Seek
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    new_docx
        .build()
        .pack(&mut cursor)?;

    Ok(ProcessedDocument {
        content: buffer,  // El Vec tiene los datos escritos por Cursor
        content_type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
        filename: generate_output_filename(original_filename),
    })
}

/// Extraer texto de un documento DOCX (simplificado)
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
            // Tablas simplificadas - solo extraer texto básico
            DocumentChild::Table(_table) => {
                // La estructura de tablas es compleja y varía según versión
                // Por ahora, añadimos placeholder
                text.push_str("[TABLA]\n");
            }
            _ => {}
        }
    }
    
    text
}

/// Crear documento DOCX con texto anonimizado
fn create_anonymized_docx(anonymized_text: &str) -> Docx {
    let mut docx = Docx::new();
    
    // Dividir el texto anonimizado en párrafos
    for line in anonymized_text.lines() {
        if !line.trim().is_empty() {
            docx = docx.add_paragraph(
                Paragraph::new()
                    .add_run(Run::new().add_text(line))
            );
        } else {
            // Línea vacía = párrafo vacío
            docx = docx.add_paragraph(Paragraph::new());
        }
    }
    
    docx
}

/// Generar nombre de archivo de salida
fn generate_output_filename(original: &str) -> String {
    let stem = original
        .trim_end_matches(".docx")
        .trim_end_matches(".DOCX");
    format!("{}_anonymized.docx", stem)
}