# Cited port: cf-invariants-jito-priorityfee

The atlas cites this port as a defender-side regression fixture: the
Jito priority-fee-distribution program is ported to the atlas's pinned
rails (anchor-lang 1.0.1, Crucible v0.2.0, solana-cli 2.1.21,
platform-tools v1.52), and one class-shaped monotonic-accounting
invariant runs against a clean reference plus a single-hunk planted
twin so that CI standing-evidences the class's proof of catch on every
push.

[![ci](https://github.com/caliperforge/cf-invariants-jito-priorityfee/actions/workflows/ci.yml/badge.svg)](https://github.com/caliperforge/cf-invariants-jito-priorityfee/actions/workflows/ci.yml)

**Live repository:** https://github.com/caliperforge/cf-invariants-jito-priorityfee

Badge state verified 2026-07-02 (this atlas ticket's re-check consulted
the port sub-tree; the port README at `a32c1d3` does not itself ship
the badge markdown at the header, so the mirrored badge above is the
canonical CI-badge URL derived from the repo's ci.yml file path). If
the badge above renders red at read time, the citation is auditably
red and the port owner is dispatched to; the atlas does not silently
carry a red-badge citation.

## Classes exercised

Round-trip to the class rows in [../../docs/coverage_map.md](../../docs/coverage_map.md):

| Invariant class (atlas family) | Invariant name (port-local) | Where (path @ commit) |
|---|---|---|
| `monotonic_accounting` (state-commit sub-shape; see [row](../../docs/coverage_map.md#class-coverage-atlas-classes-x-program-level-comparators)) | `invariant_transfer_priority_fee_tips_increments_total` | `caliperforge/cf-invariants-jito-priorityfee/port/references/jito_pfd_ref/fuzz/jito_transfer_priority_fee_total/src/main.rs @ a32c1d3` |

## Files under review at the pinned commit

The port's Anchor workspace lives under `port/` (verified against the
port sub-tree layout as of this ticket). At `a32c1d3`:

- `port/references/jito_pfd_ref/programs/` (clean reference tree).
- `port/references/jito_pfd_ref_planted_transfer_increments_total/programs/`
  (planted twin; single-hunk diff drops the
  `PriorityFeeDistributionAccount.increment_total_lamports_transferred(lamports)?;`
  call, so the on-chain state commit of the new total never happens
  even though the system-program lamport transfer still runs; the
  runtime balance check is not tripped).
- `port/references/jito_pfd_ref/fuzz/jito_transfer_priority_fee_total/`
  (fuzz-fixture entry point; cited in the classes-exercised table).

## What this citation is NOT

This CITATION does not vendor any upstream Jito code into the atlas.
It does not claim discovery of a bug in upstream Jito. It does not
claim disclosure to Jito or to any team. Per the source-substrate
eval section 12.9, the atlas has zero upstream disclosures as of
2026-07-02; the port is a defender-side regression fixture that
converts a class shape into a standing proof-of-catch receipt, not a
finding.
