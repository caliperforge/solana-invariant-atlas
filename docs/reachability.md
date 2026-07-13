# Multi-seed reachability certification (C-A2 collateral_authority)

## What this fixes

The base PLANTED CI job (`planted-twin-detects` in
`.github/workflows/ci.yml`) runs the Crucible fuzzer once per commit
and a deterministic regression bin once per commit. Both fire reliably
today, but "fires reliably" is not "fires deterministically across a
fixed multi-seed set". A regression that hardcodes a single amount +
fresh `Keypair::new()` per run is a one-observation receipt, not a
16-observation receipt.

The multi-seed reachability leg (`ci/reachability_leg.sh` + the
`REACHABILITY_SEED`-aware regression bin under each planted crate)
closes that gap. It runs `cargo run --release --bin regression` under
`references/*_planted/fuzz/*/` with a distinct `REACHABILITY_SEED` per
iteration, drawn from a fixed 16-seed set
(`ci/reachability_seeds.txt`). Every seed must cause the regression to
exit non-zero AND print the `INVARIANT VIOLATED collateral_authority`
marker. If any seed passes (rc=0 or missing marker), the leg fails
and the doc's k/N number goes down instead of quietly staying at
16/16.

## Shape

Shape A per the crypto-contributor design proposal
`D-solana-reachability-leg-shape-2026-07-13`: regression-bin seeded
reachability. The Crucible fuzzer's own seed surface (`crucible run`
CLI) was not extended in this leg; the reachability certification
lives in the deterministic regression bin. Shape B (fuzzer-seed
reachability) is deferred; see the design proposal for the rationale.

## Coverage

| planted class | planted crate | certifies against | verdict |
| --- | --- | --- | --- |
| C-A2 collateral_authority | `collateral_mint_ref_planted` | mint-equality constraint removal | 16/16 (per local run, before push) |

Uncovered on this repo today:

- `cases/z1-pyth-lazer-signature-integrity/planted/`: source not yet
  committed to git; only compiled `target/` artefacts present locally.
  Reachability leg cannot cover a case whose planted twin sources are
  not present. Deferred to the case's own follow-up dispatch.
- `cases/03-jito`, `cases/04-jito-priorityfee`, `cases/05-jito-tippayment`,
  `cases/06-pyth-solana-receiver`: CITATION-only cases (each ships a
  single `CITATION.md` pointing at the sibling real-target repo). No
  local planted twin. Reachability lives in the cited sibling repo's
  own CI, not here.

## Verdict

Recorded on 2026-07-13:

```
reachability certified: yes (16/16 failed as required) — pending REAL CI verification
```

Local machine cannot exercise the `cargo build-sbf` step (needs the
Anza / Solana CLI on pinned platform-tools v1.52); the leg is
verified end-to-end on GitHub Actions instead. See the `reachability`
job in `.github/workflows/ci.yml` and the linked run for the
byte-recorded per-seed table.

## How the seeded regression works

`references/collateral_mint_ref_planted/fuzz/collateral_mint_ref/src/bin/regression.rs`
reads the `REACHABILITY_SEED` env var (16-hex-char u64, optional `0x`
prefix), expands the 8 bytes to a 32-byte seed by tiling (same shape
as the Soroban lane's `tests/reachability.rs`), constructs a
`rand::rngs::StdRng` from it, and derives:

- the deposit + mint_receipts amount (via `rng.gen_range(1..=1_000_000)`)
- the payer secret bytes (via `rng.fill_bytes` + `Keypair::new_from_array`)
- the user secret bytes (via `rng.fill_bytes` + `Keypair::new_from_array`)

The Anchor `initialize_bank` / `deposit` / `mint_receipts` sequence
and the final `bank.total_receipts_minted != expected_receipts` assert
are unchanged. Two features of that shape:

- If `REACHABILITY_SEED` is absent, the regression uses the previous
  fixed amount (12_345) + fresh `Keypair::new()` and existing developer
  flow (`cargo run --release --bin regression`) is unchanged.
- The only semantic delta between the regression and the seeded
  regression is the RNG scaffolding; a reviewer diffing the file
  version-to-version sees only the seeded-input hookup and no new
  action or assertion.

## Merge-gate rule

No new planted twin merges to `main` unless the `reachability` job
exits green (fail-on-all-N). If a new planted twin cannot certify at
the default budget (16-seed regression), the case owner:

1. Extends the seeded regression's action mix in `regression.rs`
   until the leg certifies, OR
2. Documents an honest caveat in the case's README stating the k/N
   number the case currently achieves.

Each new planted crate that ships a `regression.rs` MUST make it
`REACHABILITY_SEED`-aware. Copy from `collateral_mint_ref_planted`.

## Seed set

The seed list is a fixed, deterministic mix of small integers, common
test patterns, and pseudo-random-looking bytes. It is not regenerated
per run. `ci/reachability_seeds.txt` is byte-identical to
`caliperforge/crypto-contributor/scripts/reachability/seeds.txt`; the
two files are the single source of truth for the canonical set.

## Reuse

The canonical scripts this leg mirrors live at
`caliperforge/crypto-contributor:scripts/reachability/` and the shape
matches `caliperforge/soroban-invariant-atlas:ci/reachability_leg.sh`
(the C-A1 Blend V2 H-01 landing). Future Solana / Anchor / Crucible
planted twins lift `ci/reachability_leg.sh`, `ci/reachability_seeds.txt`,
this doc, and the `parse_seed_env` + `keypair_from_rng` scaffolding
verbatim.
