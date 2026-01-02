// src/web/handlers.rs

use axum::{
    extract::{Json, Multipart, Form},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use crate::{
    Anonymizer, 
    EmailDetector, PhoneDetector, SpanishIdDetector, IbanDetector,
    CreditCardDetector, SsnDetector, ProjectCodeDetector, ContractNumberDetector,
    WorkOrderDetector, PurchaseOrderDetector, SerialNumberDetector,
    CostCenterDetector,
    AuditReport,
    document_processor,
};

/// Request payload for the /anonymize endpoint
#[derive(Deserialize)]
pub struct AnonymizeRequest {
    pub text: String,
}

/// Response payload for the /anonymize endpoint
#[derive(Serialize)]
pub struct AnonymizeResponse {
    pub anonymized_text: String,
    pub audit_report: AuditReport,
    pub hash: String,
}

/// Response payload for the /anonymize-file endpoint
#[derive(Serialize)]
pub struct AnonymizeFileResponse {
    pub file_base64: String,
    pub filename: String,
    pub audit_report: AuditReport,
}

/// Custom error for HTTP responses
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

/// Create anonymization engine with all detectors
fn create_anonymizer() -> Anonymizer {
    let mut engine = Anonymizer::new();
    
    // Personal data detectors
    engine.add_detector(Box::new(EmailDetector::new()));
    engine.add_detector(Box::new(PhoneDetector::new()));
    engine.add_detector(Box::new(SpanishIdDetector::new()));
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

/// Handler for plain text anonymization
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

/// Handler for file anonymization (returns JSON with base64)
pub async fn anonymize_file_handler(
    mut multipart: Multipart,
) -> Result<Json<AnonymizeFileResponse>, AppError> {
    // Extract file from multipart
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    
    while let Some(field) = multipart.next_field().await
        .map_err(|e| AppError(format!("Error reading multipart: {}", e)))? 
    {
        let field_name = field.name().unwrap_or("").to_string();
        
        if field_name == "file" {
            filename = field.file_name().map(|s| s.to_string());
            file_bytes = Some(field.bytes().await
                .map_err(|e| AppError(format!("Error reading bytes: {}", e)))?
                .to_vec());
        }
    }
    
    // Validate that we have a file
    let file_bytes = file_bytes.ok_or_else(|| AppError("No file received".to_string()))?;
    let filename = filename.ok_or_else(|| AppError("No filename received".to_string()))?;
    
    // Validate file size (max 10MB)
    if file_bytes.len() > 10 * 1024 * 1024 {
        return Err(AppError("File too large (max 10MB)".to_string()));
    }
    
    // Create anonymization engine
    let engine = create_anonymizer();
    
    // Process document
    let processed = document_processor::process_document(&file_bytes, &filename, &engine)?;
    
    // IMPORTANT: Validate that processed content is not empty
    if processed.content.is_empty() {
        return Err(AppError("Processed document is empty".to_string()));
    }
    
    // Convert content to base64 using the base64 crate
    let file_base64 = base64_encode(&processed.content);
    
    // Validate that base64 is not empty
    if file_base64.is_empty() {
        return Err(AppError("Error generating base64".to_string()));
    }
    
    // Debugging log (remove in production if necessary)
    eprintln!("✓ File processed: {} bytes -> {} base64 characters", 
              processed.content.len(), file_base64.len());
    
    // Create response with full audit report
    let response = AnonymizeFileResponse {
        file_base64,
        filename: processed.filename,
        audit_report: processed.audit_report,
    };
    
    Ok(Json(response))
}

/// Encode bytes to base64 string using base64 crate
/// Guarantees no spaces or line breaks
fn base64_encode(bytes: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};
    
    // STANDARD produces clean base64 without spaces or line breaks
    general_purpose::STANDARD.encode(bytes)
}