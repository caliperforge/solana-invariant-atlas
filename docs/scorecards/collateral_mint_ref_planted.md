# Scorecard: `collateral_mint_ref_planted` (planted twin)

> AI-disclosure banner: this scorecard was authored with AI involvement,
> disclosed at the point of use per `AI_DISCLOSURE.md` at the repo root.

## Summary

- Reference: `references/collateral_mint_ref_planted/`
- Class: access_control, missing-Anchor-constraint family,
  mint-authority sub-shape
- Invariant: `invariant_collateral_authority`
- Emit target: Crucible v0.2.0
- Invariants total: 1
- Invariants violated: >= 1 (see counterexample list below)
- Marker (`INVARIANT VIOLATED collateral_authority`): observed on
  both the fuzz and deterministic regression legs
- AI-suggested invariants in this run: 0 (fixture hand-authored on the
  existing `render_crucible_balance` ledger pattern; registry / emit
  generation of this sub-shape rides T-satlas-03)
- Suggester source: heuristic pattern (no AI at fixture time)

## Toolchain (pinned)

- anchor-lang / anchor-spl: `= 1.0.1`
- Crucible: `v0.2.0`
- solana-cli: `2.1.21` (Anza)
- platform-tools: `v1.52`
- Rust: `1.96.0` (host, from `rust-toolchain.toml`)

## Leg results (T-satlas-02 local run, 2026-07-02)

| Leg | Command | rc | Marker |
|---|---|---|---|
| Fuzz (Crucible) | `crucible run collateral_mint_ref invariant_collateral_authority --release --timeout 30` | 0 (crucible exits cleanly at timeout) | many `[FUZZ_FINDING] ... INVARIANT VIOLATED collateral_authority` lines |
| Deterministic regression | `cargo run --release --bin regression` | 1 | `INVARIANT VIOLATED collateral_authority: total_receipts_minted=12345 expected_receipts=0` |
| Unit (happy path) | `cargo test --release --test happy_path` | 0 | 1 passed (happy path still holds; the mint-equality constraint is only exercised on the wrong-mint deposit path) |

## Sample counterexamples (from the fuzz output)

```
[FUZZ_FINDING] crash:crash_ac6efcb7758dbb73 summary:INVARIANT VIOLATED collateral_authority: total_receipts_minted=7871 expected_receipts=0
[FUZZ_FINDING] crash:crash_eb6f0561dc6e3ccc summary:INVARIANT VIOLATED collateral_authority: total_receipts_minted=65536 expected_receipts=0
[FUZZ_FINDING] crash:crash_da7e9aeae66d2bdb summary:INVARIANT VIOLATED collateral_authority: total_receipts_minted=256 expected_receipts=0
[FUZZ_FINDING] crash:crash_b2e0aa4e99377fc1 summary:INVARIANT VIOLATED collateral_authority: total_receipts_minted=43495 expected_receipts=0
[FUZZ_FINDING] crash:crash_7289a84d611d96c9 summary:INVARIANT VIOLATED collateral_authority: total_receipts_minted=69589 expected_receipts=61278
```

Interpretation: on the planted twin, `action_deposit_wrong_mint`
succeeds (constraint absent), driving `bank.pending_receipts` upward
in a way the fixture ledger's `expected_pending_authorized` does not.
Subsequent `action_mint_receipts` calls then walk
`bank.total_receipts_minted` past `expected_receipts` and the invariant
snapshot trips. The fuzz finds this class of drift reliably inside the
30-second house budget.

## Deterministic regression output

```
INVARIANT VIOLATED collateral_authority: total_receipts_minted=12345 expected_receipts=0
```

The regression bin exits with rc=1 on the first invariant trip. It
detects; it does not maximize.
