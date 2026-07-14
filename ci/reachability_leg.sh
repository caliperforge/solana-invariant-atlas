#!/usr/bin/env bash
# Native multi-seed reachability leg for the C-A2 collateral_authority planted twin.
#
# For each seed in ci/reachability_seeds.txt (N=16 by default) runs the
# Crucible v0.2.0 fuzzer against the planted twin's own
# `#[invariant_test]` under `references/*_planted/fuzz/`, requiring the
# invariant to trip within the per-seed timeout budget. This is
# Solana-native: the same LibAFL+LiteSVM fuzzer that discovers invariant
# violations on the base `planted-twin-detects` job, rerun with a
# distinct fuzzer RNG seed for each of 16 canonical seeds. A seed
# "passes as required" if EITHER Crucible exits non-zero (crash caught
# with --stop-on-crash) OR its stdout emits at least one
# `INVARIANT VIOLATED collateral_authority` / `FUZZ_FINDING` marker.
#
# Crucible v0.2.0's fuzz-CLI accepts `--seed <u64>` and threads it into
# the fuzzer subprocess as `FUZZ_SEED` (verified at
# `crates/crucible-fuzz-cli/src/lib.rs`). Since Crucible parses the
# argument as a decimal u64, this script converts each 16-hex-char seed
# from ci/reachability_seeds.txt to decimal before passing it through.
# The seed file itself is byte-identical to
# `caliperforge/crypto-contributor:scripts/reachability/seeds.txt` and
# stays that way; the conversion is a runtime shape change only.
#
# The verdict shape matches the Foundry / Rust (proptest) reachability
# legs on their respective ecosystems:
#   reachability certified: yes (N/N failed as required)
#   reachability certified: no  (k/N failed; missed on seeds ...)
#
# Environment knobs:
#   SEEDS_FILE           path to the seeds list (default ci/reachability_seeds.txt)
#   PLANTED_REF          reference name (default collateral_mint_ref); the planted crate
#                        is expected under references/${PLANTED_REF}_planted/fuzz/${PLANTED_REF}
#   INVARIANT_NAME       Crucible invariant / test name (default invariant_collateral_authority)
#   INVARIANT_MARKER     stdout marker for the class violation
#                        (default "INVARIANT VIOLATED collateral_authority")
#   CRUCIBLE_TIMEOUT     per-seed Crucible timeout in seconds (default 30)
#   CRUCIBLE_EXTRA_ARGS  extra flags appended to `crucible run` (default empty)

set -uo pipefail

summary="${GITHUB_STEP_SUMMARY:-/dev/null}"
seeds_file="${SEEDS_FILE:-ci/reachability_seeds.txt}"
planted_ref="${PLANTED_REF:-collateral_mint_ref}"
invariant_name="${INVARIANT_NAME:-invariant_collateral_authority}"
invariant_marker="${INVARIANT_MARKER:-INVARIANT VIOLATED collateral_authority}"
crucible_timeout="${CRUCIBLE_TIMEOUT:-30}"
crucible_extra_args="${CRUCIBLE_EXTRA_ARGS:-}"

if [ ! -f "$seeds_file" ]; then
  echo "reachability-multi-seed: seeds file not found: $seeds_file" >&2
  exit 2
fi

fuzz_dir="references/${planted_ref}_planted/fuzz/${planted_ref}"
if [ ! -d "$fuzz_dir" ]; then
  echo "reachability-multi-seed: planted fuzz dir not found: $fuzz_dir" >&2
  exit 2
fi

if ! command -v crucible >/dev/null 2>&1; then
  echo "reachability-multi-seed: 'crucible' not on PATH; install via 'cargo install --path crates/crucible-fuzz-cli' from the sibling asymmetric-research/crucible checkout." >&2
  exit 2
fi

seeds_raw=$(grep -vE '^\s*(#|$)' "$seeds_file")
total=0
failed=0
missed=""

{
  echo "## Native multi-seed reachability (Crucible v0.2.0; ${planted_ref}_planted)"
  echo ""
  echo "invariant: \`${invariant_name}\`; per-seed timeout: ${crucible_timeout}s"
  echo ""
  echo "| seed (hex) | seed (dec) | outcome | rc | markers |"
  echo "| --- | --- | --- | --- | --- |"
} >>"$summary"

pushd "$fuzz_dir" >/dev/null || exit 2

# Convert hex seed (with or without 0x prefix) to decimal for Crucible's u64 flag.
# The seed file uses 16-hex-char u64 values (matching the Foundry / Rust legs);
# printf handles 0xffffffffffffffff to u64::MAX cleanly on bash 4+.
hex_to_dec() {
  local raw="${1#0x}"
  raw="${raw#0X}"
  printf '%llu' "0x${raw}"
}

for seed_hex in $seeds_raw; do
  total=$((total + 1))
  seed_dec=$(hex_to_dec "$seed_hex")

  set +e
  # shellcheck disable=SC2086
  out=$(crucible run "${planted_ref}" "${invariant_name}" \
        --release \
        --seed "${seed_dec}" \
        --timeout "${crucible_timeout}" \
        --stop-on-crash \
        ${crucible_extra_args} 2>&1)
  rc=$?
  set -e

  markers=$(printf '%s\n' "$out" | grep -c "$invariant_marker" || true)

  if [ "$markers" -gt 0 ] || [ "$rc" -ne 0 ]; then
    echo "seed $seed_hex ($seed_dec): FAILED as required (rc=$rc, markers=$markers)"
    echo "| \`$seed_hex\` | \`$seed_dec\` | failed (required) | $rc | $markers |" >>"$summary"
    failed=$((failed + 1))
  else
    echo "seed $seed_hex ($seed_dec): ESCAPED (rc=0, no marker). Crucible timed out ${crucible_timeout}s without tripping ${invariant_name}."
    echo "| \`$seed_hex\` | \`$seed_dec\` | ESCAPED (rc=0) | 0 | 0 |" >>"$summary"
    printf '%s\n' "$out" | tail -20
    missed="$missed $seed_hex"
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
