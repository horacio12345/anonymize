# Anonymize

Deterministic text anonymization engine with web interface.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

## Overview

`anonymize` is a rule-based text anonymization tool that detects and replaces sensitive data patterns with deterministic placeholders. Built in Rust for performance and reliability, it provides both a web interface and REST API for processing plain text and Word documents.

**Key characteristics:**

- Deterministic: identical input produces identical output
- Formal pattern matching: no heuristics or probabilistic models
- Auditable: complete trace of all replacements
- Offline-capable: no external dependencies at runtime

## Features

- **Personal data detection**: emails, phone numbers, national IDs (DNI/NIE, SSN), IBANs, credit cards
- **Corporate data detection**: project codes, contract numbers, work orders, purchase orders, serial numbers
- **Document processing**: Word (.docx) files with basic format preservation
- **Validation**: checksum verification for IBANs (ISO 7064 Mod 97-10), credit cards (Luhn), Spanish DNI/NIE
- **Processing metrics**: detection count and processing time reported for each operation
- **Web interface**: browser-based UI with file upload support
- **REST API**: programmatic access for integration

## Installation

### Prerequisites

- Rust 1.70 or higher ([installation guide](https://www.rust-lang.org/tools/install))

### Build from source

```bash
git clone https://github.com/horacio12345/anonymize.git
cd anonymize
cargo build --release
```

The compiled binary will be located at `target/release/anonymize`.

## Usage

### Running the server

```bash
cargo run --release
```

The server will start on `http://localhost:3000` by default. Configure the port via environment variable:

```bash
PORT=8080 cargo run --release
```

### Web interface

Navigate to `http://localhost:3000` in your browser. The interface provides:

- **Text tab**: paste text for immediate anonymization
- **File tab**: upload .docx documents for processing

### API endpoints

#### POST /api/anonymize

Anonymize plain text.

**Request:**

```bash
curl -X POST http://localhost:3000/api/anonymize \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d 'text=Contact juan@empresa.com, DNI 12345678Z'
```

**Response:**

```json
{
  "anonymized_text": "Contact [EMAIL_1], DNI [NATIONAL_ID_1]",
  "audit_report": {
    "version": "0.1.0",
    "timestamp": "2025-01-15T10:30:00Z",
    "input_hash": "...",
    "config_hash": "default",
    "statistics": {
      "total_matches": 2,
      "processing_time_ms": 5,
      "matches_by_category": {
        "Email": 1,
        "NationalId": 1
      },
      "conflicts_resolved": 0
    },
    "replacements": [...]
  },
  "hash": "..."
}
```

#### POST /api/anonymize-file

Anonymize Word documents (.docx).

**Request:**

```bash
curl -X POST http://localhost:3000/api/anonymize-file \
  -F "file=@document.docx"
```

**Response:**

```json
{
  "file_base64": "UEsDBBQABgAIA...",
  "filename": "document_anonymized.docx",
  "statistics": {
    "total_detections": 12,
    "processing_time_ms": 8
  }
}
```

The `file_base64` field contains the anonymized document encoded in base64. Decode and save as `.docx` to access the processed document.

## Architecture

### System design

```
┌──────────────────┐
│   Web Browser    │
└────────┬─────────┘
         │ HTTP
         ▼
┌──────────────────┐
│   Axum Server    │  (Rust async runtime)
│   Port 3000      │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Detection       │  (Regex + validation)
│  Engine          │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Replacement     │  (Deterministic placeholders)
│  Engine          │
└──────────────────┘
```

### Processing pipeline

1. **Normalization**: Unicode NFC normalization, whitespace collapse
2. **Detection**: parallel regex matching across all detectors
3. **Conflict resolution**: longest-match-wins strategy with priority tiebreaker
4. **Replacement**: sequential placeholder assignment per category
5. **Output**: anonymized text + audit report + SHA-256 hash

### Components

- **Normalizer**: text preprocessing (Unicode, whitespace)
- **Detectors**: pattern matching modules (email, phone, IBAN, etc.)
- **Conflict Resolver**: handles overlapping matches
- **Replacement Engine**: generates placeholders and substitutes text
- **Document Processor**: handles .docx file parsing and reconstruction
- **Web Layer**: Axum server + static file serving

See [ARCHITECTURE.md](./ARCHITECTURE.md) for detailed technical specification.

## Detection patterns

### Personal identifiers

| Type | Pattern | Validation |
|------|---------|------------|
| Email | RFC 5322 simplified | None |
| IBAN | Country code + check digits | ISO 7064 Mod 97-10 |
| Spanish DNI | 8 digits + letter | Checksum letter |
| Spanish NIE | X/Y/Z + 7 digits + letter | Checksum letter |
| Credit Card | 13-19 digits with separators | Luhn algorithm |
| Phone (ES) | +34 prefix, mobile format | Format only |
| Phone (US/UK) | E.164 format | Format only |
| SSN (US) | XXX-XX-XXXX | Format only |

### Corporate identifiers

| Type | Pattern | Example |
|------|---------|---------|
| Project Code | PRJ/PROY/P-YYYY-NNN | PRJ-2024-001 |
| Contract Number | CTR-YYYY-NNNN | CTR-2024-1234 |
| Work Order | WO/OT-NNNN | OT-2024-5678 |
| Purchase Order | PO/OC-NNNNNN | PO-202412345 |
| Serial Number | SNXX-NNNNNNNN | SNAB-12345678 |
| Cost Center | CC-NNNN | CC-1234 |

## Configuration

Default configuration is used for all operations. Future versions will support `anonymize.toml` for customization:

```toml
[general]
strict_mode = true
max_input_size = 104857600  # bytes

[detection]
locale = "es+en"
parallel = true

[replacement]
strategy = "sequential"
template = "[{category}_{n}]"
```

## Performance

Typical processing metrics for a 24KB document with 12 sensitive patterns:

| Metric | Value |
|--------|-------|
| Detection + replacement | 5-10ms |
| Memory usage | O(n) where n = input size |
| Throughput | ~2000 requests/second (single core) |

The Rust implementation provides consistent low-latency processing suitable for real-time applications.

## Limitations

### Current limitations

- **Text extraction**: .docx processing extracts text only; complex formatting may be lost
- **Pattern-based**: does not detect free-form text (names, addresses) without explicit patterns
- **Single document format**: only .docx is supported; .doc, .pdf, .odt are not supported
- **No streaming**: entire document must fit in memory (max 100MB by default)

### By design

- **No AI/ML**: uses deterministic rules only; no semantic understanding
- **Conservative matching**: prefers false negatives over false positives
- **No context awareness**: each pattern is matched independently

## Testing

```bash
# Run all tests
cargo test

# Run with test data
cat test_input.txt | cargo run

# Check code quality
cargo clippy
```

## Contributing

Contributions are welcome. Please ensure:

1. All tests pass (`cargo test`)
2. Code follows Rust conventions (`cargo fmt`, `cargo clippy`)
3. Changes maintain determinism guarantees
4. New detectors include validation tests

## License

MIT

---

**Technology:** Rust • Axum • HTMX
