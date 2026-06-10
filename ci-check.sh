#!/bin/bash
# Run the same checks as CI

set -e

echo "==> Running cargo fmt --check..."
cargo fmt -- --check

echo "==> Running cargo clippy..."
cargo clippy --workspace -- -D warnings

echo "==> Running cargo build..."
cargo build --workspace --verbose

echo "==> Running cargo test..."
cargo test --workspace --verbose

echo "==> All checks passed!"
