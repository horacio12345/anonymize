# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-15

### Added
- ✨ Web interface with HTMX (14KB, no heavy JS frameworks)
- 🚀 Axum web server with REST API
- 📊 Real-time anonymization via POST /api/anonymize
- 🔍 12 built-in detectors:
  - Personal: Email, Phone (ES/EN/UK), DNI/NIE, IBAN, Credit Card, SSN
  - Corporate: Project Code, Contract Number, Work Order, Purchase Order, Serial Number, Cost Center
- 📝 Complete audit reports with JSON output
- 🔒 Deterministic anonymization (same input = same output)
- 🎨 Responsive web UI with modern design
- 📦 One-command deployment script for Hetzner VPS
- 🔧 Systemd service configuration
- 🌐 Nginx reverse proxy setup
- 📚 Comprehensive documentation (README, QUICKSTART, ARCHITECTURE)
- 🧪 Sample test files and Makefile

### Technical Details
- Rust 2021 edition
- Axum 0.7 web framework
- Tower-HTTP for CORS and static file serving
- Unicode NFC normalization
- ISO 7064 Mod 97-10 IBAN validation
- Luhn algorithm for credit card validation
- Spanish DNI/NIE letter validation
- SHA-256 hashing for audit trails

### Architecture
- Modular detector system (easy to extend)
- Conflict resolution with deterministic tiebreakers
- Sequential placeholder numbering strategy
- Zero external runtime dependencies
- Offline-capable (air-gap compatible)

## [Unreleased]

### Planned
- [ ] Configuration file support (TOML)
- [ ] Dictionary-based detection (company names, personnel)
- [ ] Custom regex detectors via config
- [ ] Batch processing API endpoint
- [ ] Streaming support for large files
- [ ] Multi-language phone number support
- [ ] Docker containerization
- [ ] Prometheus metrics endpoint
- [ ] Rate limiting middleware
- [ ] CLI mode preservation (optional flag)

### Improvements Planned
- [ ] Performance optimizations for >10MB files
- [ ] Parallel detection execution
- [ ] Memory usage optimization
- [ ] Better error messages in UI
- [ ] Dark mode toggle
- [ ] Export audit reports as CSV/PDF

---

**Legend:**
- ✨ New feature
- 🐛 Bug fix
- 🔧 Improvement
- 📚 Documentation
- 🔒 Security
- ⚡ Performance
