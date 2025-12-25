# Architecture: anonymize

> Deterministic text anonymization engine written in Rust.

This document defines the technical architecture, design decisions, and constraints for `anonymize`. It serves as the authoritative
reference for implementation.

## Design Philosophy

### Core Principles

1. **Formal Verification Only**: Only data matching explicit, deterministic rules is anonymized. No heuristics, no probabilistic models,
   no AI.

2. **Auditability First**: Every transformation must be traceable. The system produces a complete audit trail suitable for compliance
   review.

3. **Determinism Guarantee**: Given identical input and configuration, the output is always identical. This includes placeholder assignment.

4. **Fail-Safe Defaults**: When in doubt, leave data untouched. False negatives are preferable to false positives in formal compliance
   contexts.

5. **Zero External Dependencies at Runtime**: No network calls, no dynamic code loading, no external services.

## System Overview

```text
┌─────────────────────────────────────────────────────────────────┐
│                          INPUT                                  │
│                     (UTF-8 text + Config)                       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                       NORMALIZER                                │
│              Unicode NFC + Whitespace normalization             │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      DETECTION ENGINE                           │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐    │
│  │  Email    │  │   IBAN    │  │  Phone    │  │  Custom   │    │
│  │ Detector  │  │ Detector  │  │ Detector  │  │ Detector  │    │
│  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘    │
│        │              │              │              │           │
│        └──────────────┴──────────────┴──────────────┘           │
│                              │                                  │
│                              ▼                                  │
│                    List<CandidateMatch>                         │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    CONFLICT RESOLVER                            │
│           Longest match wins → Priority tiebreaker              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    REPLACEMENT ENGINE                           │
│         Stable placeholder assignment + text substitution       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      OUTPUT BUILDER                             │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │ Anonymized Text │  │  Audit Report   │  │  Content Hash   │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Component Specification

### 1. Normalizer

**Purpose**: Produce a stable, canonical representation of input text.

**Operations** (in order):

1. Validate UTF-8 encoding (reject invalid input)
2. Apply Unicode NFC normalization
3. Normalize whitespace sequences to single spaces (configurable)
4. Trim leading/trailing whitespace (configurable)

**Input**: `&[u8]` or `&str`

**Output**: `Result<NormalizedText, NormalizationError>`

```rust
pub struct NormalizedText {
    content: String,
    original_len: usize,
    transformations_applied: Vec<NormalizationType>,
}

pub enum NormalizationType {
    UnicodeNfc,
    WhitespaceCollapse,
    Trim,
}

pub enum NormalizationError {
    InvalidUtf8 { position: usize },
    InputTooLarge { size: usize, max: usize },
}
```

**Configuration**:

```toml
[normalizer]
collapse_whitespace = true
trim = true
max_input_size = 104857600  # 100 MB
```

**Security Note**: Homoglyph attacks (using visually similar Unicode characters) are NOT mitigated by normalization. This is
intentional—we detect formal patterns, not visual similarity.

### 2. Detection Engine

**Purpose**: Identify candidate spans that match sensitive data patterns.

**Design**: Each detector is independent and stateless. Detectors run in parallel (when enabled) and produce candidate matches.

#### Detector Trait

```rust
pub trait Detector: Send + Sync {
    /// Unique identifier for this detector type
    fn id(&self) -> DetectorId;
    
    /// Human-readable category for audit reports
    fn category(&self) -> Category;
    
    /// Find all candidate matches in the text
    fn detect(&self, text: &str) -> Vec<CandidateMatch>;
    
    /// Validate a candidate (e.g., checksum verification)
    /// Returns None if validation is not applicable
    fn validate(&self, candidate: &str) -> ValidationResult;
    
    /// Priority for conflict resolution (higher = preferred)
    fn priority(&self) -> u32;
}

pub struct CandidateMatch {
    pub span: Span,
    pub detector_id: DetectorId,
    pub category: Category,
    pub confidence: Confidence,
    pub raw_value: String,
    pub normalized_value: Option<String>,
}

pub struct Span {
    pub start: usize,  // Byte offset, inclusive
    pub end: usize,    // Byte offset, exclusive
}

pub enum Confidence {
    /// Pattern + validation passed (e.g., IBAN with valid checksum)
    Verified,
    /// Pattern matched, no validation available or applicable
    PatternOnly,
}

pub enum ValidationResult {
    Valid,
    Invalid,
    NotApplicable,
}

pub enum Category {
    // Personal
    Email,
    Phone,
    Iban,
    NationalId,
    CreditCard,
    // Industrial/Corporate
    ProjectCode,
    ContractNumber,
    WorkOrder,
    PurchaseOrder,
    SerialNumber,
    CostCenter,
    // Dictionary-based
    CompanyName,
    ProjectName,
    PersonnelName,
    ClientName,
    // User-defined
    Custom(String),
    // Document metadata
    DocumentNumber,
    RevisedBy,
    ApprovedBy,
    DesignedBy,
}
```

#### Built-in Detectors

##### Personal Identifiers

| Detector | Pattern | Validation | Confidence |
|----------|---------|------------|------------|
| Email | RFC 5322 simplified regex | MX lookup disabled (formal only) | PatternOnly |
| IBAN | `[A-Z]{2}[0-9]{2}[A-Z0-9]{4,30}` | ISO 7064 Mod 97-10 | Verified if checksum valid |
| DNI (ES) | `[0-9]{8}[A-Z]` | Letter checksum | Verified if checksum valid |
| NIE (ES) | `[XYZ][0-9]{7}[A-Z]` | Letter checksum | Verified if checksum valid |
| SSN (US) | `[0-9]{3}-[0-9]{2}-[0-9]{4}` | Format only | PatternOnly |
| Credit Card | `[0-9]{13,19}` with separators | Luhn algorithm | Verified if checksum valid |
| Phone (ES) | `(\+34)?[679][0-9]{8}` | Format only | PatternOnly |
| Phone (US/UK) | E.164 format with country code | Format only | PatternOnly |

##### Industrial/Corporate Identifiers

| Detector | Pattern | Validation | Confidence |
|----------|---------|------------|------------|
| Project Code | Configurable prefix + sequence | Format only | PatternOnly |
| Contract Number | Configurable format | Format only | PatternOnly |
| Work Order | `WO-[0-9]{4,10}` / `OT-[0-9]{4,10}` | Format only | PatternOnly |
| PO Number | `PO-[0-9]{6,12}` | Format only | PatternOnly |
| Serial Number | Configurable per equipment type | Format only | PatternOnly |
| Cost Center | `CC-[0-9]{4,8}` | Format only | PatternOnly |

##### Entity Detection (Dictionary-based)

| Detector | Source | Matching | Confidence |
|----------|--------|----------|------------|
| Company Names | User-provided dictionary | Exact + fuzzy (configurable) | PatternOnly |
| Project Names | User-provided dictionary | Exact match | PatternOnly |
| Personnel Names | User-provided dictionary | Exact match | PatternOnly |
| Client Names | User-provided dictionary | Exact match | PatternOnly |

> **Note**: Dictionary-based detection is NOT semantic NER. It matches against explicit, user-provided lists only. No inference,
> no guessing.

#### Custom Detectors (via configuration)

```toml
[[detectors.custom]]
id = "employee_id"
category = "EmployeeId"
pattern = "EMP-[0-9]{6}|LEG-[0-9]{6}"  # English + Spanish prefix
priority = 100
confidence = "PatternOnly"

[[detectors.custom]]
id = "equipment_serial"
category = "SerialNumber"
pattern = "SN[A-Z]{2}-[0-9]{8}"
priority = 100
confidence = "PatternOnly"

[[detectors.custom]]
id = "sap_material"
category = "MaterialCode"
pattern = "[0-9]{18}"  # SAP material number
validator = "none"
priority = 80  # Lower priority to avoid false positives

[[detectors.custom]]
id = "internal_document"
category = "DocumentId"
pattern = "(DOC|INF|REP)-[0-9]{4}-[0-9]{4}"  # DOC/INF/REP-YYYY-NNNN
priority = 120
```

**Supported built-in validators for custom rules**:

- `luhn` - Luhn algorithm (credit cards, some IDs)
- `mod97` - ISO 7064 Mod 97-10 (IBAN)
- `mod11` - Modulo 11 checksum
- `none` - No validation (pattern only)

### 3. Conflict Resolver

**Purpose**: Resolve overlapping matches deterministically.

**Algorithm**:

```text
1. Sort all candidates by (start_position ASC, length DESC, priority DESC)
2. Initialize: accepted = [], last_end = 0
3. For each candidate in sorted order:
   a. If candidate.start >= last_end:
      - Add to accepted
      - Update last_end = candidate.end
   b. Else: discard (overlap with already accepted match)
4. Return accepted
```

**Conflict Resolution Rules** (in order):

1. **Longest match wins**: Given overlapping matches, prefer the longer one
2. **Priority tiebreaker**: If same length, higher priority detector wins
3. **Position tiebreaker**: If same length and priority, earlier start position wins
4. **Deterministic final tiebreaker**: If all else equal, lexicographic order of detector ID

**Example**:

```text
Text: "Contact: juan.garcia@empresa.com"
                |------- EMAIL -------|
                |NAME?|
                
Email span:  [9, 32), length=23, priority=50
Name span:   [9, 20), length=11, priority=40

Result: Email wins (longer match)
```

**Edge Case - Adjacent Matches**:

```text
Text: "DNI: 12345678Z Email: test@example.com"
      |--- DNI ---|       |---- EMAIL ----|
      
These do NOT conflict (no overlap). Both are accepted.
```

### 4. Replacement Engine

**Purpose**: Generate stable, deterministic placeholders and perform substitution.

#### Placeholder Strategy

**Default**: Type-scoped sequential numbering

```text
Format: [<CATEGORY>_<N>]

Examples:
  [EMAIL_1], [EMAIL_2]
  [IBAN_1]
  [PHONE_1], [PHONE_2], [PHONE_3]
  [NATIONAL_ID_1]
```

**Numbering Rules**:

1. Numbers are assigned by order of appearance (byte offset) in the original text
2. Each category maintains its own counter
3. Counters reset per document (not global)

**Determinism Guarantee**: Same input always produces same placeholder assignment.

**Alternative Strategies** (configurable):

```toml
[replacement]
# Default: sequential per category
strategy = "sequential"

# Alternative: hash-based (same value = same placeholder across documents)
# strategy = "hash"
# hash_algorithm = "sha256"
# hash_length = 8  # First N chars of hash

# Alternative: fixed (all instances of same category get same placeholder)
# strategy = "fixed"
```

#### Substitution Process

```rust
pub struct Replacement {
    pub span: Span,
    pub original: String,          // Only if config.store_original_values
    pub placeholder: String,
    pub category: Category,
    pub detector_id: DetectorId,
    pub confidence: Confidence,
}

pub struct ReplacementResult {
    pub anonymized_text: String,
    pub replacements: Vec<Replacement>,
    pub original_length: usize,
    pub anonymized_length: usize,
}
```

**Implementation Notes**:

- Replacements are applied in reverse order (end to start) to preserve byte offsets
- All offsets in `Replacement` refer to the ORIGINAL text, not the transformed text
- This allows audit reports to map back to source positions

### 5. Output Builder

**Purpose**: Assemble final output with audit trail and integrity verification.

#### Output Structure

```rust
pub struct AnonymizationOutput {
    pub text: String,
    pub report: AuditReport,
    pub hash: ContentHash,
}

pub struct AuditReport {
    pub version: String,                    // Schema version
    pub timestamp: DateTime<Utc>,           // Processing time
    pub input_hash: String,                 // SHA-256 of original input
    pub config_hash: String,                // SHA-256 of config used
    pub statistics: Statistics,
    pub replacements: Vec<ReplacementRecord>,
}

pub struct Statistics {
    pub total_matches: usize,
    pub matches_by_category: HashMap<Category, usize>,
    pub conflicts_resolved: usize,
    pub processing_time_ms: u64,
}

pub struct ReplacementRecord {
    pub placeholder: String,
    pub category: Category,
    pub detector_id: String,
    pub confidence: Confidence,
    pub original_span: Span,
    pub anonymized_span: Span,
    // Only present if config.store_original_values = true
    pub original_value: Option<String>,
}

pub struct ContentHash {
    pub algorithm: String,      // "SHA-256"
    pub value: String,          // Hex-encoded hash
    pub scope: HashScope,
}

pub enum HashScope {
    OutputOnly,                 // Hash of anonymized text only
    OutputAndReport,            // Hash of text + JSON report
}
```

#### Audit Modes

```toml
[audit]
# Full: includes original values (audit report is sensitive!)
mode = "full"

# Metadata: positions and categories only, no original values
# mode = "metadata"

# Minimal: only statistics, no individual replacements
# mode = "minimal"
```

**Security Warning**: In `full` mode, the audit report contains all original sensitive data. It must be protected with the same controls
as the original document.

### 6. Configuration System

#### Configuration File Format

```toml
# anonymize.toml

[general]
# Fail on any error vs. best-effort processing
strict_mode = true
# Maximum input size in bytes (0 = unlimited)
max_input_size = 104857600

[normalizer]
unicode_normalization = "NFC"  # NFC, NFD, NFKC, NFKD
collapse_whitespace = true
trim = true

[detection]
# Locale: affects phone formats, ID formats, decimal separators
# Supports: "es", "en", "es+en" (both)
locale = "es+en"
# Run detectors in parallel
parallel = true
# Built-in detectors to enable (empty = all)
enabled = ["email", "iban", "phone", "national_id", "credit_card", "project_code", "company_names"]
# Built-in detectors to explicitly disable
disabled = []

[detection.dictionaries]
# Paths to dictionary files (one entry per line, UTF-8)
company_names = "./dictionaries/companies.txt"
project_names = "./dictionaries/projects.txt"
personnel_names = "./dictionaries/personnel.txt"
client_names = "./dictionaries/clients.txt"
# Case-sensitive matching for dictionaries
case_sensitive = false
# Allow partial word matches (e.g., "ACME" matches "ACME Corp")
partial_match = false

[detection.industrial]
# Project code format (use {N} for digits, {A} for letters)
project_format = "PRJ-{N}{N}{N}{N}-{N}{N}{N}"
# Alternative formats (multiple allowed)
project_formats_alt = ["P-{N}{N}{N}{N}", "{A}{A}{A}-{N}{N}{N}{N}"]
# Contract number format
contract_format = "CTR-{N}{N}{N}{N}-{N}{N}{N}{N}"
# Work order prefixes (ES + EN)
work_order_prefixes = ["WO", "OT", "OdT"]
# Purchase order prefixes
purchase_order_prefixes = ["PO", "OC", "PC"]

[replacement]
strategy = "sequential"
# Placeholder format (supports: {category}, {n}, {hash})
template = "[{category}_{n}]"

[audit]
mode = "metadata"
include_timestamp = true
include_config_hash = true
hash_algorithm = "SHA-256"
hash_scope = "OutputOnly"

# Custom detector definitions
[[detectors.custom]]
id = "employee_id"
category = "EmployeeId"
pattern = "EMP-[0-9]{6}"
priority = 100
enabled = true
```

#### Configuration Loading Priority

1. Explicit path passed to API/CLI
2. `./anonymize.toml`
3. `~/.config/anonymize/config.toml`
4. Built-in defaults

Later sources override earlier ones at the field level (not full replacement).

## Error Handling Strategy

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum AnonymizeError {
    #[error("Invalid UTF-8 at byte position {position}")]
    InvalidUtf8 { position: usize },
    
    #[error("Input exceeds maximum size: {size} > {max}")]
    InputTooLarge { size: usize, max: usize },
    
    #[error("Configuration error: {message}")]
    ConfigError { message: String },
    
    #[error("Invalid regex pattern in detector '{detector}': {message}")]
    InvalidPattern { detector: String, message: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AnonymizeError>;
```

### Error Philosophy

1. **Fail fast in strict mode**: Any error stops processing
2. **Best effort in lenient mode**: Log warnings, continue with valid portions
3. **Never panic**: All errors are `Result` types
4. **Actionable messages**: Every error indicates what went wrong and where

## Performance Considerations

### Memory Model

- **Streaming not supported**: Full document must fit in memory
- **Copy minimization**: Use `Cow<str>` for text that may or may not be modified
- **Pre-allocation**: Estimate output size based on input size and match count

### Complexity

| Operation | Time Complexity | Space Complexity |
|-----------|-----------------|------------------|
| Normalization | O(n) | O(n) |
| Detection (single) | O(n) | O(m) where m = matches |
| Detection (parallel) | O(n/p) | O(m) |
| Conflict Resolution | O(m log m) | O(m) |
| Replacement | O(n + m) | O(n) |

Where n = input length, m = number of matches, p = parallelism factor.

### Recommended Limits

| Parameter | Recommended | Maximum Tested |
|-----------|-------------|----------------|
| Input size | < 10 MB | 100 MB |
| Matches per document | < 10,000 | 100,000 |
| Custom detectors | < 50 | 200 |
| Regex complexity | < 100 chars | Varies |

## Public API Design

### Library API

```rust
// Simple API
pub fn anonymize(text: &str) -> Result<AnonymizationOutput>;
pub fn anonymize_with_config(text: &str, config: &Config) -> Result<AnonymizationOutput>;

// Builder API for advanced use
pub struct Anonymizer {
    config: Config,
    detectors: Vec<Box<dyn Detector>>,
}

impl Anonymizer {
    pub fn new() -> Self;
    pub fn with_config(config: Config) -> Self;
    pub fn add_detector(&mut self, detector: impl Detector + 'static) -> &mut Self;
    pub fn remove_detector(&mut self, id: &DetectorId) -> &mut Self;
    pub fn anonymize(&self, text: &str) -> Result<AnonymizationOutput>;
}

// Reusable for multiple documents
impl Anonymizer {
    pub fn anonymize_batch(&self, texts: &[&str]) -> Vec<Result<AnonymizationOutput>>;
}
```

### CLI Interface

```bash
# Basic usage
anonymize input.txt -o output.txt

# With audit report
anonymize input.txt -o output.txt --report report.json

# Custom config
anonymize input.txt -c custom-config.toml -o output.txt

# Stdin/stdout
cat document.txt | anonymize > anonymized.txt

# Batch processing
anonymize --batch ./documents/ --output-dir ./anonymized/

# Verify (check hash)
anonymize --verify output.txt --hash <expected-hash>
```

## Security Model

### Threat Model

| Threat | Mitigation |
|--------|------------|
| Sensitive data in logs | No logging of matched values by default |
| Audit report exposure | Configurable verbosity; warn on `full` mode |
| Regex ReDoS | Timeout on regex execution; complexity limits |
| Memory exhaustion | Configurable input size limits |
| Timing attacks | Not in scope (not a cryptographic system) |

### Security Boundaries

1. **Input boundary**: Only UTF-8 text accepted; binary data rejected
2. **Output boundary**: Anonymized text + audit report + hash
3. **No network**: Zero external calls; air-gap compatible
4. **No persistence**: No state between invocations; no temp files

### Sensitive Data Handling

```rust
// Values that may contain sensitive data implement ZeroizeOnDrop
use zeroize::ZeroizeOnDrop;

#[derive(ZeroizeOnDrop)]
pub struct SensitiveString(String);
```

## Testing Strategy

### Test Categories

1. **Unit tests**: Each component in isolation
2. **Integration tests**: Full pipeline with known inputs
3. **Property tests**: Fuzzing with arbitrary inputs (proptest)
4. **Regression tests**: Known edge cases and bug fixes
5. **Determinism tests**: Same input → same output across runs

### Critical Test Cases

```rust
#[test]
fn test_determinism() {
    let input = "...";
    let result1 = anonymize(input);
    let result2 = anonymize(input);
    assert_eq!(result1.text, result2.text);
    assert_eq!(result1.hash, result2.hash);
}

#[test]
fn test_no_false_positives_on_clean_text() {
    let input = "This text contains no sensitive data.";
    let result = anonymize(input);
    assert_eq!(result.text, input);
    assert!(result.report.replacements.is_empty());
}

#[test]
fn test_overlapping_matches_resolved() {
    let input = "Contact: juan@empresa.com";
    let result = anonymize(input);
    // Email should win over potential name detection
    assert!(result.text.contains("[EMAIL_1]"));
}
```

## Future Considerations (Out of Scope for v1)

1. **Streaming processing** for large documents
2. **Incremental processing** for document diffs
3. **Pseudonymization** with reversible mappings
4. **Multi-language support** for phone number patterns
5. **WASM target** for browser usage
6. **FFI bindings** for Python/Node.js

## Appendix A: Regex Patterns

### Email (RFC 5322 Simplified)

```regex
[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}
```

### IBAN

```regex
[A-Z]{2}[0-9]{2}[A-Z0-9]{4}[0-9]{7}([A-Z0-9]?){0,16}
```

### Spanish DNI

```regex
[0-9]{8}[A-Z]
```

### Spanish NIE

```regex
[XYZ][0-9]{7}[A-Z]
```

### Credit Card (with optional separators)

```regex
[0-9]{4}[-\s]?[0-9]{4}[-\s]?[0-9]{4}[-\s]?[0-9]{4}|[0-9]{13,19}
```

### Spanish Phone

```regex
(?:\+34[-\s]?)?[679][0-9]{2}[-\s]?[0-9]{3}[-\s]?[0-9]{3}
```

## Appendix B: Checksum Algorithms

### IBAN (ISO 7064 Mod 97-10)

```rust
fn validate_iban(iban: &str) -> bool {
    let cleaned: String = iban.chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    
    if cleaned.len() < 5 {
        return false;
    }
    
    // Move first 4 chars to end
    let rearranged = format!("{}{}", &cleaned[4..], &cleaned[..4]);
    
    // Convert letters to numbers (A=10, B=11, ...)
    let numeric: String = rearranged.chars()
        .map(|c| {
            if c.is_ascii_digit() {
                c.to_string()
            } else {
                ((c as u32) - ('A' as u32) + 10).to_string()
            }
        })
        .collect();
    
    // Mod 97 check
    let remainder = numeric
        .chars()
        .fold(0u64, |acc, c| {
            (acc * 10 + c.to_digit(10).unwrap() as u64) % 97
        });
    
    remainder == 1
}
```

### Spanish DNI Letter

```rust
fn validate_dni(dni: &str) -> bool {
    const LETTERS: &[char] = &[
        'T', 'R', 'W', 'A', 'G', 'M', 'Y', 'F', 'P', 'D',
        'X', 'B', 'N', 'J', 'Z', 'S', 'Q', 'V', 'H', 'L',
        'C', 'K', 'E'
    ];
    
    let chars: Vec<char> = dni.chars().collect();
    if chars.len() != 9 {
        return false;
    }
    
    let number: u32 = chars[..8].iter()
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    
    let expected_letter = LETTERS[(number % 23) as usize];
    chars[8].to_ascii_uppercase() == expected_letter
}
```

### Luhn Algorithm

```rust
fn validate_luhn(number: &str) -> bool {
    let digits: Vec<u32> = number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    if digits.is_empty() {
        return false;
    }
    
    let sum: u32 = digits.iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                d
            }
        })
        .sum();
    
    sum % 10 == 0
}
```

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1.0 | 2025-XX-XX | — | Initial architecture draft |
