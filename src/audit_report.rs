// src/audit_report.rs

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::detector::Span;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuditReport {
    pub version: String,
    pub timestamp: DateTime<Utc>,
    pub input_hash: String,
    pub config_hash: String,
    pub statistics: Statistics,
    pub replacements: Vec<ReplacementRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Statistics {
    pub total_matches: usize,
    pub matches_by_category: HashMap<String, usize>,
    pub conflicts_resolved: usize,
    pub processing_time_ms: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplacementRecord {
    pub placeholder: String,
    pub category: String,
    pub detector_id: String,
    pub confidence: String,
    pub original_span: Span,
    pub original_value: Option<String>, // Añadido para usar 'original' de Replacement
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentHash {
    pub algorithm: String,
    pub value: String,
}
