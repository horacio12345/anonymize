// src/detector/contract_number.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};

/// Detector for contract numbers
pub struct ContractNumberDetector {
    regex: Regex,
}

impl ContractNumberDetector {
    pub fn new() -> Self {
        Self {
            // Formato: CTR-XXXX-XXXX o CONT-XXXX-XXXX
            regex: Regex::new(r"\b(?:CTR|CONT|CONTRACT)-[0-9]{4}-[0-9]{4,8}\b")
                .expect("BUG: Contract number regex is invalid"),
        }
    }
}

impl Detector for ContractNumberDetector {
    fn id(&self) -> DetectorId {
        "contract_number".to_string()
    }
    
    fn category(&self) -> Category {
        Category::ContractNumber
    }
    
    fn detect(&self, text: &str) -> Vec<CandidateMatch> {
        self.regex
            .find_iter(text)
            .map(|m| CandidateMatch {
                span: Span {
                    start: m.start(),
                    end: m.end(),
                },
                detector_id: self.id(),
                category: self.category(),
                priority: self.priority(),
                confidence: Confidence::PatternOnly,
                raw_value: m.as_str().to_string(),
                normalized_value: None,
            })
            .collect()
    }

    fn validate(&self, _candidate: &str) -> ValidationResult {
        ValidationResult::NotApplicable
    }

    fn priority(&self) -> u32 {
        70
    }
}