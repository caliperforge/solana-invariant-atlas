# adopt/example: worked-example artifacts

Proof-of-flow artifacts from executing `adopt/README.md` end to end
against the atlas's own reference program
(`references/collateral_mint_ref`). Full transcript:
`adopt/worked-example-log.md`. This directory stays at exactly this
one example.

- `idl/collateral_mint_ref.json`: the reference program's Anchor IDL,
  hand-transcribed from
  `references/collateral_mint_ref/programs/collateral_mint_ref/src/lib.rs`
  with real Anchor discriminators (the reference builds with
  `cargo build-sbf`, so no generated IDL ships with it; an adopter
  gets theirs from `anchor build`).
- `emitted/invariant_pending_receipts_conservation.rs`: the Crucible
  fixture emitted by the pinned `cf-invariants-anchor` CLI from that
  IDL (walkthrough step 4), committed verbatim as generated tool
  output at the recorded CLI build. Comment wording inside it is the
  upstream emitter's, not hand-edited.
