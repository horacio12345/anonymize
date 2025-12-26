// src/detector/cost_center.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};

/// Detector for cost centers
pub struct CostCenterDetector {
    regex: Regex,
}

impl CostCenterDetector {
    pub fn new() -> Self {
        Self {
            // Formato: CC-XXXX a CC-XXXXXXXX
            regex: Regex::new(r"\bCC-[0-9]{4,8}\b")
                .expect("BUG: Cost center regex is invalid"),
        }
    }
}

impl Detector for CostCenterDetector {
    fn id(&self) -> DetectorId {
        "cost_center".to_string()
    }
    
    fn category(&self) -> Category {
        Category::CostCenter
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