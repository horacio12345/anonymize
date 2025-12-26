// src/detector/work_order.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};

/// Detector for work orders
pub struct WorkOrderDetector {
    regex: Regex,
}

impl WorkOrderDetector {
    pub fn new() -> Self {
        Self {
            // Formatos: WO-XXXXX, OT-XXXXX (español), OdT-XXXXX
            regex: Regex::new(r"\b(?:WO|OT|OdT)-[0-9]{4,10}\b")
                .expect("BUG: Work order regex is invalid"),
        }
    }
}

impl Detector for WorkOrderDetector {
    fn id(&self) -> DetectorId {
        "work_order".to_string()
    }
    
    fn category(&self) -> Category {
        Category::WorkOrder
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