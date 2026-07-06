#!/usr/bin/env bash
# Run both legs against the PLANTED twin. Expect >=1 marker; regression rc!=0.
# Inverted-assertion wrapper: this script exits 0 when the planted twin
# DID trip the marker (which is the expected/desired behavior). It exits
# non-zero only when the planted twin ran clean - a class-fidelity gate
# failure.
set -euo pipefail

CASE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="${GITHUB_WORKSPACE:-$CASE_ROOT}/ci-out"
mkdir -p "$OUT_DIR"

bash "$CASE_ROOT/scripts/select-twin.sh" planted

MARKER='INVARIANT VIOLATED z1_verify_message_signer_matches_ed25519_program_check'
OUT_PROPERTY="$OUT_DIR/z1_planted_property.out"
OUT_REGRESSION="$OUT_DIR/z1_planted_regression.out"

echo "run-planted: property-based leg (z1_invariant_signer_matches)"
set +e
CASE_TWIN=planted cargo test --manifest-path "$CASE_ROOT/tests/Cargo.toml" \
    --release --test z1_invariant_signer_matches -- --nocapture 2>&1 | tee "$OUT_PROPERTY"
rc_property=${PIPESTATUS[0]}
set -e

echo "run-planted: deterministic regression leg (z1_signature_bypass)"
set +e
CASE_TWIN=planted cargo test --manifest-path "$CASE_ROOT/tests/Cargo.toml" \
    --release --test z1_signature_bypass -- --nocapture 2>&1 | tee "$OUT_REGRESSION"
rc_regression=${PIPESTATUS[0]}
set -e

echo "---"
if ! grep -q "$MARKER" "$OUT_REGRESSION"; then
    echo "::error::planted regression did not print the z1 marker; class-fidelity gate FAIL"
    exit 1
fi
if [[ "$rc_regression" -eq 0 ]]; then
    echo "::error::planted regression exited 0; class-fidelity gate FAIL"
    exit 1
fi
if ! grep -q "$MARKER" "$OUT_PROPERTY"; then
    echo "::error::planted property leg did not print the z1 marker; class-fidelity gate FAIL"
    exit 1
fi

echo "run-planted: planted twin tripped the z1 marker on both legs (expected)."
exit 0
