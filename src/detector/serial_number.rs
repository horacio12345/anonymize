// src/detector/serial_number.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};

pub struct SerialNumberDetector {
    regex: Regex,
}

impl SerialNumberDetector {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(r"\bSN[A-Z]{2}-[0-9]{8}\b")
                .expect("BUG: Serial number regex is invalid"),
        }
    }
}

impl Detector for SerialNumberDetector {
    fn id(&self) -> DetectorId {
        "serial_number".to_string()
    }
    
    fn category(&self) -> Category {
        Category::SerialNumber
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
        60
    }
}
