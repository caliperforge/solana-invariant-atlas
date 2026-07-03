# PR template: offer to wire the atlas pre-deploy screen into an Anchor team's repo

Use this when CaliperForge (or the CEO on our behalf) offers to open
the integration PR on an Anchor program team's or audit firm's repo.
Keep it short, honest, one link. The give is "we wire it in, you
review." No uniqueness claim, no would-have-caught claim, no ask
beyond a review, a merge, or a redirect.

The four fill-ins are at the top; the body below is the copy-paste PR
description.

---

**Fill in before opening the PR:**
- `<TEAM_NAME>`: the program team or audit firm's name.
- `<PROGRAM_CRATE>`: their Anchor program crate name (the
  `programs/<crate>/` directory in their repo).
- `<IDL_PATH>`: the path to their Anchor IDL JSON in their repo,
  e.g. `target/idl/<crate>.json`.
- `<INVARIANT_NAME>`: the `fn invariant_*` name in the fixture we
  wrote for them, e.g. `invariant_pending_receipts_conservation`.

---

## PR title

`pre-deploy-screen: add solana-invariant-atlas CI (defender-side, no runtime change)`

## PR description (copy-paste body)

Hi `<TEAM_NAME>` team,

We are CaliperForge. We maintain
[solana-invariant-atlas](https://github.com/caliperforge/solana-invariant-atlas),
a defender-side pre-deploy screen for Anchor programs: a curated
bug-class library (balance conservation, monotonic accounting, access
control including the mint-authority sub-shape) shipped as paired
clean/planted fixtures on the free engines the ecosystem already runs.
The runnable leg today is Crucible v0.2.0 under LiteSVM on pinned
Solana rails (solana-cli 2.1.21, platform-tools v1.52); the Trident
leg lands with our upstream compatibility work (T-satlas-03).

This PR adds two files, for review:

- `.github/workflows/pre-deploy-screen.yml`: the workflow. On every
  push and PR it checks out this repo and the atlas (pinned by commit
  SHA), builds `<PROGRAM_CRATE>` with `cargo build-sbf` on the pinned
  rails, and runs the invariant fixture below on a fixed 30-second
  Crucible budget. It passes with `clean-passes: OK` when the encoded
  properties hold, and fails with the `INVARIANT VIOLATED` marker and
  the offending sequence when they do not.
- `fuzz/<PROGRAM_CRATE>/`: a small fixture crate we wrote for you.
  Its `src/main.rs` declares `<INVARIANT_NAME>`, a fixture-side ledger
  walked in lock-step with your program's accounts, generated from
  your IDL at `<IDL_PATH>` by our pinned CLI (heuristic suggester; the
  AI suggester stays behind a flag and is provenance-tagged when used,
  see `AI_DISCLOSURE.md` in the atlas repo). The expectation
  expressions encode what we could read from your IDL; where your
  spec differs, that is exactly the part we would like your review on.

**What it is not.** Not an audit, not a runtime guard, not a discovery
engine for unknown bug classes, and not a claim of uniqueness. Trident
already ships stateful and invariant fuzzing for Anchor programs;
Crucible is the engine this leg runs on; Anchor native tests, mollusk,
and LiteSVM cover their own layers. The atlas repo's README ("what is
already free" section) says all of this plainly. A green run means the
encoded properties hold at the pinned revs; it does not clear the
program of a class no property encodes.

**Runtime footprint.** Zero. Nothing in this PR changes your program,
your deployment artifacts, or any dependency on your production path.
The workflow is CI-only, and the atlas is vendored at CI time via
`actions/checkout` at a pinned ref; no upstream code ships into your
build artifacts.

**What you would review.**
- One YAML file: the workflow (four env-var knobs at the top, nothing
  else editable).
- One small fixture crate: a `Cargo.toml` (mirrors the atlas reference
  layout) and one generated-then-reviewed `src/main.rs` with the
  ledger and the invariant. The expectation expressions are the
  substance of the review.

**How to run it locally before merging:**
```sh
git clone --depth 1 --branch v0.2.0 https://github.com/asymmetric-research/crucible.git ../crucible
export PATH=~/.local/share/solana/install/releases/2.1.21/solana-release/bin:$PATH
cargo build-sbf --tools-version v1.52 --manifest-path programs/<PROGRAM_CRATE>/Cargo.toml
cd fuzz/<PROGRAM_CRATE>
crucible run <PROGRAM_CRATE> <INVARIANT_NAME> --release --timeout 30
```
Expected: rc=0 and zero `INVARIANT VIOLATED` markers. If it trips on
a path you did not expect the walk to exercise, that is worth a
message either way: either the fixture's expectation needs your
semantics, or the finding is real.

**If you would rather write it yourselves,** the walkthrough is at
[`adopt/README.md`](https://github.com/caliperforge/solana-invariant-atlas/blob/main/adopt/README.md)
in the atlas repo (designed for sub-hour adoption). This PR is the "we
wire it in, you review" version of that same path.

**If this is not for you,** no worries; feel free to close. If it is
almost right but you want the fixture to declare a business-logic
property only your team can state (a fee ledger, a share-price
observable, an authority check), tell us the property and we will
extend the fixture in a follow-up commit on this PR.

The atlas's own reference case
(`references/collateral_mint_ref{,_planted}/`) ships as a clean/planted
twin pair, so there is a standing receipt that the property catches
the class it encodes: the clean leg passes with zero markers and the
planted twin (one dropped Anchor constraint in a synthetic copy of our
own program) trips `INVARIANT VIOLATED collateral_authority` on both
the fuzz and deterministic legs. The seeded violations are
specification-violation logic errors in our own synthetic copies, per
the framing standard at
`agents/engineering_lead/templates/planted_twin_framing_discipline.md`.

Thanks for reading. We are happy to iterate on the smallest possible
diff.

--- CaliperForge

---

## Notes for the sender (do not paste)

- **Do not** claim the screen would have caught any specific past
  incident. The reference case is a regression fixture for the encoded
  class, not a reproduction of any incident; its README cites the
  motivating postmortem once, as motivation only.
- **Do not** claim uniqueness. Trident ships stateful and invariant
  fuzzing for Anchor programs today; the atlas composes with it and
  with Crucible. On the twin discipline, the only permitted form is
  temporarily-clean: as of 2026-07-02 we did not locate a public
  Solana analog of the paired clean/planted discipline; we do not
  claim exclusivity.
- **Do use** the "defender-side pre-deploy screen" framing on the
  surface. Not offense-side framing, not discovery-engine framing.
- **The ask floor is "for review."** Never state or imply the team
  would take anything beyond reviewing the PR.
- **Keep the ask small.** One workflow file plus one small fixture
  stub at most. If the team wants properties extended, that is a
  follow-up commit on the same PR, not a re-scope.
- **Trident wording stays gated** until T-satlas-03 lands: "Crucible
  leg today; the Trident leg lands with T-satlas-03." No
  forward-looking compatibility claim.
- **Framing discipline reference:**
  `agents/engineering_lead/templates/planted_twin_framing_discipline.md`.
  Grep the PR body against the word-ban list named there (plus
  em-dashes) before hitting Submit.
