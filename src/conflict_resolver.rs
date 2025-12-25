// src/conflict_resolver.rs

use crate::detector::CandidateMatch;

/// Resolves overlapping matches deterministically.
pub struct ConflictResolver;

impl ConflictResolver {
    /// Resolve conflicts in a list of candidate matches.
    /// Algorithm:
    /// 1. Sort all candidates by (start_position ASC, length DESC, priority DESC)
    /// 2. Iterate and accept if it doesn't overlap with the last accepted match.
    pub fn resolve(mut candidates: Vec<CandidateMatch>) -> Vec<CandidateMatch> {
        candidates.sort_by(|a, b| {
            // 1. Sort by start position ASC
            a.span.start.cmp(&b.span.start)
                // 2. Sort by length DESC
                .then_with(|| {
                    let len_a = a.span.end - a.span.start;
                    let len_b = b.span.end - b.span.start;
                    len_b.cmp(&len_a)
                })
                // 3. Sort by priority DESC
                .then_with(|| b.priority.cmp(&a.priority))
                // 4. Deterministic final tiebreaker: DetectorId
                .then_with(|| a.detector_id.cmp(&b.detector_id))
        });

        let mut accepted = Vec::new();
        let mut last_end = 0;

        for candidate in candidates {
            if candidate.span.start >= last_end {
                last_end = candidate.span.end;
                accepted.push(candidate);
            }
        }

        accepted
    }
}
