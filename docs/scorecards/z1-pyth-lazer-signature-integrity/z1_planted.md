# Scorecard: `z1-pyth-lazer-signature-integrity` (planted twin)

> AI-disclosure banner: this scorecard was authored with AI involvement,
> disclosed at the point of use per `AI_DISCLOSURE.md` at the atlas root.

## Summary

- Case: `cases/z1-pyth-lazer-signature-integrity/`
- Class: signature-verification integrity
- Subclass-real anchor: `pyth-network/pyth-crosschain` at commit
  `eb7f460ab8d1c73c6c8b4942891c9fe74a589121` (Apache-2.0); vendored
  under `planted/pyth-lazer-solana-contract/` with a forked-single-file
  override on `src/signature.rs` + a matching helper-signature update
  in `src/lib.rs`. Twin diff mechanically reverses the security-critical
  hunk of PR #2250.
- Invariant: `z1_verify_message_signer_matches_ed25519_program_check`
- Companion invariant: `z1_verify_message_pass_iff_ed25519_pass`
- Primary emit target: `solana-program-test` / LiteSVM
- Secondary emit target: Trident v0.12.0 (Ackee firm-shape overlap
  scaffold; runtime execution deferred per atlas pattern)
- Invariants total: 2
- Invariants violated (expected): 1 (the primary z1 invariant; the
  companion `pass_iff_ed25519_pass` continues to hold on well-formed
  input)
- Marker (`INVARIANT VIOLATED z1_verify_message_signer_matches_ed25519_program_check`):
  >=1 occurrences on the property leg AND on the deterministic
  regression
- Suggester source: n/a (hand-authored)

## Toolchain (pinned)

Same as clean scorecard. The twin difference lives in the program
crate, not the harness.

## Leg results (this pass, local reproduction pending in build engineer's CI leg)

| Leg | Command | rc (expected) | Marker (expected) |
|---|---|---|---|
| Property-based (`z1_invariant_signer_matches`) | `bash scripts/run-planted.sh` (property step) | >=1 marker; test-body panics on planted for FIRST violation | `INVARIANT VIOLATED z1_verify_message_signer_matches_ed25519_program_check` |
| Deterministic regression (`z1_signature_bypass`) | `bash scripts/run-planted.sh` (regression step) | non-zero (test-body panics after printing marker) | `INVARIANT VIOLATED z1_verify_message_signer_matches_ed25519_program_check` |
| Trident scaffold schema-check | atlas CI `planted-twin-detects` job, `trident` matrix entry | 0 (schema-check only; runtime deferred) | scaffold contains the marker string in the flow file |

## Expected planted-twin signature

- The property leg's mixed-signer draw fires the marker within the
  first few iterations. The test-body panics after printing the marker
  (`assert markers_seen >= 1`), and cargo-test exits non-zero. The
  atlas `planted-twin-detects` CI job treats marker-plus-non-zero-rc as
  the expected/desired outcome and returns 0 via the inverted-assertion
  wrapper (see `.github/workflows/ci.yml`).
- The deterministic regression prints the marker verbatim and panics
  (rc=1), matching the atlas convention.
- Trident scaffold's `src/flows/invariant_signer_matches_ed25519.rs`
  contains the marker string; twin-symmetry `diff` between the two
  twin trees' Trident scaffolds is clean.

## Class-fidelity note

The planted twin's `src/signature.rs` matches the pre-eb7f460 code
path described in Zellic Finding 3.1: the caller-supplied
`message_offset` is honored and the byte-for-byte `slice_eq` check
against the sysvar-visible instruction bytes is absent. This is a
mechanical reversal of the PR #2250 hunk, not a fabrication of a
different-shape bug.

## Honest scope

The planted twin's marker is a defender-side regression signal on our
teaching-scale harness. It does not reproduce the Zellic finding
against any deployed Pyth Lazer program. Numeric leg counts land here
after the build engineer's local reproduction on the pinned rails.
