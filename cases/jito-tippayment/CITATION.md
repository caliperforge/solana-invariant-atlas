# Cited port: cf-invariants-jito-tippayment

The atlas cites this port as a defender-side regression fixture: the
Jito tip-payment program is ported to the atlas's pinned rails
(anchor-lang 1.0.1, Crucible v0.2.0, solana-cli 2.1.21, platform-tools
v1.52), and three class-shaped invariants run against a clean reference
plus three single-hunk planted twins so that CI standing-evidences
each class's proof of catch on every push. The port carries an honest
engineering finding in its own README: SVM runtime already enforces
total-lamport conservation, so structural-state invariants are the
meaningful class here rather than lamport totals.

[![ci](https://github.com/caliperforge/cf-invariants-jito-tippayment/actions/workflows/ci.yml/badge.svg)](https://github.com/caliperforge/cf-invariants-jito-tippayment/actions/workflows/ci.yml)

**Live repository:** https://github.com/caliperforge/cf-invariants-jito-tippayment

Badge state verified 2026-07-02 (this atlas ticket's re-check consulted
the port sub-tree; the port README at `a3d6097` does not itself ship
the badge markdown at the header, so the mirrored badge above is the
canonical CI-badge URL derived from the repo's ci.yml file path). If
the badge above renders red at read time, the citation is auditably
red and the port owner is dispatched to; the atlas does not silently
carry a red-badge citation.

## Classes exercised

Round-trip to the class rows in [../../docs/coverage_map.md](../../docs/coverage_map.md):

| Invariant class (atlas family) | Invariant name (port-local) | Where (path @ commit) |
|---|---|---|
| `monotonic_accounting` (state-commit sub-shape; see [row](../../docs/coverage_map.md#class-coverage-atlas-classes-x-program-level-comparators)) | `invariant_change_tip_receiver_updates_config` | `caliperforge/cf-invariants-jito-tippayment/port/references/jito_tippay_ref/fuzz/jito_change_tip_receiver_state/src/main.rs @ a3d6097` |
| `monotonic_accounting` (state-commit sub-shape; see [row](../../docs/coverage_map.md#class-coverage-atlas-classes-x-program-level-comparators)) | `invariant_change_block_builder_updates_config` | `caliperforge/cf-invariants-jito-tippayment/port/references/jito_tippay_ref/fuzz/jito_change_block_builder_state/src/main.rs @ a3d6097` |
| `access_control` (bounds-check sub-shape; see [row](../../docs/coverage_map.md#class-coverage-atlas-classes-x-program-level-comparators)) | `invariant_block_builder_commission_pct_bound` | `caliperforge/cf-invariants-jito-tippayment/port/references/jito_tippay_ref/fuzz/jito_block_builder_commission_bounds/src/main.rs @ a3d6097` |

## Files under review at the pinned commit

The port's Anchor workspace lives under `port/` (verified against the
port sub-tree layout as of this ticket). At `a3d6097`:

- `port/references/jito_tippay_ref/programs/` (clean reference tree).
- `port/references/jito_tippay_ref_planted_change_tip_receiver_state/programs/`
  (planted twin; single-hunk diff drops the
  `ctx.accounts.config.tip_receiver = ctx.accounts.new_tip_receiver.key();`
  commit line; lamport-side effects still run, only the rotation never
  commits).
- `port/references/jito_tippay_ref_planted_change_block_builder_state/programs/`
  (planted twin; single-hunk diff drops the
  `ctx.accounts.config.block_builder = ctx.accounts.new_block_builder.key();`
  commit line).
- `port/references/jito_tippay_ref_planted_block_builder_commission/programs/`
  (planted twin; single-hunk diff drops the
  `require_gte!(100, block_builder_commission, ...)` bounds gate).
- `port/references/jito_tippay_ref/fuzz/` (fuzz-fixture entry points
  for the three invariants above; cited in the classes-exercised
  table).

## What this citation is NOT

This CITATION does not vendor any upstream Jito code into the atlas.
It does not claim discovery of a bug in upstream Jito. It does not
claim disclosure to Jito or to any team. Per the source-substrate
eval section 12.9, the atlas has zero upstream disclosures as of
2026-07-02; the port is a defender-side regression fixture that
converts a class shape into a standing proof-of-catch receipt, not a
finding.
