#!/bin/bash
# Run all tests with coverage

set -e

echo "=== Running all tests ==="
cargo test --workspace --verbose

echo "=== Running clippy ==="
cargo clippy --workspace -- -D warnings

echo "=== Checking formatting ==="
cargo fmt --all -- --check

echo "=== All checks passed ==="

