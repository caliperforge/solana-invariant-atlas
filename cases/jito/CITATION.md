# Cited port: cf-invariants-jito

The atlas cites this port as a defender-side regression fixture: the
Jito tip-distribution program is ported to the atlas's pinned rails
(anchor-lang 1.0.1, Crucible v0.2.0, solana-cli 2.1.21, platform-tools
v1.52), and four class-shaped invariants run against a clean reference
plus four single-hunk planted twins so that CI standing-evidences
each class's proof of catch on every push.

[![ci](https://github.com/caliperforge/cf-invariants-jito/actions/workflows/ci.yml/badge.svg)](https://github.com/caliperforge/cf-invariants-jito/actions/workflows/ci.yml)

**Live repository:** https://github.com/caliperforge/cf-invariants-jito

Badge state verified 2026-07-02 (this atlas ticket's re-check consulted
the port's own README, which mirrors the badge markdown verbatim; the
port README is committed at `6d01f42` in the port sub-tree). If the
badge above renders red at read time, the citation is auditably red and
the port owner is dispatched to; the atlas does not silently carry a
red-badge citation.

## Classes exercised

Round-trip to the class rows in [../../docs/coverage_map.md](../../docs/coverage_map.md):

| Invariant class (atlas family) | Invariant name (port-local) | Where (path @ commit) |
|---|---|---|
| `balance_conservation` (see [row](../../docs/coverage_map.md#class-coverage-atlas-classes-x-program-level-comparators)) | `invariant_claim_amount_conservation` | `caliperforge/cf-invariants-jito/port/references/jito_tipdist_ref/fuzz/jito_claim_conservation/src/main.rs @ 6d01f42` |
| `access_control` (replay-guard sub-shape; see [row](../../docs/coverage_map.md#class-coverage-atlas-classes-x-program-level-comparators)) | `invariant_no_double_claim` | `caliperforge/cf-invariants-jito/port/references/jito_tipdist_ref/fuzz/jito_no_double_claim/src/main.rs @ 6d01f42` |
| `access_control` (proof-verification sub-shape; see [row](../../docs/coverage_map.md#class-coverage-atlas-classes-x-program-level-comparators)) | `invariant_merkle_proof_required` | `caliperforge/cf-invariants-jito/port/references/jito_tipdist_ref/fuzz/jito_merkle_authority/src/main.rs @ 6d01f42` |
| `access_control` (config-authority sub-shape; see [row](../../docs/coverage_map.md#class-coverage-atlas-classes-x-program-level-comparators)) | `invariant_update_config_requires_authority` | `caliperforge/cf-invariants-jito/port/references/jito_tipdist_ref/fuzz/jito_admin_gating/src/main.rs @ 6d01f42` |

## Files under review at the pinned commit

The port's Anchor workspace lives under `port/` (verified against the
port sub-tree layout as of this ticket; the Jito repos historically
carried their workspace at `port/`, not at repo root). At `6d01f42`:

- `port/references/jito_tipdist_ref/programs/` (clean reference tree).
- `port/references/jito_tipdist_ref_planted_claim_conservation/programs/`
  (planted twin, single-hunk diff on `transfer_lamports`).
- `port/references/jito_tipdist_ref_planted_no_double_claim/programs/`
  (planted twin, single-hunk diff on the `claim` runtime gate).
- `port/references/jito_tipdist_ref_planted_merkle_authority/programs/`
  (planted twin, single-hunk diff on `merkle_proof::verify`).
- `port/references/jito_tipdist_ref_planted_admin_gating/programs/`
  (planted twin, single-hunk diff on `update_config` authority check).
- `port/references/jito_tipdist_ref/fuzz/` (fuzz-fixture entry points
  for the four invariants above; cited in the classes-exercised table).

## What this citation is NOT

This CITATION does not vendor any upstream Jito code into the atlas.
It does not claim discovery of a bug in upstream Jito. It does not
claim disclosure to Jito or to any team. Per the source-substrate
eval section 12.9, the atlas has zero upstream disclosures as of
2026-07-02; the port is a defender-side regression fixture that
converts a class shape into a standing proof-of-catch receipt, not a
finding.
