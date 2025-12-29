// src/detector/dni.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};
use crate::utils::checksum::validate_dni;

pub struct DniDetector {
    dni_regex: Regex,
    nie_regex: Regex,
}

impl DniDetector {
    pub fn new() -> Self {
        Self {
            dni_regex: Regex::new(r"\b[0-9]{8}[A-Z]\b")
                .expect("BUG: DNI regex is invalid"),
            nie_regex: Regex::new(r"\b[XYZ][0-9]{7}[A-Z]\b")
                .expect("BUG: NIE regex is invalid"),
        }
    }
}

impl Detector for DniDetector {
    fn id(&self) -> DetectorId {
        "dni".to_string()
    }
    
    fn category(&self) -> Category {
        Category::NationalId
    }
    
    fn detect(&self, text: &str) -> Vec<CandidateMatch> {
        let mut matches = Vec::new();
        
        for m in self.dni_regex.find_iter(text) {
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
                normalized_value: Some(normalize_dni(raw)),
            });
        }
        
        for m in self.nie_regex.find_iter(text) {
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
                normalized_value: Some(normalize_dni(raw)),
            });
        }
        
        matches
    }

    fn validate(&self, candidate: &str) -> ValidationResult {
        if validate_dni(candidate) {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid
        }
    }

    fn priority(&self) -> u32 {
        100
    }
}

fn normalize_dni(dni: &str) -> String {
    dni.to_uppercase()
}
