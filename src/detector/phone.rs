// src/detector/phone.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};

/// Detector for phone numbers (ES, US, UK, and generic E.164)
pub struct PhoneDetector {
    patterns: Vec<PhonePattern>,
}

struct PhonePattern {
    regex: Regex,
}

impl PhoneDetector {
    /// Create a new phone detector with multiple country patterns
    pub fn new() -> Self {
        let patterns = vec![
            // España: +34 666 123 456 o 666123456 o +34-666-123-456
            PhonePattern {
                regex: Regex::new(r"(?:\+34[-\s]?)?[679][0-9]{2}[-\s]?[0-9]{3}[-\s]?[0-9]{3}")
                    .expect("BUG: Spanish phone regex is invalid"),
            },
            
            // USA: +1-555-123-4567 o (555) 123-4567 o 555-123-4567
            PhonePattern {
                regex: Regex::new(r"(?:\+1[-\s]?)?\(?\d{3}\)?[-\s]?\d{3}[-\s]?\d{4}")
                    .expect("BUG: US phone regex is invalid"),
            },
            
            // UK: +44 20 1234 5678 o +44-20-1234-5678
            PhonePattern {
                regex: Regex::new(r"(?:\+44[-\s]?)?[127][0-9]{3}[-\s]?[0-9]{6}")
                    .expect("BUG: UK phone regex is invalid"),
            },
            
            // E.164 genérico: +[país][número] (1-15 dígitos totales después del +)
            // Esto captura cualquier formato internacional que empiece con +
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
        
        // Ejecutar cada patrón y recoger todos los matches
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
        // Los teléfonos no tienen checksum, solo validación de formato
        ValidationResult::NotApplicable
    }

    fn priority(&self) -> u32 {
        50  // Misma prioridad que email
    }
}

/// Normaliza un teléfono eliminando espacios y guiones
/// Ejemplo: "+34 666-123-456" → "+34666123456"
fn normalize_phone(phone: &str) -> String {
    phone.chars()
        .filter(|c| c.is_ascii_digit() || *c == '+')
        .collect()
}