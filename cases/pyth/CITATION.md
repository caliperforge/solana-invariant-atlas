# Cited port: cf-invariants-pyth

The atlas cites this port as a defender-side regression fixture: the
Pyth Solana Receiver program is ported to the atlas's pinned rails
(anchor-lang 1.0.1, Crucible v0.2.0, solana-cli 2.1.21, platform-tools
v1.52), and two class-shaped invariants run against a clean reference
plus two single-hunk planted twins so that CI standing-evidences each
class's proof of catch on every push. A third class was retired in
Phase 3 per `D-cf-invariants-pyth-phase3-rescope-2026-06-04 = A`
because its planted variant required a guardian-signing surface
outside the port pattern; the retirement is recorded in the port's own
history for transparency.

[![ci](https://github.com/caliperforge/cf-invariants-pyth/actions/workflows/ci.yml/badge.svg)](https://github.com/caliperforge/cf-invariants-pyth/actions/workflows/ci.yml)

**Live repository:** https://github.com/caliperforge/cf-invariants-pyth

Badge state verified 2026-07-02 (the port README ships the badge
markdown at its own header at commit `6fe8285`; the mirrored badge
above is byte-identical to what the port README already carries). If
the badge above renders red at read time, the citation is auditably
red and the port owner is dispatched to; the atlas does not silently
carry a red-badge citation.

## Classes exercised

Round-trip to the class rows in [../../docs/coverage_map.md](../../docs/coverage_map.md):

| Invariant class (atlas family) | Invariant name (port-local) | Where (path @ commit) |
|---|---|---|
| `access_control` (governance-transfer atomicity sub-shape; see [row](../../docs/coverage_map.md#class-coverage-atlas-classes-x-program-level-comparators)) | `invariant_two_step_governance_atomic` | `caliperforge/cf-invariants-pyth/references/pyth_receiver_ref/fuzz/pyth_two_step_governance/src/main.rs @ 6fe8285` |
| `balance_conservation` (rent-return-authority sub-shape; see [row](../../docs/coverage_map.md#class-coverage-atlas-classes-x-program-level-comparators)) | `invariant_reclaim_rent_returns_to_write_authority` | `caliperforge/cf-invariants-pyth/references/pyth_receiver_ref/fuzz/pyth_reclaim_rent_conservation/src/main.rs @ 6fe8285` |

## Files under review at the pinned commit

The port's Anchor workspace lives at repo root (not under `port/`; the
port README is at repo root). At `6fe8285`:

- `references/pyth_receiver_ref/programs/` (clean reference tree).
- `references/pyth_receiver_ref_planted_two_step_governance/programs/`
  (planted twin; single-hunk diff drops the
  `target_governance_authority = None` reset in
  `accept_governance_authority_transfer`).
- `references/pyth_receiver_ref_planted_reclaim_rent_conservation/programs/`
  (planted twin; single-hunk diff drops the
  `constraint = price_update_account.write_authority == payer.key()`
  check on `ReclaimRent`).
- `references/pyth_receiver_ref/fuzz/pyth_two_step_governance/src/main.rs`
  and `references/pyth_receiver_ref/fuzz/pyth_reclaim_rent_conservation/src/main.rs`
  (fuzz-fixture entry points; cited in the classes-exercised table).
- `findings/ai_suggester_run_2026-06-04/` (captured AI-suggester run
  referenced in the source-substrate eval section 0; a demo of the
  suggester layer, not a claim of lift).

## What this citation is NOT

This CITATION does not vendor any upstream Pyth code into the atlas.
It does not claim discovery of a bug in upstream Pyth. It does not
claim disclosure to Pyth or to any team. Per the source-substrate
eval section 12.9, the atlas has zero upstream disclosures as of
2026-07-02; the port is a defender-side regression fixture that
converts a class shape into a standing proof-of-catch receipt, not a
finding.
