#!/bin/bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)/sim"

echo "--- cargo fmt --check ---"
cargo fmt -- --check

echo "--- cargo test ---"
cargo test
