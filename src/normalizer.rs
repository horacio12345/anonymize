// src/normalizer.rs

use unicode_normalization::UnicodeNormalization;
use crate::{AnonymizeError, Result};

/// Text after normalization with metadata
pub struct NormalizedText {
    pub content: String,
    pub original_len: usize,
    pub transformations_applied: Vec<NormalizationType>,
}

/// Types of normalization transformations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationType {
    UnicodeNfc,
    WhitespaceCollapse,
    Trim,
}

/// Normalize text according to specified rules
pub fn normalize(text: &str) -> Result<NormalizedText> {
    const MAX_INPUT_SIZE: usize = 100_000_000; // 100 MB
    
    // Validate input size
    if text.len() > MAX_INPUT_SIZE {
        return Err(AnonymizeError::InputTooLarge {
            size: text.len(),
            max: MAX_INPUT_SIZE,
        });
    }
    
    let original_len = text.len();
    let mut transformations = Vec::new();
    
    // Apply Unicode NFC normalization
    let mut normalized: String = text.nfc().collect();
    transformations.push(NormalizationType::UnicodeNfc);
    
    // Collapse whitespace sequences to single spaces
    normalized = normalized
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    transformations.push(NormalizationType::WhitespaceCollapse);
    
    // Trim leading/trailing whitespace
    normalized = normalized.trim().to_string();
    transformations.push(NormalizationType::Trim);
    
    Ok(NormalizedText {
        content: normalized,
        original_len,
        transformations_applied: transformations,
    })
}