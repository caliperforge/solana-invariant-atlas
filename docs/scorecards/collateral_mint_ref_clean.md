# Scorecard: `collateral_mint_ref` (clean twin)

> AI-disclosure banner: this scorecard was authored with AI involvement,
> disclosed at the point of use per `AI_DISCLOSURE.md` at the repo root.

## Summary

- Reference: `references/collateral_mint_ref/`
- Class: access_control, missing-Anchor-constraint family,
  mint-authority sub-shape
- Invariant: `invariant_collateral_authority`
- Emit target: Crucible v0.2.0
- Invariants total: 1
- Invariants violated: 0
- Marker (`INVARIANT VIOLATED collateral_authority`): 0 occurrences
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
| Fuzz (Crucible) | `crucible run collateral_mint_ref invariant_collateral_authority --release --timeout 30` | 0 | none |
| Deterministic regression | `cargo run --release --bin regression` | 0 | none (printed `regression: clean pass`) |
| Unit (happy path) | `cargo test --release --test happy_path` | 0 | 1 passed, 0 failed |

## Fuzz output (tail)

```
[FUZZ_PULSE] run time: 30s, clients: 1, corpus: 24, crashes: 0, executions: 6224, exec/sec: 205.8, edges: 651/3948 (16.5%), branches: 608/1974 (30.8%), actions/exec: 6.1, ok: 24228/37679 (64.3%), discovered: 2/3 actions

[FUZZ] Timeout reached (30s). Exiting gracefully.
```

Note the `discovered: 2/3 actions` line: on the clean twin, the mint-
equality constraint rejects `action_deposit_wrong_mint` at the anchor
layer, so the third action never returns `ok`. This is the expected
clean-twin signature.
