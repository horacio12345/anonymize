// src/detector/credit_card.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};
use crate::utils::checksum::validate_luhn;

/// Detector for credit card numbers
pub struct CreditCardDetector {
    regex: Regex,
}

impl CreditCardDetector {
    pub fn new() -> Self {
        Self {
            // 13-19 dígitos con separadores opcionales (-, espacio)
            regex: Regex::new(r"\b[0-9]{4}[-\s]?[0-9]{4}[-\s]?[0-9]{4}[-\s]?[0-9]{4,7}\b")
                .expect("BUG: Credit card regex is invalid"),
        }
    }
}

impl Detector for CreditCardDetector {
    fn id(&self) -> DetectorId {
        "credit_card".to_string()
    }
    
    fn category(&self) -> Category {
        Category::CreditCard
    }
    
    fn detect(&self, text: &str) -> Vec<CandidateMatch> {
        self.regex
            .find_iter(text)
            .filter_map(|m| {
                let raw = m.as_str();
                let validation = self.validate(raw);
                
                if validation == ValidationResult::Valid {
                    Some(CandidateMatch {
                        span: Span {
                            start: m.start(),
                            end: m.end(),
                        },
                        detector_id: self.id(),
                        category: self.category(),
                        priority: self.priority(),
                        confidence: Confidence::Verified,
                        raw_value: raw.to_string(),
                        normalized_value: Some(normalize_card(raw)),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn validate(&self, candidate: &str) -> ValidationResult {
        if validate_luhn(candidate) {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid
        }
    }

    fn priority(&self) -> u32 {
        90
    }
}

fn normalize_card(card: &str) -> String {
    card.chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
}