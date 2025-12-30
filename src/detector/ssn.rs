// src/detector/ssn.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};

pub struct SsnDetector {
    regex: Regex,
}

impl Default for SsnDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl SsnDetector {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(r"\b[0-9]{3}-[0-9]{2}-[0-9]{4}\b")
                .expect("BUG: SSN regex is invalid"),
        }
    }
}

impl Detector for SsnDetector {
    fn id(&self) -> DetectorId {
        "ssn".to_string()
    }
    
    fn category(&self) -> Category {
        Category::NationalId
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
        80
    }
}
