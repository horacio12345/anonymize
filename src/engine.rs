// src/engine.rs

use crate::Result;
use crate::normalizer::normalize;
use crate::detector::Detector;
use crate::conflict_resolver::ConflictResolver;
use crate::replacement_engine::ReplacementEngine;
use crate::audit_report::{AuditReport, ContentHash};
use sha2::{Sha256, Digest};
use std::time::Instant;
use std::collections::HashMap;

pub struct AnonymizationOutput {
    pub text: String,
    pub report: AuditReport,
    pub hash: ContentHash,
}

pub struct Anonymizer {
    detectors: Vec<Box<dyn Detector>>,
}

impl Anonymizer {
    pub fn new() -> Self {
        Self {
            detectors: Vec::new(),
        }
    }

    pub fn add_detector(&mut self, detector: Box<dyn Detector>) {
        self.detectors.push(detector);
    }

    pub fn anonymize(&self, text: &str) -> Result<AnonymizationOutput> {
        let start_time = Instant::now();
        
        // 1. Normalize
        let normalized = normalize(text)?;
        
        // 2. Detection
        let mut all_candidates = Vec::new();
        for detector in &self.detectors {
            let matches = detector.detect(&normalized.content);
            all_candidates.extend(matches);
        }
        
        let initial_match_count = all_candidates.len();
        
        // 3. Conflict Resolution
        let resolved_matches = ConflictResolver::resolve(all_candidates);
        let final_match_count = resolved_matches.len();
        let conflicts_resolved = initial_match_count - final_match_count;
        
        // 4. Replacement
        let replacement_result = ReplacementEngine::replace(&normalized.content, resolved_matches);
        
        // 5. Output Building & Audit
        let input_hash_val = format!("{:x}", Sha256::digest(text.as_bytes()));
        let output_hash_val = format!("{:x}", Sha256::digest(replacement_result.anonymized_text.as_bytes()));

        let mut matches_by_category = HashMap::new();
        for r in &replacement_result.replacements {
            let cat_str = format!("{:?}", r.category);
            *matches_by_category.entry(cat_str).or_insert(0) += 1;
        }
        
        let report = AuditReport {
            version: "0.1.0".to_string(),
            timestamp: chrono::Utc::now(),
            input_hash: input_hash_val,
            config_hash: "default".to_string(),
            statistics: crate::audit_report::Statistics {
                total_matches: final_match_count,
                matches_by_category,
                conflicts_resolved,
                processing_time_ms: start_time.elapsed().as_millis() as u64,
            },
            replacements: replacement_result.replacements.into_iter().map(|r| {
                crate::audit_report::ReplacementRecord {
                    placeholder: r.placeholder,
                    category: format!("{:?}", r.category),
                    detector_id: r.detector_id,
                    confidence: format!("{:?}", r.confidence),
                    original_span: r.span,
                    original_value: Some(r.original),
                }
            }).collect(),
        };

        Ok(AnonymizationOutput {
            text: replacement_result.anonymized_text,
            report,
            hash: ContentHash {
                algorithm: "SHA-256".to_string(),
                value: output_hash_val,
            },
        })
    }
}
