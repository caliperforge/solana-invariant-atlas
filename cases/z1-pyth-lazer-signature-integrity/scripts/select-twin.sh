#!/usr/bin/env bash
# Select which twin's `pyth-lazer-solana-contract` crate the `tests/`
# crate links against. Copies the overlay into place.
#
# Usage: bash scripts/select-twin.sh <clean|planted>
set -euo pipefail

TWIN="${1:-}"
if [[ "$TWIN" != "clean" && "$TWIN" != "planted" ]]; then
    echo "usage: $0 <clean|planted>" >&2
    exit 2
fi

CASE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TESTS_DIR="$CASE_ROOT/tests"

cp "$TESTS_DIR/Cargo.toml.$TWIN" "$TESTS_DIR/Cargo.toml"
echo "select-twin: selected $TWIN twin (tests/Cargo.toml now points at ../$TWIN/pyth-lazer-solana-contract)"
