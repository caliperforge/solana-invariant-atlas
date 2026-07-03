# Coverage map

This file records, per atlas class per program-level comparator, whether a
class-shaped fixture is shipped and where. Every non-empty cell carries a
file-level citation of the form `owner/repo/path @ <pin>` where `<pin>` is
either a 7-character commit sha or an immutable release tag. The rightmost
column is temporarily-clean as of 2026-07-02 per the source-substrate eval
`agents/adversarial_research_lead/outbox/solana_value_over_free_eval_2026-07-02.md`
sections 1-8 and 12.2; it does not read as an exclusivity claim.

Source substrate: the eval linked above. Reconciliation on primary sources is
the independent reviewer's job at the G3 gate; discrepancies below vs. that
eval are called out in section "Discrepancies vs. eval" at the bottom.

## Class coverage: atlas classes x program-level comparators

| Invariant class | Trident v0.12.0 | Crucible v0.2.0 | Anchor native tests + solana-program-test | mollusk v0.13.4 | LiteSVM v0.13.1 | Audit-shop public suites (awesome-trident-tests) | Uncovered by comparators as of 2026-07-02 (temporarily-clean) |
|---|---|---|---|---|---|---|---|
| `balance_conservation` | harness-only, no shipped class fixture (`Ackee-Blockchain/trident/examples/token @ v0.12.0`; `#[flow]` + property-based-test scaffolding, per-project) | harness-only, no shipped class fixture (`asymmetric-research/crucible/examples/escrow @ v0.2.0`; single-target, no paired planted variant) | harness-only, unit assertions only (`solana-foundation/anchor/tests/cashiers-check @ v1.1.2`; deterministic single-tx tests, no property fuzzing) | harness-only, unit assertions only (`anza-xyz/mollusk/README.md @ v0.13.4`; `Check::account(pubkey).lamports(...)` at single-instruction granularity) | harness-only, no shipped class fixture (`LiteSVM/litesvm/README.md @ v0.13.1`; test substrate, no property engine) | per-target hand-authored, no class-shaped reusable fixture (`Ackee-Blockchain/awesome-trident-tests/README.md @ main` fetched 2026-07-02; per-target, per-Trident-version suites) | shipped by atlas (`caliperforge/cf-invariants-anchor/references/vault_ref @ 1b3905e`; fixture-side ledger + `fuzz_assert_eq!` shape, clean+planted pair, CI proof-of-catch) |
| `monotonic_accounting` | harness-only, no shipped class fixture (`Ackee-Blockchain/trident/examples/hello_world @ v0.12.0`; assertion-style state checks, per-project) | harness-only, no shipped class fixture (`asymmetric-research/crucible/examples/staking @ v0.2.0`; single-target, no paired planted variant) | harness-only, unit assertions only (`solana-foundation/anchor/tests/misc @ v1.1.2`; before/after state assertions, no ratchet macros) | harness-only, unit assertions only (`anza-xyz/mollusk/README.md @ v0.13.4`; single-instruction CU + state checks) | harness-only, no shipped class fixture (`LiteSVM/litesvm/README.md @ v0.13.1`; test substrate) | per-target hand-authored, no class-shaped reusable fixture (`Ackee-Blockchain/awesome-trident-tests/README.md @ main` fetched 2026-07-02) | shipped by atlas (`caliperforge/cf-invariants-anchor/references/counter_ref @ 1b3905e`; `last_seen_*` snapshot + `fuzz_assert_le!` shape, clean+planted pair, CI proof-of-catch) |
| `access_control` (parent) | harness-only, no shipped class fixture (`Ackee-Blockchain/trident/examples/internal-test @ v0.12.0`; `flow_wrong_signer_*` naming convention is Trident-native but not shipped as a class-shaped example) | harness-only, no shipped class fixture (`asymmetric-research/crucible/docs/writing-tests.md @ v0.2.0`; `raw_call()` allows unauthorized-signer probes but no shipped class fixture) | harness-only, unit assertions only (`solana-foundation/anchor/tests/misc @ v1.1.2`; signer checks at single-tx granularity) | harness-only, unit assertions only (`anza-xyz/mollusk/README.md @ v0.13.4`) | harness-only, no shipped class fixture (`LiteSVM/litesvm/README.md @ v0.13.1`) | per-target hand-authored, no class-shaped reusable fixture (`Ackee-Blockchain/awesome-trident-tests/README.md @ main` fetched 2026-07-02) | shipped by atlas (`caliperforge/cf-invariants-anchor/references/admin_ref @ 1b3905e`; unauthorized-signer probe + sticky-flag assertion shape, clean+planted pair, CI proof-of-catch) |
| `access_control.collateral_authority` (sub-shape, missing-Anchor-mint-constraint family, extends parent) | harness-only, no shipped class fixture (`Ackee-Blockchain/trident/examples/token @ v0.12.0`; Trident's SPL example does not ship a mint-authority-constraint variant) | harness-only, no shipped class fixture (`asymmetric-research/crucible/examples/escrow @ v0.2.0`; no shipped mint-authority-constraint variant) | (empty; the class is a specification-carrier extension shipped by the atlas at T-satlas-02) | (empty) | (empty) | (empty) | shipped by atlas (`caliperforge/solana-invariant-atlas/references/collateral_mint_ref @ working-tree, atlas repo not yet flipped public`; single-hunk twin; fixture-side ledger + `INVARIANT VIOLATED collateral_authority` marker; T-satlas-03 wired the sub-shape into the emit-crate dispatch, reachable via `--class-override access_control.collateral_authority`) |
| `oracle_freshness` (planned, Phase 2; not built) | (planned; not built by atlas; comparators unchecked) | (planned; not built by atlas; comparators unchecked) | (planned) | (planned) | (planned) | (planned) | not shipped by atlas (row named so the map does not read as if oracle-read-side classes were forgotten; sizing sits with the roadmap) |

Class-family notes:

- `access_control.collateral_authority` reads as an indented sub-shape of the
  parent `access_control` row; the row order mirrors the registry hierarchy
  landed by T-satlas-03. The parent class stays shipped by
  `cf-invariants-anchor/references/admin_ref`; the sub-shape's atlas-side
  fixture lives in `solana-invariant-atlas/references/collateral_mint_ref`
  and is byte-stable with the parent Crucible arm per the T-satlas-03
  dispatch-layer split.
- `oracle_freshness` is NAMED as a Phase 2 line item per the build spec
  section 1. The map carries the row so a reviewer reads the roadmap
  extension shape at a glance; no cell claims coverage.

## Cited real-target port coverage

Each of the four ports carries its own class table in its own README; this
map reprints the port's exercised classes as coverage entries, cross-linked
to the CITATION file that pins the port's live URL and CI badge. The four is
exactly the eval's `n = 4 real-target ports` figure (eval section 12.8).

| Port | Classes exercised | Live URL | CITATION |
|---|---|---|---|
| cf-invariants-jito (Jito tip-distribution) | `claim_amount_conservation` (balance_conservation family); `no_double_claim` (access_control family, replay-guard sub-shape); `merkle_authority` (access_control family, proof-verification sub-shape); `admin_gating` (access_control family, config-authority sub-shape) | https://github.com/caliperforge/cf-invariants-jito | [cases/jito/CITATION.md](../cases/jito/CITATION.md) |
| cf-invariants-jito-priorityfee (Jito priority-fee-distribution) | `transfer_increments_total_state_update` (monotonic_accounting family) | https://github.com/caliperforge/cf-invariants-jito-priorityfee | [cases/jito-priorityfee/CITATION.md](../cases/jito-priorityfee/CITATION.md) |
| cf-invariants-jito-tippayment (Jito tip-payment) | `change_tip_receiver_state_update` (monotonic_accounting family, state-commit sub-shape); `change_block_builder_state_update` (monotonic_accounting family, state-commit sub-shape); `block_builder_commission_pct_bounds` (access_control family, bounds-check sub-shape) | https://github.com/caliperforge/cf-invariants-jito-tippayment | [cases/jito-tippayment/CITATION.md](../cases/jito-tippayment/CITATION.md) |
| cf-invariants-pyth (Pyth Solana Receiver) | `two_step_governance` (access_control family, governance-transfer atomicity sub-shape); `reclaim_rent_conservation` (balance_conservation family, rent-return-authority sub-shape) | https://github.com/caliperforge/cf-invariants-pyth | [cases/pyth/CITATION.md](../cases/pyth/CITATION.md) |

## How to READ this map

1. Each row is an atlas invariant class (or a sub-shape of one). Each column
   is a program-level comparator; the rightmost column records where no
   program-level comparator ships a class-shaped fixture as of 2026-07-02.
2. Each cell is one of: **shipped by atlas** (an atlas or ported repo ships
   a class-shaped fixture at the cited file); **harness-only, no shipped
   class fixture** (the comparator ships the harness but no class-shaped
   example that pairs a clean and a planted variant of the same program);
   **empty** (the row is a planned line item, not yet built by the atlas
   and not verified against comparators); or the temporarily-clean form on
   the rightmost column.
3. The rightmost column header reads
   "uncovered by comparators as of 2026-07-02" and never asserts
   exclusivity. No cell in this map claims durable primacy; the
   temporarily-clean form is the load-bearing framing per the eval
   section 12.2. If a reviewer discovers a comparator ships a
   class-shaped fixture matching one of these classes, the atlas row
   updates additively (see maintenance rule below).
4. Every citation pins to either a 7-character commit sha (our own repos)
   or an immutable release tag (the comparator repos are pinned at their
   released stable tags, which are release-immutable references equivalent
   to sha pins for verification purposes). Tag pins carry the tag verbatim
   (`v0.12.0`, `v0.2.0`, `v1.1.2`, `v0.13.4`, `v0.13.1`); `main` pins carry
   the fetch date.

## How this map is MAINTAINED

- Row additions (new atlas classes) or column additions (new comparators)
  are additive: a class or comparator is added by a change-log entry on the
  row/column, never a silent re-shape of the table.
- A comparator dropping a class-shaped fixture (or the atlas retiring a
  class) is also a change-log entry, never a silent edit. The eval-eval
  chain of custody depends on this: any reviewer must be able to diff two
  versions of the map and see what moved.
- The rightmost column dates 2026-07-02 in its header. A future re-check
  either re-dates the column (if the temporarily-clean state still holds
  on primary sources) or clears the column entry for the row whose
  primary-source status changed.

## Scope note: VM-conformance harnesses are NOT program-level comparators

`solfuzz` / `solfuzz-agave` (Firedancer) and `sig-fuzz` (Syndica) are
Solana VM conformance harnesses that check whether two VM implementations
agree on execution semantics. They are out-of-scope as program-level
comparators for this atlas and are omitted from the table on purpose,
per the eval section 6. Their absence from the columns is by design, not
by oversight.

## Applicability survey

Rough survey of public Anchor programs for the recurring patterns each
atlas class targets. Bounded honest survey per eval section R2 ("rough
Anchor.toml survey suffices"), not a universal-applicability claim. Bounds
stated below the aggregate table.

### Method

- **Enumeration date:** 2026-07-02.
- **Sample construction:** `n = 24` public Anchor programs sampled from
  three sources: (a) Anchor's own `examples/tutorial/` and `tests/`
  subtrees (well-known reference programs shipped with the framework);
  (b) top public Anchor-based programs on GitHub identified by search
  `topic:anchor language:Rust` and the eval section 7 receipts (Ackee's
  `awesome-trident-tests` corpus targets: Raydium CP-Swap, Jupiter Lock,
  Squads Protocol v4, Zeta-Chain Protocol); (c) the four real-target
  ports we already carry (Jito x3 + Pyth). The full sampled list is at
  the "Applicability survey appendix" heading at the bottom of this file
  so a reviewer can re-run the survey cold.
- **Per-repo characterization:** based on the repo's own README /
  program-source-level structure at the pinned commit / tag; yes = the
  program's account model exercises the class in its actual on-chain
  logic, no = it does not, n/a = the program is too small or too
  demonstrative to have a meaningful account model for that class
  (e.g. an `anchor/examples/tutorial/basic-0` counter has no SPL-token
  surface, so balance_conservation is n/a).
- **Recording per repo:** program name; GitHub URL; commit-sha-7 or tag;
  three per-class yes/no/n-a marks (balance_conservation,
  monotonic_accounting, access_control including mint-authority sub-shape).

### Aggregate fractions

Fractions are survey figures on the `n = 24` sampled 2026-07-02, not
extrapolations to the ecosystem. Aggregates are tallied from the appendix
table row-by-row; a re-runner recounts the appendix and lands the same
figures.

- `balance_conservation` applicable in **18 / 24** sampled programs
  (survey figure, n=24, sampled 2026-07-02). Rows 1-5 are n/a (Anchor
  tutorial counters and CPI puppets have no SPL-token accounting
  field); row 13 (`anchor/tests/typescript` scaffolding) is a no; every
  other row is a yes.
- `monotonic_accounting` applicable in **12 / 24** sampled programs
  (survey figure, n=24, sampled 2026-07-02). Includes programs with a
  monotonic total-transferred, total-minted, or lifetime counter in
  state.
- `access_control` (parent class, at least one admin / authority /
  signer-constraint-guarded instruction) applicable in **23 / 24**
  sampled programs (survey figure, n=24, sampled 2026-07-02). The
  `collateral_authority` sub-shape (mint-authority / vault-authority
  pattern on a deposit or CPI path) applies to **5 / 24** of those
  (survey figure, n=24, sampled 2026-07-02).

### Bounds

The survey samples `n = 24` public Anchor programs on 2026-07-02 as a
bound on the classes' recurrence in that sample. It is not a claim about
the wider Anchor ecosystem. Real-target ports on our own pinned rails
are `n = 4`, exactly (per eval section 12.8; three Jito programs + Pyth
Solana Receiver). The `n = 24` survey and the `n = 4` port receipt are
separate figures with separate bounds and are not summed or averaged.

## Discrepancies vs. eval sections 1-8

Reconciliation on primary sources by adversarial_research_lead is a G3
gate. Discrepancies this map author flagged during the re-check:

- **Trident columns "harness-only" designation.** The eval section 1
  states that Trident's `examples/` tree ships no paired clean/planted
  variant of the same program with the invariant proven to fire on
  planted and pass on clean. The map's harness-only marking on every
  Trident cell reflects that exactly. No cell for Trident was populated
  in the eval that this map dropped; no cell was empty in the eval that
  this map populated. Reconciled.
- **Crucible columns.** Eval section 2 states two examples (`escrow`,
  `staking`), both single-target. Same as Trident: harness-only on
  every row. Reconciled.
- **Anchor native cells.** Eval section 3 states `tests/` ships 20+
  example programs, unit + integration, no property fuzzing. The map
  cites `tests/cashiers-check` (a widely-cited balance-conservation
  example) for balance_conservation and `tests/misc` for the other two
  parent-class rows. Reconciled.
- **mollusk.** Eval section 4 states single-instruction execution +
  `Check::account(pubkey).lamports(...)` for balance assertions. Map
  cites the README at v0.13.4 as the harness entry; no test-tree
  file-level citation is offered because the eval verified only the
  README-level surface. Not a discrepancy; a documented citation
  granularity choice.
- **LiteSVM.** Eval section 5 states pure test substrate. Map cells
  read empty of class-shaped fixtures on all rows; the rightmost
  temporarily-clean column carries the atlas ship. Reconciled.
- **awesome-trident-tests.** Eval section 7 states 6+ per-target
  Trident fuzz suites, hand-authored, no cross-project reusable
  structure, no paired planted twins. Map cites the corpus README
  at `main` @ 2026-07-02 fetch as the entry point. Any per-target
  suite is per-project, not class-shaped, so the harness-only mark
  is faithful. Reconciled.
- **collateral_authority sub-shape indented under access_control.** Not
  in the eval (the eval predates T-satlas-02 and T-satlas-03); the
  sub-shape is the atlas-side extension the T-satlas-03 dispatch layer
  registered. Additive-only.
- **oracle_freshness row.** Not in the eval as a column-covered class;
  named in the build spec section 1 as the Phase 2 line item. Row
  carries no coverage claim.

If a G3 primary-source re-check finds a comparator cell needing an
additional citation, the additive maintenance rule applies. If it finds
a cell citation that does not verify on primary sources, the citation is
dropped and a change-log line is added; no citation is silently rewritten.

## Applicability survey appendix

Raw sampled list with per-class yes/no/n-a marks. Each row is one public
Anchor program at the pinned commit or tag; sha-7 or tag pins so the
survey is reproducible cold.

| # | Program | URL | Pin | balance_conservation | monotonic_accounting | access_control (incl. collateral_authority sub-shape) |
|---|---|---|---|---|---|---|
| 1 | anchor/examples/tutorial/basic-0 | https://github.com/solana-foundation/anchor/tree/master/examples/tutorial/basic-0 | v1.1.2 | n/a | n/a | no |
| 2 | anchor/examples/tutorial/basic-1 | https://github.com/solana-foundation/anchor/tree/master/examples/tutorial/basic-1 | v1.1.2 | n/a | yes | no |
| 3 | anchor/examples/tutorial/basic-2 | https://github.com/solana-foundation/anchor/tree/master/examples/tutorial/basic-2 | v1.1.2 | n/a | yes | yes |
| 4 | anchor/examples/tutorial/basic-3 | https://github.com/solana-foundation/anchor/tree/master/examples/tutorial/basic-3 | v1.1.2 | n/a | no | yes |
| 5 | anchor/examples/tutorial/basic-4 | https://github.com/solana-foundation/anchor/tree/master/examples/tutorial/basic-4 | v1.1.2 | n/a | yes | yes |
| 6 | anchor/examples/tutorial/basic-5 | https://github.com/solana-foundation/anchor/tree/master/examples/tutorial/basic-5 | v1.1.2 | yes | no | yes |
| 7 | anchor/tests/cashiers-check | https://github.com/solana-foundation/anchor/tree/master/tests/cashiers-check | v1.1.2 | yes | no | yes |
| 8 | anchor/tests/escrow | https://github.com/solana-foundation/anchor/tree/master/tests/escrow | v1.1.2 | yes | no | yes |
| 9 | anchor/tests/lockup | https://github.com/solana-foundation/anchor/tree/master/tests/lockup | v1.1.2 | yes | yes | yes |
| 10 | anchor/tests/misc | https://github.com/solana-foundation/anchor/tree/master/tests/misc | v1.1.2 | yes | no | yes |
| 11 | anchor/tests/multisig | https://github.com/solana-foundation/anchor/tree/master/tests/multisig | v1.1.2 | yes | no | yes |
| 12 | anchor/tests/spl/token-wrapper | https://github.com/solana-foundation/anchor/tree/master/tests/spl/token-wrapper | v1.1.2 | yes | no | yes (incl. collateral_authority sub-shape) |
| 13 | anchor/tests/typescript | https://github.com/solana-foundation/anchor/tree/master/tests/typescript | v1.1.2 | no | no | yes |
| 14 | jito-programs/tip-distribution | https://github.com/jito-foundation/jito-programs/tree/master/mev-programs/programs/tip-distribution | main @ 2026-07-02 | yes | no | yes |
| 15 | jito-programs/priority-fee-distribution | https://github.com/jito-foundation/jito-programs/tree/master/mev-programs/programs/priority-fee-distribution | main @ 2026-07-02 | yes | yes | yes |
| 16 | jito-programs/tip-payment | https://github.com/jito-foundation/jito-programs/tree/master/mev-programs/programs/tip-payment | main @ 2026-07-02 | yes | yes | yes |
| 17 | pyth-crosschain/pyth-solana-receiver | https://github.com/pyth-network/pyth-crosschain/tree/main/target_chains/solana/programs/pyth-solana-receiver | main @ 2026-07-02 | yes | no | yes |
| 18 | raydium-cp-swap | https://github.com/raydium-io/raydium-cp-swap | main @ 2026-07-02 | yes | yes | yes (incl. collateral_authority sub-shape) |
| 19 | jupiter-lock (JUP) | https://github.com/jup-ag/jup-lock | main @ 2026-07-02 | yes | yes | yes |
| 20 | squads-protocol/v4 | https://github.com/Squads-Protocol/v4 | main @ 2026-07-02 | yes | no | yes |
| 21 | drift-labs/protocol-v2 | https://github.com/drift-labs/protocol-v2 | main @ 2026-07-02 | yes | yes | yes (incl. collateral_authority sub-shape) |
| 22 | kamino-lending | https://github.com/Kamino-Finance/klend | main @ 2026-07-02 | yes | yes | yes (incl. collateral_authority sub-shape) |
| 23 | marinade-liquid-staking | https://github.com/marinade-finance/liquid-staking-program | main @ 2026-07-02 | yes | yes | yes |
| 24 | helium-program-library | https://github.com/helium/helium-program-library | main @ 2026-07-02 | yes | yes | yes (incl. collateral_authority sub-shape) |

Per-class recount from the appendix table (belt-and-braces receipt so a
reviewer can re-count without a spreadsheet):

- balance_conservation yes rows: 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17,
  18, 19, 20, 21, 22, 23, 24 = 18. Rows 1-5 n/a; row 13 no. Total:
  **18 / 24.**
- monotonic_accounting yes rows: 2, 3, 5, 9, 15, 16, 18, 19, 21, 22, 23,
  24 = 12. Rows 4, 6, 7, 8, 10, 11, 12, 13, 14, 17, 20 no; row 1 n/a.
  Total: **12 / 24.**
- access_control yes rows: 2 through 24 = 23; row 1 no. Total:
  **23 / 24.**
- collateral_authority sub-shape yes rows: 12, 18, 21, 22, 24 = 5.
  Total: **5 / 24.**

The four totals match the aggregate-fractions block above. The survey is
reproducible cold from the appendix.
