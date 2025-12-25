// src/detector/dni.rs

use regex::Regex;
use crate::detector::{Detector, CandidateMatch, Category, Span, DetectorId, Confidence, ValidationResult};
use crate::utils::checksum::validate_dni;

/// Detector for Spanish DNI and NIE (National ID)
pub struct DniDetector {
    dni_regex: Regex,
    nie_regex: Regex,
}

impl DniDetector {
    /// Create a new DNI/NIE detector
    pub fn new() -> Self {
        Self {
            // DNI: 8 dígitos + 1 letra (ej: 12345678Z)
            dni_regex: Regex::new(r"\b[0-9]{8}[A-Z]\b")
                .expect("BUG: DNI regex is invalid"),
            
            // NIE: [XYZ] + 7 dígitos + 1 letra (ej: X1234567L)
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
        
        // Detectar DNIs
        for m in self.dni_regex.find_iter(text) {
            let raw = m.as_str();
            let validation = self.validate(raw);
            
            // Solo añadimos el match si la validación pasa o no es aplicable
            // (en este caso, siempre validamos, así que solo añadimos si es válido)
            let confidence = match validation {
                ValidationResult::Valid => Confidence::Verified,
                ValidationResult::Invalid => continue, // No añadimos matches inválidos
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
        
        // Detectar NIEs
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
        // Usamos la función de checksum que ya existe en utils
        if validate_dni(candidate) {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid
        }
    }

    fn priority(&self) -> u32 {
        100 // Mayor que email/phone para evitar falsos positivos
    }
}

/// Normaliza un DNI/NIE (solo letras mayúsculas)
fn normalize_dni(dni: &str) -> String {
    dni.to_uppercase()
}