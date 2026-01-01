# Anonymize

Deterministic text anonymization engine with web interface.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

## Overview

`anonymize` is a rule-based text anonymization tool that detects and replaces sensitive data patterns with deterministic placeholders. Built in Rust for performance and reliability, it provides both a web interface and HTTP API for processing plain text and Word documents.

**Key features:**

- Deterministic replacement (same input → same output)
- 15+ pattern detectors (DNI/NIE, IBAN, credit cards, emails, etc.)
- Cryptographic audit trail (SHA-256 hashes)
- Web UI with file upload support (.docx, .pdf)
- Zero configuration required

## Quick Start

### Prerequisites

- Rust 1.70 or later
- Cargo

### Installation

```bash
git clone https://github.com/yourusername/anonymize.git
cd anonymize
cargo build --release
```

### Usage

**Web Interface:**

```bash
cargo run --release
# Open http://localhost:3000
```

**Command Line:**

```bash
echo "My DNI is 12345678Z" | cargo run --release
# Output: My DNI is [DNI_001]
```

**HTTP API:**

```bash
curl -X POST http://localhost:3000/api/anonymize \
  -H "Content-Type: application/json" \
  -d '{"text":"Contact: john@example.com, phone +34-612-345-678"}'
```

## Supported Patterns

| Category | Examples | Checksum Validation |
|----------|----------|---------------------|
| **Spanish ID** | DNI, NIE | ✅ Mod-23 algorithm |
| **Banking** | IBAN (ES), Credit Cards | ✅ ISO 7064, Luhn |
| **Contact** | Email, Phone (ES/intl), URLs | ❌ Format only |
| **Identification** | Passport, Social Security | ❌ Format only |
| **Legal** | NIF, CIF, License Plates | ✅ Checksums |
| **Finance** | Bank accounts, Swift codes | ✅ Partial |
| **Business** | VAT (EU), Tax IDs | ✅ Country-specific |
| **Professional** | Work orders, contracts, invoices | ❌ Format only |

### Pattern Details

- **DNI/NIE**: Spanish national ID with mod-23 letter validation
- **IBAN**: International bank account (ES prefix validated)
- **Credit Card**: Visa, MasterCard, Amex (Luhn algorithm)
- **Phone**: Spanish landlines/mobiles (+34) and international formats
- **License Plate**: Spanish vehicle plates (1234-ABC format)

## Architecture

### Core Components

```
src/
├── detector/          # 15+ pattern detection modules
│   ├── dni.rs        # Spanish ID with checksum
│   ├── iban.rs       # IBAN with ISO 7064
│   ├── credit_card.rs # Luhn validation
│   └── ...
├── engine.rs          # Main processing pipeline
├── normalizer.rs      # Input sanitization
├── conflict.rs        # Overlap resolution
├── audit_report.rs    # Cryptographic audit trail
└── web/              # HTTP server (Axum)
```

### Processing Pipeline

1. **Normalization**: Unicode normalization, max 100MB limit
2. **Detection**: All patterns matched in parallel
3. **Conflict Resolution**: Overlaps resolved by priority
4. **Replacement**: Deterministic substitution with counters
5. **Audit**: SHA-256 hashes + full trace report

## API Reference

### POST /api/anonymize

**Request:**

```json
{
  "text": "Contact: john@example.com, DNI 12345678Z"
}
```

**Response:**

```json
{
  "anonymized_text": "Contact: [EMAIL_001], DNI [DNI_001]",
  "audit_report": {
    "version": "0.1.0",
    "timestamp": "2024-12-27T10:30:00Z",
    "input_hash": "a3f5...",
    "config_hash": "default",
    "statistics": {
      "total_matches": 2,
      "matches_by_category": {
        "email": 1,
        "dni": 1
      },
      "conflicts_resolved": 0,
      "processing_time_ms": 8
    },
    "replacements": [
      {
        "original": "john@example.com",
        "replacement": "[EMAIL_001]",
        "category": "email",
        "position": 9
      }
    ]
  },
  "hash": "b7e9..."
}
```

### POST /api/anonymize-file

**Request:** `multipart/form-data` with file field

**Response:** Anonymized document (same format as input)

**Supported formats:** .docx, .pdf

## Configuration

### Environment Variables

```bash
PORT=3000              # Server port (default: 3000)
RUST_LOG=info         # Logging level
```

### Future: anonymize.toml

Custom configuration via `anonymize.toml` is planned for future versions.

## Performance

Indicative metrics from local testing (24KB document, 12 sensitive patterns):

| Metric | Value |
|--------|-------|
| Detection + replacement | 5-10ms |
| Memory usage | O(n) where n = input size |

Performance varies depending on hardware and input characteristics.

## Limitations

### Current limitations

- **Text extraction**: .docx processing extracts text only; complex formatting (tables, styles) may be simplified
- **Pattern-based**: does not detect free-form text (names, addresses) without explicit patterns
- **Single document format**: only .docx and .pdf are supported; .doc, .odt are not supported
- **No streaming**: entire document must fit in memory (max 100MB)
- **Web UI file limit**: 10MB maximum in browser interface

### By design

- **No AI/ML**: uses deterministic rules only; no semantic understanding
- **Conservative matching**: prefers false negatives over false positives
- **No context awareness**: each pattern is matched independently

## Testing

```bash
# Run all tests
cargo test

# Run with test data
echo "Test DNI: 12345678Z" | cargo run

# Check code quality
cargo clippy
cargo fmt --check
```

## Deployment

### Production Build

```bash
cargo build --release
./target/release/anonymize
```

### Docker (Optional)

```bash
docker build -t anonymize .
docker run -p 3000:3000 anonymize
```

### Systemd Service

```ini
[Unit]
Description=Anonymize Web Service
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/anonymize
ExecStart=/opt/anonymize/target/release/anonymize
Restart=always

[Install]
WantedBy=multi-user.target
```

## Contributing

Contributions are welcome. Please ensure:

1. All tests pass (`cargo test`)
2. Code follows Rust conventions (`cargo fmt`, `cargo clippy`)
3. Changes maintain determinism guarantees
4. New detectors include validation tests

## License

MIT - See [LICENSE](LICENSE) for details.

## Acknowledgments

Built with:

- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [HTMX](https://htmx.org/) - Frontend interactivity

## Roadmap

- [ ] Custom pattern configuration via TOML
- [ ] Batch processing API
- [ ] More document formats (.odt, .rtf)
- [ ] Docker compose with nginx
- [ ] Prometheus metrics endpoint
- [ ] CLI with colored output
