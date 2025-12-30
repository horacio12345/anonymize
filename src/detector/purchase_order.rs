// src/detector/purchase_order.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};

pub struct PurchaseOrderDetector {
    regex: Regex,
}

impl Default for PurchaseOrderDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl PurchaseOrderDetector {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(r"\b(?:PO|OC|PC)-[0-9]{6,12}\b")
                .expect("BUG: Purchase order regex is invalid"),
        }
    }
}

impl Detector for PurchaseOrderDetector {
    fn id(&self) -> DetectorId {
        "purchase_order".to_string()
    }
    
    fn category(&self) -> Category {
        Category::PurchaseOrder
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
