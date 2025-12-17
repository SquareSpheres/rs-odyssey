#!/bin/bash
# Development environment setup

set -e

echo "=== Rust Forge Setup ==="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust not found. Install from https://rustup.rs/"
    exit 1
fi

echo "Rust version: $(rustc --version)"

# Install useful tools
echo "Installing development tools..."
cargo install cargo-watch cargo-expand cargo-audit

echo "=== Setup Complete ==="
echo "Run 'cargo build --workspace' to build all projects"

