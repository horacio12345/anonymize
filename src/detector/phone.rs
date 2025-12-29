// src/detector/phone.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};

pub struct PhoneDetector {
    patterns: Vec<PhonePattern>,
}

struct PhonePattern {
    regex: Regex,
}

impl PhoneDetector {
    pub fn new() -> Self {
        let patterns = vec![
            PhonePattern {
                regex: Regex::new(r"(?:\+34[-\s]?)?[679][0-9]{2}[-\s]?[0-9]{3}[-\s]?[0-9]{3}")
                    .expect("BUG: Spanish phone regex is invalid"),
            },
            PhonePattern {
                regex: Regex::new(r"(?:\+1[-\s]?)?\(?\d{3}\)?[-\s]?\d{3}[-\s]?\d{4}")
                    .expect("BUG: US phone regex is invalid"),
            },
            PhonePattern {
                regex: Regex::new(r"(?:\+44[-\s]?)?[127][0-9]{3}[-\s]?[0-9]{6}")
                    .expect("BUG: UK phone regex is invalid"),
            },
            PhonePattern {
                regex: Regex::new(r"\+[1-9]\d{1,14}")
                    .expect("BUG: E.164 phone regex is invalid"),
            },
        ];
        Self { patterns }
    }
}

impl Detector for PhoneDetector {
    fn id(&self) -> DetectorId {
        "phone".to_string()
    }
    
    fn category(&self) -> Category {
        Category::Phone
    }
    
    fn detect(&self, text: &str) -> Vec<CandidateMatch> {
        let mut all_matches = Vec::new();
        for pattern in &self.patterns {
            let matches = pattern.regex
                .find_iter(text)
                .map(|m| CandidateMatch {
                    span: Span {
                        start: m.start(),
                        end: m.end(),
                    },
                    detector_id: self.id(),
                    category: Category::Phone,
                    priority: self.priority(),
                    confidence: Confidence::PatternOnly,
                    raw_value: m.as_str().to_string(),
                    normalized_value: Some(normalize_phone(m.as_str())),
                })
                .collect::<Vec<_>>();
            all_matches.extend(matches);
        }
        all_matches
    }

    fn validate(&self, _candidate: &str) -> ValidationResult {
        ValidationResult::NotApplicable
    }

    fn priority(&self) -> u32 {
        50
    }
}

fn normalize_phone(phone: &str) -> String {
    phone.chars()
        .filter(|c| c.is_ascii_digit() || *c == '+')
        .collect()
}
