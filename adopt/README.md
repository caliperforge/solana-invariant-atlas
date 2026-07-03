# Bring your program: wire the atlas into your CI as a pre-deploy screen

You have an Anchor program. You want a defender-side pre-deploy screen
that runs on every PR: your program built on pinned Solana rails, a
Crucible invariant fixture walking it under LiteSVM, and a standing
green/red receipt on each commit. Not an audit, not a runtime guard,
and not a discovery engine for unknown bug classes. A pre-deploy
screen: the properties your fixture encodes run against your program
on every commit, so a class of regression that the fixture encodes
cannot silently land on `main`.

This walkthrough is designed for sub-hour adoption. Anchor build
cycles are longer than an EVM toolchain's, so budget the hour rather
than ten minutes; most of it is the first `cargo build-sbf` and the
first Crucible build.

## 1. What this is, and what it is not

The atlas is a curated bug-class library plus a paired clean/planted
fixture discipline on top of the free engines the Solana ecosystem
already ships. It composes with those rails; it replaces none of them:

- **Crucible** (v0.2.0) is the fuzz engine this walkthrough's leg runs
  on.
- **Trident** (v0.12.0) ships its own stateful and invariant fuzzing
  for Anchor programs. The atlas emits a Trident-compatible fixture
  shape as a second target: Crucible leg today; the Trident leg lands
  with T-satlas-03.
- **Anchor native tests**, **mollusk**, and **LiteSVM** cover their
  own layers and stay in your stack.

The atlas repo's top-level `README.md` has a "what is already free"
section with one honest line on each of these and what this repository
adds on top. The receipt behind the class library is n = 4 real-target
ports on these same rails (the three jito repos plus pyth), cited at
file level in `cases/*/CITATION.md`. This adoption path claims no
finding disclosures anywhere; the planted violations it detects are
seeded in synthetic copies of our own reference programs, per the
framing standard at
`agents/engineering_lead/templates/planted_twin_framing_discipline.md`
in the CaliperForge org repo.

Honest scope, up front: the scaffold gives you the harness and the
recurring-class patterns (balance conservation, monotonic accounting,
access control including the mint-authority sub-shape).
Program-specific business-logic properties still require you to state
your own semantics; the suggester ranks candidates from your IDL, it
does not know your spec.

## 2. Step 1: clone and pin check

```sh
git clone https://github.com/caliperforge/solana-invariant-atlas
cd solana-invariant-atlas
bash scripts/check-solana-pin.sh
```

The atlas rails are solana-cli 2.1.21 with platform-tools v1.52,
pinned explicitly (never latest). Developer machines commonly default
to a newer solana-cli, and the pin check fails loudly when that is the
case. This is the PATH-prefix trap, stated plainly: put the pinned
release first on PATH before any `cargo build-sbf`:

```sh
export PATH=~/.local/share/solana/install/releases/2.1.21/solana-release/bin:$PATH
bash scripts/check-solana-pin.sh   # expect: check-solana-pin: OK (solana-cli 2.1.21 ...)
```

If the pinned release is not installed yet:

```sh
sh -c "$(curl -sSfL https://release.anza.xyz/v2.1.21/install)"
mkdir -p ~/.cache/solana   # Agave 2.1.21 cargo-build-sbf NotFound trap on fresh machines
```

The full pin table is `docs/toolchain.md`.

## 3. Step 2: point the atlas at YOUR Anchor program

Copy the workflow template into your program repo and set the four
env-var knobs at the top (nothing else in the file needs your
attention):

```sh
mkdir -p .github/workflows
curl -sSL \
  https://raw.githubusercontent.com/caliperforge/solana-invariant-atlas/main/adopt/workflow-template.yml \
  -o .github/workflows/pre-deploy-screen.yml
```

```yaml
env:
  ADOPTER_PROGRAM_CRATE: my_anchor_program            # your program crate name
  ADOPTER_IDL_PATH: target/idl/my_anchor_program.json # your Anchor IDL path
  SOLANA_ATLAS_REPO: caliperforge/solana-invariant-atlas  # leave as-is
  SOLANA_ATLAS_REF: <commit SHA>                      # pin to a specific rev
```

`SOLANA_ATLAS_REF` is pinned so a change on the atlas cannot silently
change what runs in your CI. Bump it deliberately.

Install the pinned CLI once (the same rev the atlas `library/` driver
pins):

```sh
cargo install --git https://github.com/caliperforge/cf-invariants-anchor \
  --rev 1b3905e9f756d87dba488e1e47b4d704bfbaa3f5 cf-invariants-anchor-cli
cf-invariants-anchor version   # cf-invariants-anchor 0.1.0 (target: Crucible v0.2.0)
```

## 4. Step 3: `suggest` (heuristic default; AI behind a flag, disclosed)

Point the suggester at your IDL (`anchor build` writes it to
`target/idl/<crate>.json`):

```sh
cf-invariants-anchor suggest target/idl/my_anchor_program.json
```

You get ranked class candidates as JSON: `access_control` (including
the mint-authority sub-shape), `balance_conservation`,
`monotonic_accounting`, each with a rank, a rationale, and
`"source": {"kind": "Heuristic"}`. On the atlas's own reference
program the top of the list looks like:

```json
{
  "name": "invariant_mint_receipts_rejects_unauthorized",
  "class": "access_control",
  "rank": 0.78,
  "source": { "kind": "Heuristic", "suggester_version": "0.2.0" }
}
```

Review the list before emitting; the heuristic reads your IDL, not
your spec. On the worked example below it also proposes a conservation
candidate over a PDA bump byte (`bank_authority_bump: u8`), which is
not a balance and should be discarded by a human. That is the honest
shape of the tool: it ranks candidates, you pick the ones that encode
your semantics.

Provenance is `Heuristic` unless you pass `--ai`. With `--ai` the
candidates are tagged
`InvariantSource::AiSuggested { model, prompt_version, timestamp_utc }`,
an audit-log entry is written, and the emitted fixture's disclosure
header names the source (the `AI_DISCLOSURE.md` pattern in the atlas
repo). No performance claim is attached to the AI layer; its
provenance is disclosed, not scored.

## 5. Step 4: `emit --target crucible` and create the fuzz crate

One-time scaffold: your fuzz crate lives at
`fuzz/<ADOPTER_PROGRAM_CRATE>/` in your repo, in the same layout as
the atlas's own reference at
`references/collateral_mint_ref/fuzz/collateral_mint_ref/`. Copy that
reference `Cargo.toml` and adjust three things, nothing else: the
package name; the program path dependency
(`../../programs/<your-crate>` with the `no-entrypoint` feature); and
the two Crucible path deps, which point at a Crucible v0.2.0 checkout
sibling to the repo root. The atlas reference sits two directories
deeper than your `fuzz/<crate>/` will, so shorten
`../../../../../crucible/...` to `../../../crucible/...`. The
workflow template lays CI out the same way (your repo and `crucible/`
side by side):

```sh
git clone --depth 1 --branch v0.2.0 https://github.com/asymmetric-research/crucible.git ../crucible
mkdir -p fuzz/my_anchor_program/src
cp <atlas>/references/collateral_mint_ref/fuzz/collateral_mint_ref/Cargo.toml fuzz/my_anchor_program/
# edit: package name + program path dep, as above
```

Then emit the fixture for the candidate you picked (index from the
`suggest` output):

```sh
cf-invariants-anchor emit target/idl/my_anchor_program.json \
  --target crucible --candidate-index 2 \
  --out fuzz/my_anchor_program/src/main.rs
```

(`--target trident` renders the Trident fixture shape once T-satlas-03
lands; Crucible is the runnable leg today.)

The emitted fixture opens with a disclosure header so you can
recognize the shape; from the worked example:

```rust
// invariant_pending_receipts_conservation
//
// Emitted by cf-invariants-anchor v0.2.0 for the balance_conservation class.
// Target: Crucible v0.2.0 (asymmetric-research/crucible).
// Source: Heuristic (suggester v0.2.0). No AI suggestion in this candidate.
```

The fixture carries a fixture-side ledger walked in lock-step with
your program's accounts, and one `fn invariant_*` per property. The
generated actions are generic to the class; the expectation
expressions are the part you review and, where your semantics differ,
edit. That edit is the "state your own spec" moment from section 1.

## 6. Step 5: run both legs

**Clean leg.** Build your program on the pinned rails, then run the
fixture (matches the atlas's own CI clean job):

```sh
cargo build-sbf --tools-version v1.52 \
  --manifest-path programs/my_anchor_program/Cargo.toml
cd fuzz/my_anchor_program
crucible run my_anchor_program invariant_pending_receipts_conservation \
  --release --timeout 30
```

Expected: rc=0 and zero `INVARIANT VIOLATED` markers in the output.
That is the standing receipt on that commit.

**Optional planted twin leg.** Copy your program, seed a single-hunk
specification violation in the copy (drop one Anchor constraint, flip
one accounting step), place it under `planted/<twin-name>/` in the
layout the workflow template documents, and re-run. The leg is green
exactly when the seeded violation is caught with a marker. This is the
receipt discipline: "our clean leg passes" alone is falsifiable only
against real defects; "our clean leg passes AND our planted twin fails
with our marker" is the pair that makes the receipt mean something,
a standing detection receipt that the invariant catches the class it
encodes. As of 2026-07-02 we did not locate a public Solana analog of
the paired clean/planted discipline; we do not claim exclusivity. The
framing standard for seeding the violation (a logic error in your own
synthetic copy, nothing else) is
`agents/engineering_lead/templates/planted_twin_framing_discipline.md`.

## 7. Step 6: add the badge to your README

One line, pointing at your own workflow:

```markdown
![pre-deploy-screen](https://github.com/<you>/<your-program>/actions/workflows/pre-deploy-screen.yml/badge.svg)
```

If you also want to show the atlas rev you pin against, copy the
atlas's own badge line from its top-level `README.md`.
TODO: badge URL after §10 flip (the atlas repo URL is finalized at the
public flip; T-satlas-06 resolves this marker).

## 8. Step 7: local dev loop

The CI checks the atlas out fresh at `SOLANA_ATLAS_REF` on every run.
Locally you have two options:

**Option A (simpler): clone the atlas alongside your repo** and use it
as the reference for the fuzz-crate scaffold and the pin check, as in
steps 1 and 4. Day to day you only re-run:

```sh
(cd fuzz/my_anchor_program && crucible run my_anchor_program <invariant> --release --timeout 30)
```

**Option B (in-repo): vendor the atlas as a git submodule** at
`lib/solana-invariant-atlas/` in your repo, so the reference layouts,
`scripts/check-solana-pin.sh`, and `docs/toolchain.md` travel with
your checkout. The CI still checks the atlas out fresh at the pinned
ref, so the submodule is a workstation convenience, not a CI
dependency.

## 9. Using this as a pre-deploy screen (for audit firms)

If you are an audit firm (Zellic, Neodyme, OtterSec, or similar)
offering Anchor program engagements, adding this workflow to a
client's repo at engagement start gives you three things:

1. **A standing receipt of the encoded classes on every commit**
   between engagement start and delivery. If a commit lands that
   breaks a property the fixture encodes, the badge goes red on the
   commit that introduced it, not at delivery.
2. **A shared, auditable substrate for property discussion.** When you
   tell the client "add a ledger expectation for the receipt mint" and
   they push it, the CI runs it under the same fuzz walk you would use
   to reproduce a finding. The invariant is the discussion object, and
   its marker string is the client's own regression handle after
   delivery.
3. **A pre-deploy gate that outlives the engagement.** After delivery,
   the client's own team keeps the workflow green through maintenance.
   If a future PR trips the marker, the PR author sees the receipt
   before it ships to mainnet.

The scope discipline is the honest sale: this is a pre-deploy screen
for the encoded classes plus whatever business-logic properties the
engagement declares. It is not a replacement for manual review, for
reasoning about upgrade authority and deployment posture, or for the
classes that no encoded property covers. The atlas repo's `README.md`
"what is already free" section names the complementary tools.

## 10. What you will see when it passes, and when it catches something

Both excerpts below are pasted from the worked example (the atlas's
own `collateral_mint_ref` reference; full unabridged transcript in
`adopt/worked-example-log.md`).

**Passing clean leg (excerpt).**

```
$ crucible run collateral_mint_ref invariant_collateral_authority --release --timeout 30
[FUZZ_PULSE] run time: 15s, clients: 1, corpus: 22, crashes: 0, executions: 8196, ...

[FUZZ] Timeout reached (30s). Exiting gracefully.
rc=0

$ cargo run --release --bin regression
regression: clean pass (total_receipts_minted=0, expected_receipts=0)
rc=0
```

Zero crashes, zero markers, rc=0 on both the fuzz walk and the
deterministic regression leg. That is the standing receipt on that
commit.

**Catching the planted specification violation (excerpt).**

```
$ cargo run --release --bin regression      # planted twin
INVARIANT VIOLATED collateral_authority: total_receipts_minted=12345 expected_receipts=0
rc=1

$ crucible run collateral_mint_ref invariant_collateral_authority --release --timeout 30
[FUZZ_FINDING] crash:crash_7f49f60aa9bf086c summary:INVARIANT VIOLATED collateral_authority: total_receipts_minted=7746 expected_receipts=0
```

The marker names the property that diverged (`collateral_authority`),
and the numbers are the on-chain counter against the fixture-side
ledger. The planted twin here differs from the clean reference by one
dropped Anchor constraint and nothing else; the regression leg stops
at the first violation. It detects, it does not maximize.

## 11. Repo hygiene and license

The atlas is Apache-2.0. `NOTICE` in the atlas repo attributes
Crucible, Trident, Anchor, and LiteSVM per their own licenses; no
upstream program code is vendored into the atlas. The workflow above
vendors the atlas at CI time via `actions/checkout` at a pinned ref,
so nothing about the workflow ships upstream code into your repo or
your build artifacts.

## 12. AI disclosure

CaliperForge's authoring stack is AI-augmented. What is AI-touched in
this adoption path, and the reviews that gate it, is disclosed in
`AI_DISCLOSURE.md` in the atlas repo, including the suggester
provenance model (`InvariantSource::AiSuggested`) and its
mock-transport default. CI verdicts and license posture are not
AI-touched.
