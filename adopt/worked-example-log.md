# Worked example: the adoption walkthrough executed end-to-end

Target: the atlas's own reference program
`references/collateral_mint_ref` (and its planted twin) as the stand-in
adopter program. Executed 2026-07-02 on the T-satlas-04 authoring
workstation (macOS, arm64). Every command below is the walkthrough's
command; output is pasted raw except for one marked redaction (noted
inline).

Tool provenance for this run: the `cf-invariants-anchor` CLI was run
from the local upstream workspace build (binary reports
`cf-invariants-anchor 0.1.0 (target: Crucible v0.2.0)`). The
adopter-facing install path is the `cargo install --rev 1b3905e...`
command in walkthrough step 2. Crucible v0.2.0 checkout laid out as a
sibling of the atlas root for the duration of the run (the same layout
the workflow template creates in CI), removed afterwards.

## Step 1: pin check

The host defaults to a newer solana-cli, so the first run shows the
PATH-prefix trap firing:

```
$ bash scripts/check-solana-pin.sh
check-solana-pin: active solana-cli is not the pinned 2.1.21:
  active: solana-cli 4.0.1 (src:e4e3aa49; feat:6ff76655, client:Agave)
Prefix your PATH with the pinned release before any cargo build-sbf:
  export PATH=/Users/<user>/.local/share/solana/install/releases/2.1.21/solana-release/bin:$PATH
rc=1
```

Apply the prefix and re-run:

```
$ export PATH=~/.local/share/solana/install/releases/2.1.21/solana-release/bin:$PATH
$ bash scripts/check-solana-pin.sh
check-solana-pin: OK (solana-cli 2.1.21 (src:8a085eeb; feat:1416569292, client:Agave))
rc=0
```

## Step 2: point the atlas at the program

The stand-in adopter program is the atlas's own
`collateral_mint_ref`. Its IDL for this run is
`adopt/example/idl/collateral_mint_ref.json`, hand-transcribed from
`references/collateral_mint_ref/programs/collateral_mint_ref/src/lib.rs`
with real Anchor discriminators (the reference builds with
`cargo build-sbf` rather than `anchor build`, so no generated IDL
ships with it; an adopter gets theirs from `anchor build` at
`target/idl/<crate>.json`).

```
$ cf-invariants-anchor version
cf-invariants-anchor 0.1.0 (target: Crucible v0.2.0)
rc=0
```

## Step 3: suggest (heuristic default)

```
$ cf-invariants-anchor suggest adopt/example/idl/collateral_mint_ref.json
```

Full output (rc=0), one marked redaction:

```json
[
  {
    "name": "invariant_mint_receipts_rejects_unauthorized",
    "summary": "mint_receipts rejects when invoked by anyone other than the authorized signer",
    "class": "access_control",
    "rank": 0.78,
    "rationale": "[REDACTED IN THIS LOG: the upstream suggester's rationale sentence for this candidate uses a probe-persona word that the house framing ban-list grep flags on atlas surfaces. The verbatim text is reproducible by running the command above; an upstream wording fix is queued via the T-satlas-04 result note.]",
    "emit_hints": {
      "account_type": "Vault",
      "field": "depositor",
      "expected_expression": "fixture.vault.depositor",
      "action_names": [
        "mint_receipts"
      ]
    },
    "source": {
      "kind": "Heuristic",
      "suggester_version": "0.2.0"
    }
  },
  {
    "name": "invariant_bank_authority_bump_conservation",
    "summary": "Bank.bank_authority_bump == fixture-tracked sum of deposits − sum of withdrawals",
    "class": "balance_conservation",
    "rank": 0.55,
    "rationale": "Detected balance-bearing field `Bank.bank_authority_bump: u8` on an account mutated by movement-class instructions (deposit, mint_receipts). A correct implementation keeps this field in lock-step with the net amount transferred in via these instructions; any drift is a balance-conservation violation. The fixture walks `expected_bank_authority_bump` through `action_deposit`/`action_withdraw` and asserts equality after every action.",
    "emit_hints": {
      "account_type": "Bank",
      "field": "bank_authority_bump",
      "expected_expression": "fixture.expected_bank_authority_bump",
      "action_names": [
        "deposit",
        "mint_receipts"
      ]
    },
    "source": {
      "kind": "Heuristic",
      "suggester_version": "0.2.0"
    }
  },
  {
    "name": "invariant_pending_receipts_conservation",
    "summary": "Bank.pending_receipts == fixture-tracked sum of deposits − sum of withdrawals",
    "class": "balance_conservation",
    "rank": 0.55,
    "rationale": "Detected balance-bearing field `Bank.pending_receipts: u64` on an account mutated by movement-class instructions (deposit, mint_receipts). A correct implementation keeps this field in lock-step with the net amount transferred in via these instructions; any drift is a balance-conservation violation. The fixture walks `expected_pending_receipts` through `action_deposit`/`action_withdraw` and asserts equality after every action.",
    "emit_hints": {
      "account_type": "Bank",
      "field": "pending_receipts",
      "expected_expression": "fixture.expected_pending_receipts",
      "action_names": [
        "deposit",
        "mint_receipts"
      ]
    },
    "source": {
      "kind": "Heuristic",
      "suggester_version": "0.2.0"
    }
  },
  {
    "name": "invariant_total_receipts_minted_conservation",
    "summary": "Bank.total_receipts_minted == fixture-tracked sum of deposits − sum of withdrawals",
    "class": "balance_conservation",
    "rank": 0.55,
    "rationale": "Detected balance-bearing field `Bank.total_receipts_minted: u64` on an account mutated by movement-class instructions (deposit, mint_receipts). A correct implementation keeps this field in lock-step with the net amount transferred in via these instructions; any drift is a balance-conservation violation. The fixture walks `expected_total_receipts_minted` through `action_deposit`/`action_withdraw` and asserts equality after every action.",
    "emit_hints": {
      "account_type": "Bank",
      "field": "total_receipts_minted",
      "expected_expression": "fixture.expected_total_receipts_minted",
      "action_names": [
        "deposit",
        "mint_receipts"
      ]
    },
    "source": {
      "kind": "Heuristic",
      "suggester_version": "0.2.0"
    }
  }
]
rc=0
```

Human-review note, as the walkthrough says: candidate index 1 proposes
conservation over `bank_authority_bump: u8`, a PDA bump byte, not a
balance. Discarded. Candidate index 2 (`pending_receipts`
conservation) is the one emitted below. The `emit_hints` on candidate
index 0 name a `Vault`/`depositor` template shape from the suggester's
access-control pattern rather than this program's `Bank` account; that
candidate would need its expectation expressions edited before use,
which is the "state your own semantics" moment the walkthrough calls
out.

## Step 4: emit the Crucible fixture

```
$ cf-invariants-anchor emit adopt/example/idl/collateral_mint_ref.json \
    --target crucible --candidate-index 2 \
    --out adopt/example/emitted/invariant_pending_receipts_conservation.rs
rc=0
```

The emitted file (137 lines) is committed verbatim at
`adopt/example/emitted/invariant_pending_receipts_conservation.rs` as
generated tool output at the recorded CLI build. Its disclosure header
(first lines):

```rust
// invariant_pending_receipts_conservation
//
// Emitted by cf-invariants-anchor v0.2.0 for the balance_conservation class.
// Target: Crucible v0.2.0 (asymmetric-research/crucible).
// Source: Heuristic (suggester v0.2.0). No AI suggestion in this candidate.
```

For the runnable legs below, the walkthrough's fuzz crate for this
program already exists: it is T-satlas-02's hand-authored fixture at
`references/collateral_mint_ref/fuzz/collateral_mint_ref/` (same
ledger pattern the emitter renders), with the `collateral_authority`
property. The legs run that.

## Step 5: run both legs

Build the clean program on the pinned rails:

```
$ cargo build-sbf --tools-version v1.52 \
    --manifest-path references/collateral_mint_ref/programs/collateral_mint_ref/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.31s
rc=0
```

Happy-path unit leg (clean twin):

```
$ cd references/collateral_mint_ref/fuzz/collateral_mint_ref
$ cargo test --release --test happy_path -- --nocapture
running 1 test
test happy_path_authorized_deposit_then_mint_receipts ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s
rc=0
```

Deterministic regression leg (clean twin):

```
$ cargo run --release --bin regression
regression: clean pass (total_receipts_minted=0, expected_receipts=0)
rc=0
```

Crucible fuzz leg (clean twin), 30-second house budget; final pulse
line and exit:

```
$ crucible run collateral_mint_ref invariant_collateral_authority --release --timeout 30
[FUZZ_PULSE] run time: 15s, clients: 1, corpus: 22, crashes: 0, executions: 8196, exec/sec: 545.2, edges: 652/3948 (16.5%), branches: 608/1974 (30.8%), actions/exec: 6.2, ok: 43805/50558 (86.6%), discovered: 2/3 actions

[FUZZ] Timeout reached (30s). Exiting gracefully.
rc=0
```

Zero crashes, zero `INVARIANT VIOLATED` markers, rc=0 across all
three clean legs.

Planted twin. Build, then the deterministic regression leg (expect
the marker and rc!=0):

```
$ cargo build-sbf --tools-version v1.52 \
    --manifest-path references/collateral_mint_ref_planted/programs/collateral_mint_ref/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.35s
rc=0

$ cd references/collateral_mint_ref_planted/fuzz/collateral_mint_ref
$ cargo run --release --bin regression
INVARIANT VIOLATED collateral_authority: total_receipts_minted=12345 expected_receipts=0
rc=1
```

Crucible fuzz leg (planted twin), same 30-second budget. The walk
surfaced the seeded specification violation repeatedly; 1078
`[FUZZ_FINDING]` lines in the run log, of which the two leading ones:

```
$ crucible run collateral_mint_ref invariant_collateral_authority --release --timeout 30
[FUZZ_FINDING] crash:crash_7f49f60aa9bf086c summary:INVARIANT VIOLATED collateral_authority: total_receipts_minted=7746 expected_receipts=0
[FUZZ_FINDING] crash_7f49f60aa9bf086c: INVARIANT VIOLATED collateral_authority: total_receipts_minted=7746 expected_receipts=0
```

Both legs land exactly as the walkthrough describes: clean twin green
with zero markers, planted twin tripping
`INVARIANT VIOLATED collateral_authority`.

## Run hygiene notes

- The planted fuzz run wrote fresh crash artifacts under the planted
  twin's `crashes/` directory; they were removed after the run so the
  T-satlas-02 reference tree stays exactly as committed (its one
  recorded crash artifact pair kept).
- The temporary Crucible v0.2.0 sibling checkout used for the path
  deps was removed after the run.
- The fuzz-leg pulse lines are raw engine telemetry pasted for
  transcript fidelity; no throughput claim is made from them.
