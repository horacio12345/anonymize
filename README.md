# anonymize

Deterministic text anonymization engine written in Rust.

## Problem

Organizations need to process documents through external systems (LLMs, cloud services, third-party tools) but cannot expose sensitive data—personal or corporate.

Current solutions rely on AI/ML for detection, which means:

- Non-deterministic results
- Unexplainable decisions
- Compliance risks

## Solution

`anonymize` removes sensitive data using **formal, verifiable rules only**.

No AI. No heuristics. No guessing.

If a value matches an explicit pattern and passes validation (when applicable), it gets replaced. If not, it stays untouched.

## What it detects

**Personal data:**

- Emails, phone numbers (ES/EN)
- IBANs, credit cards
- National IDs (DNI/NIE, SSN)

**Corporate/Industrial data:**

- Project codes, contract numbers
- Work orders, purchase orders
- Company/client/personnel names (via dictionaries)
- Custom identifiers (configurable)

## What it produces

1. **Anonymized text** — sensitive values replaced with placeholders (`[EMAIL_1]`, `[IBAN_1]`, `[PROJECT_CODE_1]`)
2. **Audit report** — full traceability of what was replaced and where
3. **Content hash** — cryptographic verification of output integrity

## Design principles

- **Deterministic**: Same input + config = same output. Always.
- **Auditable**: Every replacement is traceable and explainable.
- **Offline**: No network calls. Runs on air-gapped environments.
- **Conservative**: When in doubt, leave data untouched.

## Status

Early stage — architecture defined, implementation pending.

See [ARCHITECTURE.md](./ARCHITECTURE.md) for technical details.

## License

TBD
