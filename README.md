# solana-invariant-atlas

[![clean-passes](https://github.com/caliperforge/solana-invariant-atlas/actions/workflows/ci.yml/badge.svg)](https://github.com/caliperforge/solana-invariant-atlas/actions/workflows/ci.yml)

## Reachability verdict

Native multi-seed reachability certification via the Solana-native
Crucible v0.2.0 fuzzer. Sixteen distinct `crucible run --seed <s>`
campaigns per push (`ci/reachability_leg.sh`); each seed drives a
distinct LibAFL+LiteSVM fuzzer campaign against the planted twin's
`#[invariant_test]`. The base planted CI job runs one unseeded campaign
per commit; this leg runs all 16 seeded campaigns on every push.
Merge-gate rule: fail-on-any-clean-seed does-not-merge. The k/N cell
below reflects the latest live CI run on `main`; see `docs/reachability.md`
for the shape + budget knobs.

| planted class | planted crate | fuzzer | k/N certified |
| --- | --- | --- | --- |
| C-A2 collateral_authority | `collateral_mint_ref_planted` | Crucible v0.2.0 | see live `reachability` CI job |

Uncovered on this repo today: `cases/z1-pyth-lazer-signature-integrity`
(source not committed yet); the four `cases/*/CITATION.md` cases
(reachability lives in the cited sibling repos: cf-invariants-jito,
cf-invariants-jito-priorityfee, cf-invariants-jito-tippayment,
cf-invariants-pyth). See `docs/reachability.md` for the full uncovered
list + rationale.

## Overview

The atlas is a curated Anchor-program bug-class library and a
class-shaped emit layer that renders into Crucible v0.2.0 test suites
today and Trident v0.12.0 test suites on the roadmap. The unit of work
is the Anchor account-and-IDL model; the execution rails are Crucible
v0.2.0 over LiteSVM on the pinned Solana rails (anchor-lang 1.0.1,
solana-cli 2.1.21, platform-tools v1.52). The receipt behind the class
library is `n = 4` real-target ports on those same pinned rails: three
Jito programs (tip-distribution, priority-fee-distribution, tip-payment)
plus the Pyth Solana Receiver. Each port ships class-shaped invariants
with paired clean and planted twins so that every push standing-evidences
each class's proof of catch. The atlas natively owns the `collateral_mint_ref`
reference twins, the adoption scaffold, the coverage map, and this doc
set; it CITES the four real-target port repositories by URL via
`cases/*/CITATION.md` and consumes the `cf-invariants-anchor-*` crates
as pinned git dependencies. Every cited port URL stays canonical.

## What this is / what it is not

**What this is.** A curated bug-class library and a paired-fixture
discipline sitting on top of the free engines the Solana ecosystem
already ships. A class-shaped emit layer that renders the atlas's
class-family invariants into fixture sources for the existing engines,
so the same class shape reaches whichever engine a program team already
uses. A regression suite with standing proof of detection at each
cited commit.

**What this is not.** Not a fuzzer; the atlas renders into fuzzers, it
is not one. Not a replacement for Trident or Crucible; those engines
stay upstream, unmodified, and are the runtime substrate on which the
atlas's fixtures run. Not a bug-discovery engine or a security-finding
service; as of 2026-07-02 the atlas has zero upstream disclosures and
makes no discovery claim. Not an audit substitute; manual review, spec
verification, and posture review (upgrade authority, program-derived
address hygiene, sysvar handling) are not what this repository does.

## What is already free

The Solana ecosystem already ships a rich free stack for invariant
work. The atlas sits ON TOP of that stack; it does not replace any
part of it. Each of the following is a first-class dependency or
comparator, cited in one honest line so a reader can find the tool the
atlas does not aim to replace.

- **Trident v0.12.0** (Ackee Blockchain, MIT). Stateful and invariant
  fuzzing for Solana / Anchor programs. Shipped and maintained; ships
  its own `#[flow]`-annotated flows, snapshot API, assertion-style
  state checks, and an examples tree covering token, hello_world, and
  internal-test surfaces.
- **Crucible v0.2.0** (Asymmetric Research). Property-based fuzzing
  over LiteSVM with sBPF coverage guidance. Ships `#[invariant_test]`
  macros, a runtime `crucible run`, and example programs (`escrow`,
  `staking`) at single-target granularity.
- **Anchor native tests + `solana-program-test`** (Solana Foundation,
  Apache-2.0). The unit-and-integration test rails shipped with the
  Anchor framework; deterministic single-transaction and multi-instruction
  assertions on live account state.
- **mollusk v0.13.4** (Anza, Apache-2.0). Lightweight in-process runtime
  for `solana-program-test` cases with `Check::account(pubkey).lamports(...)`
  at single-instruction granularity.
- **LiteSVM v0.13.1** (Apache-2.0). Fast in-process SVM used as the
  substrate under Crucible and as a general test rail.

**What the atlas adds on top.** A single-source curated class library
of Solana bug shapes (balance conservation via fixture-side ledger +
`fuzz_assert_eq!`; monotonic accounting via `last_seen_*` snapshot +
`fuzz_assert_le!`; access control including the missing-Anchor-constraint
mint-authority sub-shape via a paired probe + sticky-flag assertion),
a paired same-source clean and planted twin discipline whose CI asserts
`clean = 0` markers and `planted >= 1` marker per cell on every push,
a class-shaped emit layer that renders those class shapes into fixture
sources for both Crucible (today) and Trident v0.12.0 (renderers landed
on the T-satlas-03 branch; runtime verification and the surface-language
flip are gated on a follow-up dispatch per the design notes), and a
bring-your-program adoption path that points the atlas at an adopter's
own Anchor program under the same pinned rails. As of 2026-07-02 we did
not locate a public Solana analog of the paired clean and planted
discipline; we do not claim exclusivity.

**Why this is not a PR into Crucible.** Crucible is the engine and the
primitive testing surface. The atlas is one layer above that: a curated
class library, a paired-fixture discipline, and a class-shaped emit
layer that composes with (not extends) the engine. A fuzzer team would
not merge a curated corpus and a class-registry code-generator into an
engine repository; the scopes are honestly separate. Crucible stays the
canonical engine home; the atlas stays the canonical class-library
home; the Trident v0.12.0 emit target does the same for the other engine.

## Coverage map (excerpt)

Compressed view of `docs/coverage_map.md`. The full table with
file-level citations, reader-proofing footer, applicability survey
(`n = 24` public Anchor programs sampled 2026-07-02), and appendix
lives at [docs/coverage_map.md](docs/coverage_map.md).

| Invariant class | Trident v0.12.0 | Crucible v0.2.0 | Anchor native + solana-program-test v1.1.2 | mollusk v0.13.4 | LiteSVM v0.13.1 | Audit-shop suites (awesome-trident-tests, `main` @ 2026-07-02) | Uncovered by comparators as of 2026-07-02 (temporarily-clean) |
|---|---|---|---|---|---|---|---|
| `balance_conservation` | harness-only | harness-only | harness-only, unit-level only | harness-only, unit-level only | (empty) | per-target, no class-shaped fixture | shipped by atlas |
| `monotonic_accounting` | harness-only | harness-only | harness-only, unit-level only | harness-only, unit-level only | (empty) | per-target, no class-shaped fixture | shipped by atlas |
| `access_control` | harness-only | harness-only | harness-only, unit-level only | harness-only, unit-level only | (empty) | per-target, no class-shaped fixture | shipped by atlas |
| `access_control.collateral_authority` (sub-shape) | harness-only | harness-only | (empty) | (empty) | (empty) | (empty) | shipped by atlas |
| `oracle_freshness` (planned, Phase 2) | (planned) | (planned) | (planned) | (planned) | (planned) | (planned) | not shipped by atlas |

**How to read this map.** Each row is an atlas invariant class or a
sub-shape of one. Each column is a program-level comparator; the
rightmost column records where no program-level comparator ships a
class-shaped fixture as of 2026-07-02. Every non-empty cell in the
full map carries a file-level citation of the form
`owner/repo/path @ <pin>` where `<pin>` is either a 7-character commit
sha or an immutable release tag. `harness-only` means the comparator
ships the engine but no class-shaped example that pairs a clean and a
planted variant of the same program with the invariant proven to fire
on planted and pass on clean. The rightmost column is
temporarily-clean as of 2026-07-02 and does NOT read as a durable
exclusivity claim; if a reviewer discovers a comparator ships a
class-shaped fixture matching one of these rows, the atlas row updates
additively per the maintenance rules in the full map.

**Scope note.** Solfuzz / solfuzz-agave (Firedancer) and sig-fuzz
(Syndica) are Solana VM-conformance harnesses that check whether two
VM implementations agree on execution semantics; they are out of scope
as program-level comparators and are omitted from the columns above
on purpose.

## Reference case and cited ports

### R1 reference: `references/collateral_mint_ref{,_planted}`

The synthetic Anchor collateralized-mint program shape. Three
instructions on a Bank config: `initialize_bank`, `deposit`, and
`mint_receipts`. The clean twin carries the mint-equality Anchor
constraint that binds `collateral_account` to the collateral mint the
bank was initialized with; the planted twin drops that one line and
nothing else. That missing constraint is a specification violation in
the planted twin: `deposit` reads `collateral_account` without
validating its mint field, so the fixture-side ledger of
`expected_receipts` (which only increments on deposits drawn from the
authorized collateral mint) diverges from the on-chain
`bank.total_receipts_minted` after any deposit routed through an
unauthorized mint. The Crucible fuzz walk and the deterministic
regression leg both stop at the first divergence and print
`INVARIANT VIOLATED collateral_authority`. Historical postmortems
document real Anchor-family Solana programs where the collateral or
printer-account mint was accepted without a mint-equality constraint
on the deposit path; the Cashio incident is the widely-cited example,
cited here once for motivation only. The actionable specification and
fixture stand on their own. Full case write-up:
[references/collateral_mint_ref/README.md](references/collateral_mint_ref/README.md);
diff shown in the case README.

### Four cited real-target ports (`n = 4`, exactly)

| Port | Class exercised (family) | Live URL | CITATION |
|---|---|---|---|
| cf-invariants-jito (Jito tip-distribution) | balance_conservation and three access_control sub-shapes | https://github.com/caliperforge/cf-invariants-jito | [cases/jito/CITATION.md](cases/jito/CITATION.md) |
| cf-invariants-jito-priorityfee (Jito priority-fee-distribution) | monotonic_accounting (state-commit sub-shape) | https://github.com/caliperforge/cf-invariants-jito-priorityfee | [cases/jito-priorityfee/CITATION.md](cases/jito-priorityfee/CITATION.md) |
| cf-invariants-jito-tippayment (Jito tip-payment) | monotonic_accounting (two state-commit sub-shapes) and access_control (bounds-check sub-shape) | https://github.com/caliperforge/cf-invariants-jito-tippayment | [cases/jito-tippayment/CITATION.md](cases/jito-tippayment/CITATION.md) |
| cf-invariants-pyth (Pyth Solana Receiver) | access_control (governance-transfer atomicity sub-shape) and balance_conservation (rent-return-authority sub-shape) | https://github.com/caliperforge/cf-invariants-pyth | [cases/pyth/CITATION.md](cases/pyth/CITATION.md) |

Each CITATION file mirrors the port's CI badge inline, pins the port's
class table to a 7-character commit sha, and states plainly that the
citation vendors no upstream code into the atlas and claims no
discovery of a bug in upstream Jito or Pyth. The `n = 4` figure is
exactly the receipt behind the class library; the aggregate cover
figures on a `n = 24` public-Anchor-program applicability survey live
in `docs/coverage_map.md` as a separate, bounded figure.

## Bring your program

The `adopt/` tree carries a copy-paste GitHub Actions workflow, an
adoption walkthrough README with numbered steps from `git clone` to a
green run on the adopter's own program (pin check on solana-cli
2.1.21, IDL parse, `cf-invariants-anchor suggest` with heuristic
default and AI provenance behind an explicit flag, `emit --target
crucible`, run both legs), and a worked example executed end-to-end
against `collateral_mint_ref` with full transcript. The scaffold is
designed for sub-hour adoption; the observed cold-run number is
recorded in this section once the timed cold run by a non-author has
returned PASS from the code_quality_reviewer. Honest scope: the
scaffold gives the harness and the recurring-class patterns;
program-specific business-logic properties still require the adopter
to state their own semantics. The suggester ranks candidates from the
IDL, it does not know the program's spec.

Full walkthrough: [adopt/README.md](adopt/README.md). Workflow
template: [adopt/workflow-template.yml](adopt/workflow-template.yml).
Worked example transcript:
[adopt/worked-example-log.md](adopt/worked-example-log.md).

## License

The atlas is Apache-2.0. See [LICENSE](LICENSE). Third-party
attributions live in [NOTICE](NOTICE): Crucible, Trident v0.12.0,
Anchor, `solana-program-test`, LiteSVM, and mollusk each cited under
the license each ships. No upstream program code is vendored into the
atlas; the `collateral_mint_ref` synthetic program is entirely our
own code. AI involvement in the authoring stack is disclosed in
[AI_DISCLOSURE.md](AI_DISCLOSURE.md), including the suggester
provenance model and its `MockTransport` default.

## Governance and additive discipline

The five existing `experiments/cf-invariants-*` port repositories stay
byte-untouched by this atlas; the five live-cited port URLs remain
canonical at their own commit histories. New class families land in
this atlas or in `cf-invariants-anchor`, never as forks of the port
repositories. The rightmost coverage-map column is temporarily-clean
and dated 2026-07-02; a future re-check either re-dates the column or
clears the entry for the row whose primary-source status changed, per
`docs/coverage_map.md`. Postmortem sourcing follows the one-sentence
non-actionable convention: the actionable spec must stand on its own
without the incident, and no incident mechanism, monetization path, or
specific-target reproduction ships in this repository. Full design
rationale, class-add procedure, Trident v0.12.0 pin decision, and the
solana-cli 2.1.21 PATH-prefix note live in
[docs/design_notes.md](docs/design_notes.md). The wording standard the
atlas holds itself to is
`agents/engineering_lead/templates/planted_twin_framing_discipline.md`
in the CaliperForge org repository; the design notes carry a reader
summary and a pointer.
