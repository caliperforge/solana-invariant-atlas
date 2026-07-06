#!/usr/bin/env bash
# Build both twin program crates under the atlas pinned rails.
# Called by CI before either leg. Local reproduction: run this once,
# then use scripts/run-clean.sh / scripts/run-planted.sh.
set -euo pipefail

CASE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ATLAS_ROOT="$(cd "$CASE_ROOT/../.." && pwd)"
SOLANA_TOOLS_VERSION="${SOLANA_TOOLS_VERSION:-v1.52}"

echo "build-both: building clean twin at $CASE_ROOT/clean/pyth-lazer-solana-contract"
cargo build-sbf \
    --tools-version "$SOLANA_TOOLS_VERSION" \
    --manifest-path "$CASE_ROOT/clean/pyth-lazer-solana-contract/Cargo.toml"

echo "build-both: building planted twin at $CASE_ROOT/planted/pyth-lazer-solana-contract"
cargo build-sbf \
    --tools-version "$SOLANA_TOOLS_VERSION" \
    --manifest-path "$CASE_ROOT/planted/pyth-lazer-solana-contract/Cargo.toml"

echo "build-both: both twin program crates built OK"
