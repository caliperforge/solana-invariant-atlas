# library/ (atlas-driver)

Thin driver over the canonical `cf-invariants-anchor` crates, consumed
as git dependencies pinned at a single rev (see `Cargo.toml` and
`docs/design_notes.md`). It exposes three operations:

```sh
atlas-driver suggest <surface.json>                      # ranked candidates, JSON
atlas-driver emit <surface.json> <index> <crucible|trident>  # rendered fixture source
atlas-driver scorecard <scorecard.json>                  # scorecard markdown
```

The full upstream CLI (IDL ingestion, AI-suggest flag, report paths) is
`cf-invariants-anchor-cli`, a bin-only package. Install it at the SAME
pinned rev:

```sh
cargo install --git https://github.com/caliperforge/cf-invariants-anchor \
  --rev 1b3905e9f756d87dba488e1e47b4d704bfbaa3f5 cf-invariants-anchor-cli
```

Build checks (both must be green on the pinned rails, see
`docs/toolchain.md`):

```sh
cargo check
cargo build-sbf --tools-version v1.52   # with solana-cli 2.1.21 active
```
