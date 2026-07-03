# Toolchain pins

Decided in the Phase 1 build spec (section 3). Do not relitigate in
tickets; any change to this table is a spec change.

| Item | Pin | Notes |
|---|---|---|
| anchor-lang / anchor-spl | `= 1.0.1` exactly, everywhere | All five existing CaliperForge Solana repos are green on this pin; `anchor-spl = "=1.0.1"` is the one net-new dependency for the `collateral_mint_ref` case |
| Crucible | v0.2.0 | Primary emit target and fuzz engine rail |
| Trident | v0.12.0 stable (2025-11-27) | Emit target pin for T-satlas-03. Do NOT chase v0.13.0-rc; the v0.12 assertion-style API is the emit shape we render |
| solana-cli | 2.1.21, pinned EXPLICITLY in CI | Never "latest". See the PATH-prefix note below |
| platform-tools | v1.52 | The bundled default in Agave v2.1.21 is v1.43, too old for some anchor-lang 1.0.1 deps; pass `--tools-version v1.52` to `cargo build-sbf` |
| Rust (host) | 1.96.0 via `rust-toolchain.toml` | Governs host-side builds; platform-tools carries its own rustc for the sbf target |
| cf-invariants-anchor git-dep | pinned at main SHA `1b3905e9f756d87dba488e1e47b4d704bfbaa3f5` | Re-pinned to the merge tag when the T-satlas-03 upstream branch lands |
| Load-bearing build check | `cargo build-sbf` in CI, not just `cargo check` | `cargo check` does not exercise the BPF linker (spike Q4 drift note 3) |
| Fuzz budget per CI cell | see `docs/design_notes.md` | Fixed iteration budget, wall-clock capped at 5 min/cell |

## The solana-cli 2.1.21 PATH-prefix note

Developer machines commonly default to a newer solana-cli (the spike
host defaults to 4.0.1). Local reproduction of CI results requires the
pinned release on PATH before any `cargo build-sbf` invocation:

```sh
export PATH=~/.local/share/solana/install/releases/2.1.21/solana-release/bin:$PATH
```

`scripts/check-solana-pin.sh` verifies the active version and prints
this line if the pin is not active. Run it before any local build.

## Known toolchain trap

Agave v2.1.21 ships a cargo-build-sbf that panics with a NotFound error
if `~/.cache/solana` does not exist (fresh runners). CI pre-creates the
directory before the first invocation; do the same on a fresh machine:

```sh
mkdir -p ~/.cache/solana
```
