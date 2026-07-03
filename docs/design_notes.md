# Design notes

Filled at T-satlas-06 on top of the T-satlas-01 skeleton. Decisions
recorded here are binding. Any relitigation is a spec change routed
through engineering_lead, not through a build ticket.

## Thin-index topology (why this repo links out instead of absorbing)

This repository is a thin index (spike Q3, Option B). It natively owns
the reference twins under `references/`, the adoption scaffold under
`adopt/`, the coverage map under `docs/`, the class-shaped emit layer
consumed via the pinned `cf-invariants-anchor-*` git dependencies, and
the docs; it CITES the four real-target port repositories by URL via
`cases/*/CITATION.md` and consumes the `cf-invariants-anchor-*` crates
as pinned git dependencies. It does not absorb any port repo by
subtree, submodule, or copy. Rationale:

- Live-cited port URLs stay canonical. A reviewer who clicks a cited
  port link lands on that repository with its own commit history, its
  own CI badge, and its own contribution graph. No competing
  snapshot, no "which is canonical" question.
- Additive-only rule satisfied literally: nothing upstream is renamed,
  restructured, or superseded. The five existing `experiments/cf-invariants-*`
  trees stay byte-untouched by this repository.
- Atlas CI stays lean: only code native to this repo is re-tested per
  push; the cited ports keep their own CI on their own repositories.
- The Trident emit target strengthens the live-cited flagship
  (`cf-invariants-anchor`) too when the T-satlas-03 additive upstream
  branch merges to `main`; the atlas is the demo surface, not a
  second canonical home for the emit crate.

## Library consumption: pinned git dependencies (decision + fallback + lane taken)

Decided in the build spec section 2, rule 3: `library/` consumes the
`cf-invariants-anchor-*` crates as git dependencies pinned at a single
revision, not as a copy. One canonical library home (the live-cited
flagship repository), zero canonical-source ambiguity, and upstream
improvements strengthen the live-cited repository too.

- **Current pin:** `rev = "1b3905e9f756d87dba488e1e47b4d704bfbaa3f5"`
  (`cf-invariants-anchor` `main` at scaffold time, workspace version
  0.1.1).
- **Re-pin schedule:** the atlas git-dep re-pins to the merge tag cut
  when the T-satlas-03 additive upstream branch (`feature/trident-emit-v012`)
  lands on `main`.
- **Fallback (spec section 2, rule 3):** if the upstream branch cannot
  merge green inside the send window (window opens 2026-07-13, tail
  2026-07-17 per the win-conditions read), the Trident renderers ship
  as an atlas-native extension crate under `library/` and the upstream
  merge becomes a follow-up. R3 is satisfied either way; the
  extension-crate fallback keeps the flip independent of the upstream
  merge.

### Fallback lane taken (T-satlas-03 status at time of write)

T-satlas-03 landed the three `render_trident_*` render functions, the
`Trident.toml` template, the `trident-tests/` scaffold generator, the
class-registry `access_control.collateral_authority` sub-shape
extension, and the atlas twin's `trident-tests/` cells as
working-tree changes on the `feature/trident-emit-v012` branch in
`experiments/cf-invariants-anchor` (uncommitted; per dispatch
convention, Director commits post-review). Item-3 runnable
verification on the four twin-x-target cells was DEFERRED to a
follow-up dispatch because Trident v0.12.0 is not installed on the
host workstation. Consequences for this NOTICE and README pair:

- The `feature/trident-emit-v012` upstream branch is not yet merged
  to `main`; the atlas git-dep remains pinned at
  `1b3905e9f756d87dba488e1e47b4d704bfbaa3f5`.
- Trident-surface language in `README.md` reads "Crucible-first,
  Trident on roadmap" (spec section 12.7 wording constraint) until
  the follow-up dispatch produces pasted runnable verification and
  the upstream branch merges. When both land, this section is edited
  to record the merge tag, the atlas git-dep is re-pinned to that
  tag, and the README's Trident sentence is flipped per spec section
  5.
- If the follow-up dispatch cannot install Trident v0.12.0 inside the
  window, the atlas-native extension-crate fallback (spec section 2,
  rule 3) is the lane taken; the renderers ship in a new
  `library/emit-trident/` crate consumed by `library/` and the
  upstream merge becomes a follow-up. This section is updated to
  record the lane.

### Deviation note: the cli crate is consumed as a pinned binary

`cf-invariants-anchor-cli` is a bin-only package upstream (no lib
target), so Cargo cannot take it as a library dependency. The four
lib crates (`core`, `emit`, `suggest`, `report`) are git dependencies
of the `library/` driver; the CLI itself is consumed at the SAME
pinned rev via:

```sh
cargo install --git https://github.com/caliperforge/cf-invariants-anchor \
  --rev 1b3905e9f756d87dba488e1e47b4d704bfbaa3f5 cf-invariants-anchor-cli
```

If the T-satlas-03 additive upstream branch wants to add a lib target
to the cli crate, that is additive and welcome; nothing here depends
on it landing.

## Class-add procedure

How a new class family lands. The ordering below is load-bearing: a
class registered without a paired reference is not adoptable, and a
coverage-map row added before the registry entry lands is a red herring
for reviewers.

1. **Registry extension in `cf-invariants-anchor-core`.** Add the
   class string (parent, or `parent.subshape` for a sub-shape) to the
   class registry. Additive-only; existing class strings keep their
   existing string identity so downstream candidates and emitted
   fixtures stay byte-stable on the old registry.
2. **Renderer per emit target in `cf-invariants-anchor-emit`.** Add
   `render_crucible_<class>` and `render_trident_<class>`, or thread
   the sub-shape through the parent renderer with an explicit toggle
   (as T-satlas-03 did for the `collateral_authority` sub-shape).
   Both target dispatches route the class string to the right
   renderer.
3. **Reference twin authored under `references/<class_ref>{,_planted}/`.**
   Byte-identical clean and planted twins except for a single-hunk
   diff on the constraint or accounting line that the class captures.
   The fixture-side ledger snapshots after each action and prints
   `INVARIANT VIOLATED <class>` on divergence.
4. **Coverage-map row addition to `docs/coverage_map.md`.** Row added
   with file-level citations for every non-empty cell; the rightmost
   temporarily-clean column carries a date and no exclusivity claim.
5. **CI cells added to `.github/workflows/ci.yml` matrix.** The
   `discover` job in the T-satlas-01 skeleton picks up new
   `references/<class_ref>{,_planted}/fuzz/` and `.../trident-tests/`
   subtrees automatically; the matrix expands without a workflow
   edit as long as the twin layout is followed.
6. **README updates.** Coverage-map excerpt row added; if the new
   class is the R1 of a new spec phase, the R1 case block gains an
   entry.
7. **Independent review at the code gate (spec section 10 G2)** by
   `code_quality_reviewer`, plus a claims-review pass at G3 if the
   class is publicly-visible on the coverage map.

Additive-only. Class removals or renames are a spec-level change; a
class dropping out of the coverage map records a change-log entry,
never a silent edit (per the map's maintenance rules).

## CI per-cell budget (decided in T-satlas-01, per spec section 3)

| Leg | Iteration budget | Wall-clock cap |
|---|---|---|
| Crucible fuzz cell | `crucible run --timeout 30` (30 s fuzz budget; the minimal counterexample on every shipped class fires in 1-2 actions) | 5 min/cell (`timeout-minutes: 5` on the matrix step) |
| Trident fuzz cell | 1000 flow iterations recorded in the emitted `Trident.toml` (initial budget; the follow-up runnable-verification dispatch verifies it trips the planted marker well inside the cap and tunes downward if possible, never above the cap) | 5 min/cell |
| Deterministic regression cell | fixed action sequence, no fuzz budget | 5 min/cell |

The matrix for Phase 1 is `collateral_mint_ref` x {clean, planted} x
{crucible, trident} = 4 cells plus the `library` build job. Upstream
reference matrices stay in the upstream repo's CI (thin-index rule).

## The solana-cli 2.1.21 PATH-prefix trap

See `docs/toolchain.md` for the full pin table. The load-bearing
paragraph is: developer machines commonly default to a newer
solana-cli (the spike host defaults to 4.0.1). Local reproduction of
CI results requires the pinned release first on PATH before any
`cargo build-sbf` invocation:

```sh
export PATH=~/.local/share/solana/install/releases/2.1.21/solana-release/bin:$PATH
```

`scripts/check-solana-pin.sh` verifies the active version and prints
this line if the pin is not active. Reviewers reproducing the atlas's
CI cold cite this section (the walkthrough README also cites it).

Related trap: Agave v2.1.21 ships a `cargo-build-sbf` that panics
with a `NotFound` error if `~/.cache/solana` does not exist (fresh
runners). CI pre-creates the directory before the first invocation;
do the same on a fresh machine (`mkdir -p ~/.cache/solana`).

## Trident v0.12.0 vs. v0.13.0-rc pin decision

Trident v0.12.0 stable (released 2025-11-27) is the pinned emit
target. The active development line at spec time was v0.13.0-rc.4
(2026-05-18). The decision to pin to v0.12 stable and not chase
v0.13-rc is deliberate, made in the spike Q2 and re-affirmed in the
build spec section 3:

- v0.12 is the version with a stable assertion-style state-check API
  and a stable `#[flow]` annotation surface; a reviewer reproducing
  atlas Trident cells does so on the same API surface the atlas emits
  against, without a version-drift diagnosis.
- v0.13-rc is a release candidate, not stable. Pinning against a
  moving rc bakes an ongoing maintenance tax into the atlas: any
  incompatible-change beat between rc points ripples through the
  emit crate's Trident renderers.
- v0.12's audit-shop deployment footprint (Ackee's own suites on
  named production targets) is a receipt that v0.12's surface is
  the one program teams actually build against today; the atlas
  emits into what teams use, not what teams may use later.

If v0.13 ships stable and a program team asks for a v0.13-target
render, the class-add procedure above is extended with a v0.13
renderer alongside the v0.12 renderer, additively. v0.12 stays
supported; nothing about the v0.12 emit path is retired by a v0.13
addition.

## Wording discipline pointer

The wording standard for every case ticket, case README, case source
file, case scorecard, coverage-map row prose, and reader-facing
document in this repository is
`agents/engineering_lead/templates/planted_twin_framing_discipline.md`
in the CaliperForge org repository. That template is the canonical
source; this section is a reader-summary pointer, not a copy.

Reader summary of the discipline the atlas holds itself to:

- The seeded logic error is always a specification violation in the
  atlas's own synthetic program. Not an adversary action, not
  reproduction of a real production incident's mechanism. Format:
  "the planted twin's `<code path>` reads or computes or calls
  `<wrong source or direction or step>`, violating the `<named
  property>`."
- The detection leg is always a regression sequence that trips the
  invariant marker. Never an adversary-side leg. Assertions are on
  the `INVARIANT VIOLATED <marker>` string and on the return-code
  discipline (`rc = 0` on clean, `rc != 0` on planted). Never on
  extraction or gain figures.
- No adversary persona, no receiver-address ledger, no
  adversary-succeeds verb, no gain computation. Handlers are generic
  to the class, not tuned to any specific real target.
- If the class is sourced from a public incident, the incident is
  cited ONLY in a one-sentence non-actionable motivation note; the
  actionable spec stands on its own without the incident.
- Word bans apply to every atlas-authored surface and are
  grep-checked at the code gate. The authoritative ban list lives in
  the framing-discipline template referenced by path above; this
  reader summary does not reprint the ban words so grep receipts on
  this file stay clean. Em-dashes are banned on all public surfaces
  (house rule); use the ASCII hyphen-and-space or two hyphens where
  a phrase requires a dash.
- Verb register stays defensive throughout: "the invariant catches
  the class"; "the regression trips the marker"; "the seeded
  specification violation".

The wording discipline exists because case tickets whose text can
plausibly be read as a recipe against a real named target are refused
by the authoring model at build time, even when the technical work is
a defensive regression fixture. Refused builds are wasted Fable and
Opus hours; this block keeps refusal the exception, not the norm.

## Local-tree note for maintainers

In the CaliperForge working tree the three cited Jito repositories
house their public repository root under a `port/` subdirectory (the
outer directory also carries an upstream submodule and a Crucible
build tree). Badge and link targets in `cases/*/CITATION.md`
therefore point at the PUBLIC repository roots on GitHub, where each
Cargo workspace sits at the top level. Do not link `port/` paths
publicly.

## Crucible license state at v0.2.0

The GitHub repository-level license field for
`asymmetric-research/crucible` reads NOASSERTION because the
auto-detector does not identify the LICENSE template. compliance_lead
verified on 2026-07-03 that the `LICENSE` file at tag v0.2.0
(tag commit sha 35ec899ae5a4148788f7face863b0ca24d92781d) is the
standard MIT template with copyright "Copyright (c) 2026 Asymmetric
Research". NOTICE finalizes Crucible's attribution to MIT on that
basis (Resolution A per T-satlas-06 gate G1). No downstream compliance
action open on this item.
