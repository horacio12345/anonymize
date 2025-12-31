// src/web/handlers.rs

use axum::{
    extract::{Json, Multipart, Form},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use crate::{
    Anonymizer, 
    EmailDetector, PhoneDetector, DniDetector, IbanDetector,
    CreditCardDetector, SsnDetector, ProjectCodeDetector, ContractNumberDetector,
    WorkOrderDetector, PurchaseOrderDetector, SerialNumberDetector,
    CostCenterDetector,
    AuditReport,
    document_processor,
};

/// Request payload para el endpoint /anonymize
#[derive(Deserialize)]
pub struct AnonymizeRequest {
    pub text: String,
}

/// Response payload del endpoint /anonymize
#[derive(Serialize)]
pub struct AnonymizeResponse {
    pub anonymized_text: String,
    pub audit_report: AuditReport,
    pub hash: String,
}

/// Response payload del endpoint /anonymize-file
#[derive(Serialize)]
pub struct AnonymizeFileResponse {
    pub file_base64: String,
    pub filename: String,
    pub statistics: FileStatistics,
}

/// Estadísticas simplificadas para archivos
#[derive(Serialize)]
pub struct FileStatistics {
    pub total_detections: usize,
    pub processing_time_ms: u64,
}

/// Error personalizado para respuestas HTTP
#[derive(Debug)]
pub struct AppError(String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": self.0
            }))
        ).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: std::error::Error,
{
    fn from(err: E) -> Self {
        AppError(err.to_string())
    }
}

/// Crear motor de anonimización con todos los detectores
fn create_anonymizer() -> Anonymizer {
    let mut engine = Anonymizer::new();
    
    // Personal data detectors
    engine.add_detector(Box::new(EmailDetector::new()));
    engine.add_detector(Box::new(PhoneDetector::new()));
    engine.add_detector(Box::new(DniDetector::new()));
    engine.add_detector(Box::new(IbanDetector::new()));
    engine.add_detector(Box::new(CreditCardDetector::new()));
    engine.add_detector(Box::new(SsnDetector::new()));
    
    // Corporate/Industrial detectors
    engine.add_detector(Box::new(ProjectCodeDetector::new()));
    engine.add_detector(Box::new(ContractNumberDetector::new()));
    engine.add_detector(Box::new(WorkOrderDetector::new()));
    engine.add_detector(Box::new(PurchaseOrderDetector::new()));
    engine.add_detector(Box::new(SerialNumberDetector::new()));
    engine.add_detector(Box::new(CostCenterDetector::new()));
    
    engine
}

/// Handler para anonimización de texto plano
pub async fn anonymize_handler(
    Form(payload): Form<AnonymizeRequest>,
) -> Result<Json<AnonymizeResponse>, AppError> {
    let engine = create_anonymizer();
    let output = engine.anonymize(&payload.text)?;

    let response = AnonymizeResponse {
        anonymized_text: output.text,
        audit_report: output.report,
        hash: output.hash.value,
    };

    Ok(Json(response))
}

/// Handler para anonimización de archivos (devuelve JSON con base64)
pub async fn anonymize_file_handler(
    mut multipart: Multipart,
) -> Result<Json<AnonymizeFileResponse>, AppError> {
    use std::time::Instant;
    
    let start = Instant::now();
    
    // Extraer archivo del multipart
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    
    while let Some(field) = multipart.next_field().await
        .map_err(|e| AppError(format!("Error al leer multipart: {}", e)))? 
    {
        let field_name = field.name().unwrap_or("").to_string();
        
        if field_name == "file" {
            filename = field.file_name().map(|s| s.to_string());
            file_bytes = Some(field.bytes().await
                .map_err(|e| AppError(format!("Error al leer bytes: {}", e)))?
                .to_vec());
        }
    }
    
    // Validar que tenemos archivo
    let file_bytes = file_bytes.ok_or_else(|| AppError("No se recibió archivo".to_string()))?;
    let filename = filename.ok_or_else(|| AppError("No se recibió nombre de archivo".to_string()))?;
    
    // Crear motor de anonimización
    let engine = create_anonymizer();
    
    // Procesar documento
    let processed = document_processor::process_document(&file_bytes, &filename, &engine)?;
    
    // Calcular tiempo de procesamiento
    let processing_time = start.elapsed().as_millis() as u64;
    
    // Convertir contenido a base64
    let file_base64 = base64_encode(&processed.content);
    
    // Crear respuesta con estadísticas
    let response = AnonymizeFileResponse {
        file_base64,
        filename: processed.filename,
        statistics: FileStatistics {
            total_detections: 0,  // TODO: Extraer del audit report si está disponible
            processing_time_ms: processing_time,
        },
    };
    
    Ok(Json(response))
}

/// Encode bytes to base64 string
fn base64_encode(bytes: &[u8]) -> String {
    use std::fmt::Write;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut result = String::with_capacity((bytes.len() + 2) / 3 * 4);
    let mut i = 0;
    
    while i + 2 < bytes.len() {
        let b1 = bytes[i];
        let b2 = bytes[i + 1];
        let b3 = bytes[i + 2];
        
        let _ = write!(result, "{}{}{}{}",
            CHARSET[(b1 >> 2) as usize] as char,
            CHARSET[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char,
            CHARSET[(((b2 & 0x0F) << 2) | (b3 >> 6)) as usize] as char,
            CHARSET[(b3 & 0x3F) as usize] as char,
        );
        
        i += 3;
    }
    
    // Handle remaining bytes
    if i < bytes.len() {
        let b1 = bytes[i];
        let _ = write!(result, "{}{}", 
            CHARSET[(b1 >> 2) as usize] as char,
            CHARSET[((b1 & 0x03) << 4) as usize] as char,
        );
        
        if i + 1 < bytes.len() {
            let b2 = bytes[i + 1];
            let _ = write!(result, "{}",
                CHARSET[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char,
            );
            let _ = write!(result, "{}",
                CHARSET[((b2 & 0x0F) << 2) as usize] as char,
            );
        } else {
            result.push('=');
        }
        result.push('=');
    }
    
    result
}