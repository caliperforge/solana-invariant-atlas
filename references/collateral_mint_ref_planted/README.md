# Reference: `collateral_mint_ref{,_planted}`

> AI-disclosure banner: this reference (source code, this README, and the
> paired scorecards) was authored with AI involvement, disclosed at the
> point of use per `AI_DISCLOSURE.md` at the repo root.

## What class this is

Missing-Anchor-constraint family, mint-authority sub-shape (extends the
existing `access_control` class). The clean twin enforces a mint-equality
constraint on a deposit's collateral token account; the planted twin
drops that one constraint line and nothing else.

## Why this class matters

Historical postmortems document real Anchor-family Solana programs
where the collateral / printer-account mint was accepted without a
mint-equality constraint on the deposit path; the Cashio incident is
the widely-cited example, cited here once for motivation only. The
actionable specification below stands on its own.

## The synthetic program

Three-instruction bank on the twin, entirely our own code:

- `initialize_bank(collateral_mint, receipt_mint)` - Bank config PDA
  records the authorized collateral mint, the receipt mint, a
  bank-authority bump, plus the pending / total counters.
- `deposit(amount)` - CPI'd SPL transfer of the user's collateral
  token account into a bank-owned token vault, then walks
  `bank.pending_receipts`.
- `mint_receipts(amount)` - bank-authority-signed CPI to mint receipt
  tokens 1:1 against pending, moving the amount from
  `bank.pending_receipts` into `bank.total_receipts_minted`.

## The single-hunk twin diff

`diff -r -x target -x crashes -x Cargo.lock references/collateral_mint_ref references/collateral_mint_ref_planted`:

```diff
--- references/collateral_mint_ref/programs/collateral_mint_ref/src/lib.rs
+++ references/collateral_mint_ref_planted/programs/collateral_mint_ref/src/lib.rs
@@
     // Clean: the mint-equality constraint binds `collateral_account`
     // to the collateral mint the bank was initialized with. The
     // planted twin drops this line; that is the entire diff.
-    #[account(
-        mut,
-        constraint = collateral_account.mint == bank.collateral_mint @ BankError::WrongCollateralMint,
-    )]
+    #[account(mut)]
     pub collateral_account: Account<'info, TokenAccount>,
```

Everything else - Cargo.toml, fuzz crate, tests, program IDs, IDL
surface - is byte-identical between the twins.

## Property under test: `collateral_authority`

After every action the fuzz walk takes, the on-chain
`bank.total_receipts_minted` must equal a fixture-side ledger
`expected_receipts` that only increments on `mint_receipts` calls
backed by pending balance that traces back to authorized-mint deposits.

Fixture-side ledger (single field, walked in lock-step with the bank):

- `expected_pending_authorized: u128` - pending balance the bank
  should recognize, per the fixture, that traces back to
  authorized-mint deposits.
- `expected_receipts: u128` - total receipts the bank should have
  minted given only authorized-mint deposits.

Snapshot after each action:

```
bank.total_receipts_minted == expected_receipts
```

Marker on drift: `INVARIANT VIOLATED collateral_authority` (carried in
the Crucible `[FUZZ_FINDING]` summary line and printed verbatim by the
deterministic regression binary).

## How the fuzz walk exercises the wrong-mint path

Fixture setup creates two SPL mints - one the bank was initialized with
(`authorized_mint`) and one it never authorized (`unauthorized_mint`) -
plus the corresponding user token accounts and bank-owned vaults. Three
generic-to-class actions:

- `action_deposit_authorized(amount)` - authorized-mint deposit, walks
  `expected_pending_authorized`.
- `action_deposit_wrong_mint(amount)` - deposit from the unauthorized
  mint, does NOT walk the fixture ledger.
- `action_mint_receipts(amount)` - mints receipts; the fixture credits
  only the portion backed by `expected_pending_authorized`.

Handler naming stays inside the `wrong_mint` / `unauthorized`
vocabulary (per the atlas framing discipline). No adversary persona,
no gain computation, no extraction ledger; handlers are generic to the
class.

## The three legs

1. **Fuzz leg** (`crucible run collateral_mint_ref
   invariant_collateral_authority --release --timeout 30`) - Crucible
   drives the three fuzz actions and snapshots the invariant after
   each. Clean: zero markers, rc=0. Planted: many `[FUZZ_FINDING]`
   lines carrying `INVARIANT VIOLATED collateral_authority`.
2. **Deterministic regression leg** (`cargo run --release --bin
   regression`) - plays the fixed sequence: initialize, deposit drawn
   from the unauthorized second mint, mint_receipts. Clean: prints
   `regression: clean pass`, rc=0. Planted: prints
   `INVARIANT VIOLATED collateral_authority ...`, rc=1. Stops at the
   FIRST invariant violation; it detects, it does not maximize.
3. **Unit leg** (`cargo test --release --test happy_path`) - the
   clean-twin happy path: initialize, authorized-mint deposit,
   mint_receipts, assert the bank's counters land where expected. Runs
   on both twins (fuzz crate is byte-identical) but the acceptance
   criterion only requires clean-twin pass.

## Honest scope

- The twin is a specification carrier for the mint-authority
  sub-shape. It does not model any specific real protocol's
  business-logic surface beyond the constraint under test.
- The invariant class is the missing-Anchor-constraint family, not a
  claim about all access-control bugs. Coverage-map extension is
  additive (see `docs/coverage_map.md`).
- The fuzz walk finds the class violation reliably inside the 30-second
  house budget; no engine or throughput claim is implied by the run.
- Registry / emit generation of this sub-shape rides T-satlas-03; the
  T-satlas-02 fixture is hand-authored on the existing
  `render_crucible_balance` ledger pattern from
  `cf-invariants-anchor-emit`.

## Reproduce locally

Pinned rails (build spec section 3; `docs/toolchain.md`): anchor-lang
`= 1.0.1`, anchor-spl `= 1.0.1`, Crucible v0.2.0, solana-cli 2.1.21,
platform-tools v1.52.

```sh
# From the repo root, with the pinned solana-cli on PATH:
export PATH=~/.local/share/solana/install/releases/2.1.21/solana-release/bin:$PATH
bash scripts/check-solana-pin.sh

# Build the clean twin program.
cargo build-sbf \
  --tools-version v1.52 \
  --manifest-path references/collateral_mint_ref/programs/collateral_mint_ref/Cargo.toml

# Unit leg on the clean twin.
(cd references/collateral_mint_ref/fuzz/collateral_mint_ref \
 && cargo test --release --test happy_path -- --nocapture)

# Deterministic regression leg on the clean twin.
(cd references/collateral_mint_ref/fuzz/collateral_mint_ref \
 && cargo run --release --bin regression)

# Fuzz leg on the clean twin.
(cd references/collateral_mint_ref/fuzz/collateral_mint_ref \
 && crucible run collateral_mint_ref invariant_collateral_authority \
      --release --timeout 30)

# Build the planted twin program and re-run the fuzz + regression legs
# against it. Expect [FUZZ_FINDING] and INVARIANT VIOLATED
# collateral_authority markers with the regression bin exiting non-zero.
cargo build-sbf \
  --tools-version v1.52 \
  --manifest-path references/collateral_mint_ref_planted/programs/collateral_mint_ref/Cargo.toml

(cd references/collateral_mint_ref_planted/fuzz/collateral_mint_ref \
 && cargo run --release --bin regression)

(cd references/collateral_mint_ref_planted/fuzz/collateral_mint_ref \
 && crucible run collateral_mint_ref invariant_collateral_authority \
      --release --timeout 30)
```

## Paired scorecards

- `docs/scorecards/collateral_mint_ref_clean.md` - clean twin
- `docs/scorecards/collateral_mint_ref_planted.md` - planted twin
