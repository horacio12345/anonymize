// src/document_processor/pdf.rs

use crate::{Anonymizer, Result, AnonymizeError};
use super::ProcessedDocument;
use lopdf::{Document, Object, Stream, dictionary};
use pdf_extract::extract_text_from_mem;

/// Procesar archivo PDF
pub fn process_pdf(
    file_bytes: &[u8],
    original_filename: &str,
    anonymizer: &Anonymizer,
) -> Result<ProcessedDocument> {
    // Extraer texto del PDF
    let text = extract_text_from_mem(file_bytes)
        .map_err(|e| AnonymizeError::ConfigError {
            message: format!("Error al extraer texto del PDF: {}", e),
        })?;

    // Anonimizar texto con el motor existente
    let output = anonymizer.anonymize(&text)?;

    // Crear nuevo PDF con texto anonimizado
    let pdf_bytes = create_anonymized_pdf(&output.text)?;

    // Generar nombre de archivo
    let new_filename = generate_output_filename(original_filename);

    Ok(ProcessedDocument {
        content: pdf_bytes,
        content_type: "application/pdf".to_string(),
        filename: new_filename,
    })
}

/// Crear PDF simple con texto anonimizado
fn create_anonymized_pdf(text: &str) -> Result<Vec<u8>> {
    let mut doc = Document::with_version("1.5");
    
    // Crear catálogo
    let catalog_id = doc.new_object_id();
    let pages_id = doc.new_object_id();
    
    // Catálogo
    let catalog = dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    };
    doc.objects.insert(catalog_id, Object::Dictionary(catalog));
    
    // Páginas
    let mut page_ids = Vec::new();
    
    // Dividir texto en páginas (aproximadamente 50 líneas por página)
    let lines: Vec<&str> = text.lines().collect();
    let lines_per_page = 50;
    
    for (page_num, chunk) in lines.chunks(lines_per_page).enumerate() {
        let page_id = doc.new_object_id();
        page_ids.push(page_id);
        
        // Crear contenido de la página
        let content = create_page_content(chunk, page_num);
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.into_bytes()));
        
        // Crear página
        let page = dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()], // A4
            "Resources" => dictionary! {
                "Font" => dictionary! {
                    "F1" => dictionary! {
                        "Type" => "Font",
                        "Subtype" => "Type1",
                        "BaseFont" => "Courier",
                    },
                },
            },
        };
        doc.objects.insert(page_id, Object::Dictionary(page));
    }
    
    // Objeto Pages
    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => page_ids.iter().map(|&id| Object::Reference(id)).collect::<Vec<_>>(),
        "Count" => page_ids.len() as i64,
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    
    // Establecer catálogo como root
    doc.trailer.set("Root", catalog_id);
    
    // Serializar PDF
    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| AnonymizeError::ConfigError {
            message: format!("Error al crear PDF: {}", e),
        })?;
    
    Ok(buffer)
}

/// Crear contenido de una página PDF
fn create_page_content(lines: &[&str], _page_num: usize) -> String {
    let mut content = String::new();
    content.push_str("BT\n");
    content.push_str("/F1 10 Tf\n");
    content.push_str("50 800 Td\n"); // Posición inicial
    content.push_str("12 TL\n"); // Leading (espaciado entre líneas)
    
    for line in lines {
        // Escapar caracteres especiales en PDF
        let escaped = escape_pdf_text(line);
        content.push_str(&format!("({}) Tj T*\n", escaped));
    }
    
    content.push_str("ET\n");
    content
}

/// Escapar texto para PDF
fn escape_pdf_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace('\r', "")
}

/// Generar nombre de archivo de salida
fn generate_output_filename(original: &str) -> String {
    let stem = original
        .trim_end_matches(".pdf")
        .trim_end_matches(".PDF");
    format!("{}_anonymized.pdf", stem)
}