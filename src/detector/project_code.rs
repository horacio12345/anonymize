// src/detector/project_code.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};

pub struct ProjectCodeDetector {
    regex: Regex,
}

impl Default for ProjectCodeDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectCodeDetector {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(r"\b(?:PRJ|PROY|P)-[0-9]{4}(?:-[0-9]{3,4})?\b")
                .expect("BUG: Project code regex is invalid"),
        }
    }
}

impl Detector for ProjectCodeDetector {
    fn id(&self) -> DetectorId {
        "project_code".to_string()
    }
    
    fn category(&self) -> Category {
        Category::ProjectCode
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
