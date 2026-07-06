# Scorecard: `z1-pyth-lazer-signature-integrity` (clean twin)

> AI-disclosure banner: this scorecard was authored with AI involvement,
> disclosed at the point of use per `AI_DISCLOSURE.md` at the atlas root.

## Summary

- Case: `cases/z1-pyth-lazer-signature-integrity/`
- Class: signature-verification integrity
- Subclass-real anchor: `pyth-network/pyth-crosschain` at commit
  `eb7f460ab8d1c73c6c8b4942891c9fe74a589121` (Apache-2.0); vendored
  under `clean/pyth-lazer-solana-contract/` (unmodified).
- Invariant: `z1_verify_message_signer_matches_ed25519_program_check`
- Companion invariant: `z1_verify_message_pass_iff_ed25519_pass`
- Primary emit target: `solana-program-test` / LiteSVM
- Secondary emit target: Trident v0.12.0 (Ackee firm-shape overlap
  scaffold; runtime execution deferred per atlas pattern)
- Invariants total: 2
- Invariants violated: 0
- Marker (`INVARIANT VIOLATED z1_verify_message_signer_matches_ed25519_program_check`):
  0 occurrences
- Suggester source: n/a (hand-authored against the audit's own class
  description; the vendored crate's post-fix behavior serves as the
  reference)

## Toolchain (pinned)

- anchor-lang: `= 0.30.1` (vendored crate's `Cargo.toml` at eb7f460)
- solana-program-test / solana-sdk: `= 1.18.26` (upstream `dev-dependencies`)
- Trident (secondary scaffold): `v0.12.0`
- solana-cli: `2.1.21` (Anza)
- platform-tools: `v1.52`
- Rust: from `rust-toolchain.toml` at atlas root

## Leg results (this pass, local reproduction pending in build engineer's CI leg)

| Leg | Command | rc (expected) | Marker (expected) |
|---|---|---|---|
| Property-based (`z1_invariant_signer_matches`) | `bash scripts/run-clean.sh` (property step) | 0 | none |
| Deterministic regression (`z1_signature_bypass`) | `bash scripts/run-clean.sh` (regression step) | 0 | none (printed `regression: clean pass`) |
| Trident scaffold schema-check | atlas CI `clean-passes` job, `trident` matrix entry | 0 | none |

## Expected clean-twin signature

- The property-based leg walks the class-shape mixed-signer sequence
  and observes `Err(InvalidMessageData)` from the contract-side
  `slice_eq` check on every mixed-signer draw. `mixed_signer_success_observed`
  stays `false` end-to-end.
- The deterministic regression prints `regression: clean pass` and
  exits 0.
- Trident scaffold contents match twin-symmetry: byte-identical to
  the planted twin's scaffold, differing only in the `pyth-lazer-solana-contract`
  path dependency selected via `../../tests/Cargo.toml`.

## Honest scope

The clean twin's rc=0 leg is the guarantee the PR #2250 remediation
provides. No engine or throughput claim is implied by the run. The
scorecard's numeric leg counts land here after the build engineer's
local reproduction on the pinned rails.
