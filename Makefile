.PHONY: build release test lint format check clean watch doc install-tools help

build:
	cargo build --workspace

release:
	cargo build --workspace --release

test:
	cargo test --workspace

lint:
	cargo clippy --workspace -- -D warnings

format:
	cargo fmt --all

format-check:
	cargo fmt --all -- --check

check: format-check lint test

clean:
	cargo clean

watch:
	cargo watch -x 'build --workspace'

watch-test:
	cargo watch -x 'test --workspace'

doc:
	cargo doc --workspace --no-deps --open

install-tools:
	cargo install cargo-watch cargo-expand cargo-audit cargo-nextest

audit:
	cargo audit

# Run a specific project: make run P=1 (for project1-echo-server)
run:
ifdef P
	cargo run -p project$(P)-$(shell ls -d project$(P)-* 2>/dev/null | sed 's/project$(P)-//')
else
	@echo "Usage: make run P=<project_number>"
	@echo "Example: make run P=1"
endif

# Test a specific project
test-p:
ifdef P
	cargo test -p project$(P)-$(shell ls -d project$(P)-* 2>/dev/null | sed 's/project$(P)-//')
else
	@echo "Usage: make test-p P=<project_number>"
endif

help:
	@echo "Available targets:"
	@echo "  build         - Build all workspace crates (debug)"
	@echo "  release       - Build all workspace crates (release)"
	@echo "  test          - Run all tests"
	@echo "  lint          - Run clippy with warnings as errors"
	@echo "  format        - Format all code"
	@echo "  format-check  - Check formatting without modifying"
	@echo "  check         - Run format-check, lint, and test"
	@echo "  clean         - Remove target directory"
	@echo "  watch         - Watch and rebuild on changes"
	@echo "  watch-test    - Watch and run tests on changes"
	@echo "  doc           - Generate and open documentation"
	@echo "  audit         - Check for security vulnerabilities"
	@echo "  install-tools - Install useful cargo tools"
	@echo "  run P=N       - Run project N (e.g., make run P=1)"
	@echo "  test-p P=N    - Test project N"
	@echo "  help          - Show this help"

