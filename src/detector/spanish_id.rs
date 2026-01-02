// src/detector/spanish_id.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};
use crate::utils::checksum::validate_spanish_id;

pub struct SpanishIdDetector {
    national_id_regex: Regex,
    foreign_id_regex: Regex,
}

impl Default for SpanishIdDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl SpanishIdDetector {
    pub fn new() -> Self {
        Self {
            national_id_regex: Regex::new(r"\b[0-9]{8}[A-Z]\b")
                .expect("BUG: Spanish National ID regex is invalid"),
            foreign_id_regex: Regex::new(r"\b[XYZ][0-9]{7}[A-Z]\b")
                .expect("BUG: Spanish Foreigner ID regex is invalid"),
        }
    }
}

impl Detector for SpanishIdDetector {
    fn id(&self) -> DetectorId {
        "spanish_id".to_string()
    }
    
    fn category(&self) -> Category {
        Category::NationalId
    }
    
    fn detect(&self, text: &str) -> Vec<CandidateMatch> {
        let mut matches = Vec::new();
        
        for m in self.national_id_regex.find_iter(text) {
            let raw = m.as_str();
            let validation = self.validate(raw);
            
            let confidence = match validation {
                ValidationResult::Valid => Confidence::Verified,
                ValidationResult::Invalid => continue,
                ValidationResult::NotApplicable => Confidence::PatternOnly,
            };
            
            matches.push(CandidateMatch {
                span: Span {
                    start: m.start(),
                    end: m.end(),
                },
                detector_id: self.id(),
                category: self.category(),
                priority: self.priority(),
                confidence,
                raw_value: raw.to_string(),
                normalized_value: Some(normalize_spanish_id(raw)),
            });
        }
        
        for m in self.foreign_id_regex.find_iter(text) {
            let raw = m.as_str();
            let validation = self.validate(raw);
            
            let confidence = match validation {
                ValidationResult::Valid => Confidence::Verified,
                ValidationResult::Invalid => continue,
                ValidationResult::NotApplicable => Confidence::PatternOnly,
            };
            
            matches.push(CandidateMatch {
                span: Span {
                    start: m.start(),
                    end: m.end(),
                },
                detector_id: self.id(),
                category: self.category(),
                priority: self.priority(),
                confidence,
                raw_value: raw.to_string(),
                normalized_value: Some(normalize_spanish_id(raw)),
            });
        }
        
        matches
    }

    fn validate(&self, candidate: &str) -> ValidationResult {
        if validate_spanish_id(candidate) {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid
        }
    }

    fn priority(&self) -> u32 {
        100
    }
}

fn normalize_spanish_id(id: &str) -> String {
    id.to_uppercase()
}
