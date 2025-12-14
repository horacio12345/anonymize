// src/detector/email.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span};

/// Detector for email addresses
pub struct EmailDetector {
    regex: Regex,
}

impl EmailDetector {
    /// Create a new email detector
    pub fn new() -> Self {
        let regex = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}")
            .expect("BUG: Email regex pattern is invalid");
        
        Self { regex }
    }
}

impl Detector for EmailDetector {
    fn id(&self) -> &str {
        "email"
    }
    
    fn category(&self) -> Category {
        Category::Email
    }
    
    fn detect(&self, text: &str) -> Vec<CandidateMatch> {
        self.regex
            .find_iter(text)
            .map(|m| CandidateMatch {
                span: Span {
                    start: m.start(),
                    end: m.end(),
                },
                value: m.as_str().to_string(),
                category: Category::Email,
            })
            .collect()
    }
}