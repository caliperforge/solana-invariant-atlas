#!/usr/bin/env bash
# Multi-seed reachability leg for the C-A2 collateral_authority planted twin.
#
# Runs `cargo run --release --bin regression` under the planted fuzz
# crate across every seed in ci/reachability_seeds.txt (N=16 by default).
# Every seed must exit non-zero AND print at least one `INVARIANT
# VIOLATED collateral_authority` marker. This upgrades the base planted
# CI job's one-seed catch (fixed sequence, fixed amount) to a
# deterministic N-of-N certification: no false-green from a lucky
# hardcoded amount. Mirrors the Soroban-lane
# `experiments/soroban-invariant-atlas/ci/reachability_leg.sh` shape and
# the caliperforge/crypto-contributor `scripts/reachability/` canonical
# runner (Shape A per D-solana-reachability-leg-shape-2026-07-13).
#
# The planted regression crate MUST accept `REACHABILITY_SEED` and use
# it to seed both the deposit amount and the payer / user Keypair
# secret bytes via `StdRng`. See
# `references/collateral_mint_ref_planted/fuzz/collateral_mint_ref/src/bin/regression.rs`
# for the reference implementation.
#
# Emits per-seed lines + a single verdict:
#   reachability certified: yes (N/N failed as required)
#   reachability certified: no  (k/N failed; missed on seeds ...)

set -uo pipefail

summary="${GITHUB_STEP_SUMMARY:-/dev/null}"
seeds_file="${SEEDS_FILE:-ci/reachability_seeds.txt}"
planted_ref="${PLANTED_REF:-collateral_mint_ref}"
invariant_marker="${INVARIANT_MARKER:-INVARIANT VIOLATED collateral_authority}"

if [ ! -f "$seeds_file" ]; then
  echo "reachability-multi-seed: seeds file not found: $seeds_file" >&2
  exit 2
fi

fuzz_dir="references/${planted_ref}_planted/fuzz/${planted_ref}"
if [ ! -d "$fuzz_dir" ]; then
  echo "reachability-multi-seed: planted fuzz dir not found: $fuzz_dir" >&2
  exit 2
fi

seeds=$(grep -vE '^\s*(#|$)' "$seeds_file")
total=0
failed=0
missed=""

{
  echo "## Multi-seed reachability (${planted_ref}_planted)"
  echo ""
  echo "| seed | outcome | markers |"
  echo "| --- | --- | --- |"
} >>"$summary"

pushd "$fuzz_dir" >/dev/null || exit 2

for seed in $seeds; do
  total=$((total + 1))
  set +e
  out=$(REACHABILITY_SEED="$seed" cargo run --release --bin regression 2>&1)
  rc=$?
  set -e
  markers=$(printf '%s\n' "$out" | grep -c "$invariant_marker" || true)

  if [ "$rc" -ne 0 ] && [ "$markers" -gt 0 ]; then
    echo "seed $seed: FAILED as required (rc=$rc, markers=$markers)"
    echo "| \`$seed\` | failed (required) | $markers |" >>"$summary"
    failed=$((failed + 1))
  elif [ "$rc" -eq 0 ]; then
    echo "seed $seed: passed unexpectedly (rc=0). planted-twin escaped on this seed."
    echo "| \`$seed\` | ESCAPED (rc=0) | 0 |" >>"$summary"
    missed="$missed $seed"
  else
    echo "seed $seed: rc=$rc but no INVARIANT VIOLATED marker; treating as escape."
    echo "| \`$seed\` | escape (no marker, rc=$rc) | 0 |" >>"$summary"
    printf '%s\n' "$out" | tail -20
    missed="$missed $seed"
  fi
done

popd >/dev/null

echo ""
if [ "$failed" -eq "$total" ]; then
  verdict="reachability certified: yes ($failed/$total failed as required)"
  echo "$verdict"
  echo "" >>"$summary"
  echo "**$verdict**" >>"$summary"
  exit 0
fi

verdict="reachability certified: no ($failed/$total failed; missed on seeds:$missed)"
echo "$verdict"
echo "" >>"$summary"
echo "**$verdict**" >>"$summary"
exit 1
