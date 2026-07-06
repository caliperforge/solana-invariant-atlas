#!/usr/bin/env bash
# Run both legs against the CLEAN twin. Expect no marker; rc=0.
set -euo pipefail

CASE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

bash "$CASE_ROOT/scripts/select-twin.sh" clean

echo "run-clean: property-based leg (z1_invariant_signer_matches)"
CASE_TWIN=clean cargo test --manifest-path "$CASE_ROOT/tests/Cargo.toml" \
    --release --test z1_invariant_signer_matches -- --nocapture

echo "run-clean: deterministic regression leg (z1_signature_bypass)"
CASE_TWIN=clean cargo test --manifest-path "$CASE_ROOT/tests/Cargo.toml" \
    --release --test z1_signature_bypass -- --nocapture

echo "run-clean: both legs green (clean twin)"
