// src/replacement_engine.rs

use crate::detector::{CandidateMatch, Category, Span, DetectorId, Confidence};
use std::collections::HashMap;

/// Result of a substitution process
pub struct ReplacementResult {
    pub anonymized_text: String,
    pub replacements: Vec<Replacement>,
    pub original_length: usize,
    pub anonymized_length: usize,
}

/// Metadata about a single replacement
pub struct Replacement {
    pub span: Span,
    pub original: String,
    pub placeholder: String,
    pub category: Category,
    pub detector_id: DetectorId,
    pub confidence: Confidence,
}

pub struct ReplacementEngine;

impl ReplacementEngine {
    /// Perform text substitution using a sequential numbering strategy.
    pub fn replace(text: &str, matches: Vec<CandidateMatch>) -> ReplacementResult {
        let mut replacements = Vec::new();
        let mut category_counters: HashMap<Category, usize> = HashMap::new();
        
        let mut sorted_matches = matches;
        sorted_matches.sort_by_key(|m| m.span.start);
        
        let mut match_data = Vec::new();
        for m in sorted_matches {
            let counter = category_counters.entry(m.category.clone()).or_insert(0);
            *counter += 1;
            
            let cat_name = match &m.category {
                Category::Email => "EMAIL".to_string(),
                Category::Phone => "PHONE".to_string(),
                Category::Iban => "IBAN".to_string(),
                Category::NationalId => "NATIONAL_ID".to_string(),
                Category::CreditCard => "CREDIT_CARD".to_string(),
                Category::ProjectCode => "PROJECT_CODE".to_string(),
                Category::ContractNumber => "CONTRACT_NUMBER".to_string(),
                Category::WorkOrder => "WORK_ORDER".to_string(),
                Category::PurchaseOrder => "PURCHASE_ORDER".to_string(),
                Category::SerialNumber => "SERIAL_NUMBER".to_string(),
                Category::CostCenter => "COST_CENTER".to_string(),
                Category::CompanyName => "COMPANY_NAME".to_string(),
                Category::ProjectName => "PROJECT_NAME".to_string(),
                Category::PersonnelName => "PERSONNEL_NAME".to_string(),
                Category::ClientName => "CLIENT_NAME".to_string(),
                Category::Custom(s) => s.to_uppercase(),
                Category::DocumentNumber => "DOCUMENT_NUMBER".to_string(),
                Category::RevisedBy => "REVISED_BY".to_string(),
                Category::ApprovedBy => "APPROVED_BY".to_string(),
                Category::DesignedBy => "DESIGNED_BY".to_string(),
            };
            
            let placeholder = format!("[{}_{}]", cat_name, counter);
            match_data.push((m, placeholder));
        }
        
        let mut anonymized_text = text.to_string();
        let original_length = text.len();
        
        // Reverse order replacement
        for (m, placeholder) in match_data.into_iter().rev() {
            let replacement = Replacement {
                span: m.span,
                original: m.raw_value.clone(),
                placeholder: placeholder.clone(),
                category: m.category,
                detector_id: m.detector_id,
                confidence: m.confidence,
            };
            
            anonymized_text.replace_range(m.span.start..m.span.end, &placeholder);
            replacements.push(replacement);
        }
        
        // The replacements list in the result should be in order of appearance in the original text (for audit)
        replacements.reverse();
        
        let anonymized_length = anonymized_text.len();
        
        ReplacementResult {
            anonymized_text,
            replacements,
            original_length,
            anonymized_length,
        }
    }
}
