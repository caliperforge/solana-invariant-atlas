# AI Disclosure: solana-invariant-atlas

solana-invariant-atlas is built and maintained by CaliperForge under an
AI-augmented authoring stack. This document is calm disclosure of which
surfaces are AI-touched and the review discipline that gates each one.
The discipline mirrors the sibling libraries `cf-invariants-anchor`,
`uniswap-v4-invariants`, `invariant-atlas`, and `bsc-invariants`.

## What is AI-touched

- **Reference programs and invariant fixtures.** The clean and planted
  twin Anchor programs under `references/`, their invariant fixtures,
  and the regression sequences are drafted by a Claude model and
  reviewed and edited by the case specialist (`rust_anchor_specialist`)
  before landing. Every case is additionally gated by an independent
  code-quality review before any public flip.
- **READMEs, case write-ups, coverage map, scorecards, and this file.**
  Drafted with AI assistance; reviewed against CaliperForge's internal
  register rubric plus an independent claims review before publish.
- **Class-shape drafting and emit renderer prose.** The class-shape
  library and the atlas's per-emit-target renderer prose (docstrings,
  disclosure headers, error strings) are drafted with AI assistance
  and reviewed at the code gate. The renderer output itself is
  deterministic once the class shape is locked; the fixture bytes for
  a given (class, program, target) triple do not vary run-to-run.
- **The invariant suggester (consumed, not authored here).** The
  `library/` crate consumes the `cf-invariants-anchor` suggester as a
  pinned git dependency. Its AI path is opt-in and fully
  provenance-tagged; see "Suggester provenance model" below.

## What is NOT AI-touched

- **The CI verdict.** Pass or fail is a function of the fuzz and
  regression runs against the built programs on the pinned toolchain,
  not of any model output. Both CI legs (`clean-passes` and
  `planted-twin-detects`) run on every push and their exit codes are
  the ground truth.
- **The engines.** Crucible and Trident are upstream projects;
  nothing in them is authored or modified here.
- **The license posture.** LICENSE, NOTICE attributions, and the
  compliance G1 sign-off are human-authored and human-reviewed. The
  Crucible license-state resolution flagged in NOTICE is a
  primary-source verification by compliance_lead, not a model call.
- **The coverage-map cells.** Each non-empty cell in
  `docs/coverage_map.md` carries a file-level citation of the form
  `owner/repo/path @ <pin>`. Citation content is verified on the
  cited tree by the human author and reconciled with the source
  substrate at the G3 claims-review gate; no model output is trusted
  without primary-source verification.
- **The operator's final-pass sign-off decisions** and the gate
  reviews (license compliance, code quality, claims) that precede
  the public flip.

## Suggester provenance model

The default `suggest` path in the consumed `cf-invariants-anchor`
crates is heuristic (no AI call) and produces
`InvariantSource::Heuristic { suggester_version }` candidates with no
AI-disclosure banner. The AI path is opt-in behind an explicit
`--ai` flag on `cf-invariants-anchor suggest`, and every candidate
returned through it carries:

```
source: InvariantSource::AiSuggested {
    model: "<model id, e.g. claude-sonnet-4-6>",
    prompt_version: "<pinned prompt id, e.g. suggest-v0.2.0>",
    timestamp_utc: "<ISO-8601, e.g. 2026-07-02T14:31:17Z>",
}
```

The three fields are load-bearing:

- `model` names the model that returned the candidate; a re-run
  against a newer model records the newer model verbatim.
- `prompt_version` names the pinned prompt id at candidate time; a
  suggester prompt change bumps the version so historical audit-log
  entries are attributable to the exact prompt that produced them.
- `timestamp_utc` marks the moment of the response; audit-log
  reconciliation depends on the ordering across candidates.

The scorecard renderer's AI-disclosure banner fires whenever any such
candidate is part of a run. The banner gate is type-driven (counting
candidates whose source `.is_ai_suggested()`), so there is no in-band
way to bypass disclosure; the emitted fixture's disclosure header
carries the same provenance line so a reviewer opening the fixture
source sees the AI provenance without needing to open the scorecard.

## Captured live runs and non-claims

One captured live run exists in the CaliperForge org history: the
`cf-invariants-anchor` first-fifteen suggester exercise recorded in
the source-substrate eval
`agents/adversarial_research_lead/outbox/solana_value_over_free_eval_2026-07-02.md`
(section A5, `n = 1` captured live run). This atlas discloses the
suggester layer and its provenance pattern; it makes no
AI-vs-heuristic lift claim, no "AI finds bugs the engines cannot"
claim, and no throughput or cost claim tied to the AI path.
Provenance is disclosed; it is not scored. If future captured runs
land, the sample size in this section is updated and the non-claim
discipline continues to hold until a defensible measurement across
enough targets is in the record.

## Transport default: `MockTransport`

The `library/` crate is configured with `MockTransport` by default:
a canned, deterministic response used unless the CLI is built with
the `live-ai` feature AND the opt-in `CF_INVARIANTS_ANCHOR_LIVE_AI=1`
environment variable AND an API key are all present. CI runs the
mock path only, so the green badge is reproducible by anyone without
keys or network access, and no external API call happens as part of
a green CI run on any commit. The `--ai` flag on `cf-invariants-anchor
suggest` is a separately opted-in code path (mock transport by
default; live transport requires the feature and the env var and the
key). A live-transport suggester call always writes an audit-log
entry with token counts, cost, and a SHA-256 of the response body,
recorded in `~/.cf-invariants-anchor/audit.log` or the path the
`CF_INVARIANTS_ANCHOR_AUDIT_LOG` env var overrides.

## Audit trail

- Every commit lists the author (Michael Moffett, operator at
  CaliperForge) and is operator-clean.
- Both CI legs (`clean-passes` and `planted-twin-detects`) run on
  every push; the planted leg prints its `INVARIANT VIOLATED` markers
  in the job output so reviewers can see the catches at the cited
  commit.
- The scorecard artifacts regenerated by CI carry the AI-disclosure
  banner when any candidate in the run is AI-sourced; when the run
  is heuristic-only the banner is absent by design.
