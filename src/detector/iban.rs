// src/detector/iban.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};
use crate::utils::checksum::validate_iban;

pub struct IbanDetector {
    regex: Regex,
}

impl Default for IbanDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl IbanDetector {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(r"\b[A-Z]{2}[0-9]{2}[A-Z0-9\s-]{15,30}\b")
                .expect("BUG: IBAN regex is invalid"),
        }
    }
}

impl Detector for IbanDetector {
    fn id(&self) -> DetectorId {
        "iban".to_string()
    }
    
    fn category(&self) -> Category {
        Category::Iban
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
                        normalized_value: Some(normalize_iban(raw)),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn validate(&self, candidate: &str) -> ValidationResult {
        if validate_iban(candidate) {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid
        }
    }

    fn priority(&self) -> u32 {
        100
    }
}

fn normalize_iban(iban: &str) -> String {
    iban.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_uppercase()
}
