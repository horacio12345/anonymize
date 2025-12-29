.PHONY: build run test clean dev release deploy

# Development
dev:
	cargo run

# Build release
build:
	cargo build --release

# Run release binary
run: build
	./target/release/anonymize

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Release build
release:
	cargo build --release --locked

# Deploy to server (requires SERVER variable)
deploy: release
	@if [ -z "$(SERVER)" ]; then \
		echo "❌ Error: Especifica SERVER=user@host"; \
		exit 1; \
	fi
	./deploy.sh $(SERVER)

# Format code
fmt:
	cargo fmt

# Check code
check:
	cargo check
	cargo clippy

# Full check before commit
pre-commit: fmt check test
	@echo "✅ All checks passed!"

# Quick test with sample input
test-input: build
	cat test_input.txt | ./target/release/anonymize

# Help
help:
	@echo "Anonymize - Makefile Commands"
	@echo "=============================="
	@echo ""
	@echo "Development:"
	@echo "  make dev         - Run in development mode"
	@echo "  make build       - Build release binary"
	@echo "  make run         - Build and run release binary"
	@echo "  make test        - Run all tests"
	@echo ""
	@echo "Code Quality:"
	@echo "  make fmt         - Format code"
	@echo "  make check       - Run checks and linting"
	@echo "  make pre-commit  - Run all checks before commit"
	@echo ""
	@echo "Deployment:"
	@echo "  make deploy SERVER=user@host  - Deploy to server"
	@echo ""
	@echo "Other:"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make test-input  - Test with sample input"
	@echo "  make help        - Show this help"
