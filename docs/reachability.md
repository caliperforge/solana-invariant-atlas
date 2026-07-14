# Native multi-seed reachability certification (C-A2 collateral_authority)

## What this fixes

The base `planted-twin-detects` CI job runs the Crucible v0.2.0 fuzzer
once per commit against the planted twin's `#[invariant_test]`. It has
never passed the CLI a `--seed`, so every run picks a fresh LibAFL RNG
seed. "One unseeded run trips the invariant" is a single-observation
receipt: the class fires within budget on ONE fuzzer seed, and any
future silent flake on that seed would go unnoticed.

The native multi-seed reachability leg (`ci/reachability_leg.sh` +
`.github/workflows/ci.yml::reachability` job) closes that gap. It
invokes `crucible run <ref> <invariant> --seed <s>` sixteen times, once
per seed in `ci/reachability_seeds.txt`, and requires the planted twin
to trip the invariant within a per-seed timeout budget on EVERY seed.
If any seed escapes (Crucible times out `rc=0` with no `INVARIANT
VIOLATED` marker), the leg fails and the doc's k/N number goes down
instead of quietly staying at 16/16.

## Shape

The leg is Solana-native: it runs the same LibAFL+LiteSVM fuzzer that
already ships the base planted-twin catch, just re-seeded 16 times.
Crucible v0.2.0's fuzz CLI (`crates/crucible-fuzz-cli`) accepts
`--seed <u64>` and threads it through to the fuzzer subprocess as the
`FUZZ_SEED` environment variable. Each seed produces a genuinely
distinct fuzzer campaign; the invariant must fire within the timeout
budget on every one.

This supersedes the earlier `Shape A` regression-bin leg (per
`D-solana-native-reachability-supersede-2026-07-14`, queued for CEO
ratification alongside this landing). The deterministic
`src/bin/regression.rs` binary is retained as a fast developer-flow
smoke test (fixed-sequence fallback that runs in seconds locally) but
the CI reachability leg no longer wraps it: the CI receipt is now the
fuzzer's own catch, not a hand-authored replay.

## Coverage

| planted class | planted crate | fuzzer | verdict |
| --- | --- | --- | --- |
| C-A2 collateral_authority | `collateral_mint_ref_planted` | Crucible v0.2.0 | see `reachability` CI job for the latest live k/N |

Uncovered on this repo today:

- `cases/z1-pyth-lazer-signature-integrity/planted/`: source not yet
  committed to git; only compiled `target/` artefacts present locally.
  Reachability cannot cover a case whose planted twin sources are not
  present. Deferred to the case's own follow-up dispatch.
- `cases/03-jito`, `cases/04-jito-priorityfee`, `cases/05-jito-tippayment`,
  `cases/06-pyth-solana-receiver`: CITATION-only cases (each ships a
  single `CITATION.md` pointing at the sibling real-target repo). No
  local planted twin. Reachability lives in the cited sibling repo's
  own CI, not here.

## Verdict source of truth

The k/N verdict recorded in `README.md` reflects the latest live CI run
on `main`. The load-bearing artefact is the `reachability` job in
`.github/workflows/ci.yml`; the job's step summary emits a per-seed
table (`seed (hex) | seed (dec) | outcome | rc | markers`) followed by
`reachability certified: yes (16/16 failed as required)` on success or
`reachability certified: no (k/N failed; missed on seeds ...)` on
partial coverage. Uncovered seeds are named explicitly so a future
harness tightening (larger budget, richer action mix) has a concrete
handle to reduce them.

## How the seeded native leg works

Per seed the leg runs:

```
crucible run <planted_ref> invariant_collateral_authority \
  --release \
  --seed <decimal-u64> \
  --timeout <T> \
  --stop-on-crash
```

The bash driver at `ci/reachability_leg.sh` converts each hex seed from
`ci/reachability_seeds.txt` to decimal (Crucible parses `--seed` as a
`u64` in the clap sense; the seed file itself stays byte-identical to
`caliperforge/crypto-contributor:scripts/reachability/seeds.txt`).
Crucible sets `FUZZ_SEED` into the fuzzer subprocess (see
`crates/crucible-fuzz-cli/src/lib.rs`), which controls the LibAFL RNG
that drives action selection, argument drawing, and state-pool ordering
for the planted twin's `#[invariant_test]` harness. `--stop-on-crash`
returns as soon as the invariant fires so the leg does not burn the
full timeout on the norm case.

A seed "passes as required" if EITHER:

- Crucible exits non-zero (`stop_on_crash` fired on the invariant), OR
- Crucible's stdout emits at least one `INVARIANT VIOLATED
  collateral_authority` marker within the timeout.

A seed "escapes" if Crucible completes the full timeout with `rc=0` and
no marker: the fuzzer failed to reach the class within budget on that
seed. Escaped seeds are named in the verdict line and in `docs/reachability.md`
so per-class budget tuning has a concrete handle.

## Merge-gate rule

No new planted twin merges to `main` unless the `reachability` job
exits green (fail-on-all-N). If a new planted twin cannot certify at
the default budget (16-seed Crucible run at
`CRUCIBLE_TIMEOUT=${env.CRUCIBLE_TIMEOUT}`s), the case owner:

1. Extends the planted twin's Crucible harness action mix in
   `src/main.rs` until the leg certifies, OR
2. Raises the `CRUCIBLE_TIMEOUT` for that job (recorded per-case in
   this doc so the budget is legible), OR
3. Documents an honest caveat in the case's README stating the k/N the
   case currently achieves and which specific seeds escape.

Any planted crate that ships a Crucible `#[invariant_test]` harness IS
picked up automatically via the `discover` job's `references/*_ref +
references/*_planted` scan; the reachability leg drops in with the
same additive diff.

## Seed set

The seed list is a fixed, deterministic mix of small integers, common
test patterns, and pseudo-random-looking bytes. It is not regenerated
per run. `ci/reachability_seeds.txt` is byte-identical to
`caliperforge/crypto-contributor:scripts/reachability/seeds.txt`; the
two files are the single source of truth for the canonical set. Bash
`printf %llu 0x<hex>` handles the full u64 range up to
`0xffffffffffffffff` cleanly.

## Reuse

The canonical scripts this leg mirrors live at
`caliperforge/crypto-contributor:scripts/reachability/` and the shape
matches `caliperforge/soroban-invariant-atlas:ci/reachability_leg.sh`
(the C-A1 Blend V2 H-01 landing). Future Solana / Anchor / Crucible
planted twins lift `ci/reachability_leg.sh`, `ci/reachability_seeds.txt`,
this doc, and the workflow reachability job verbatim; the only per-crate
knob is the `PLANTED_REF` / `INVARIANT_NAME` / `INVARIANT_MARKER` env
vars set in the workflow step.
