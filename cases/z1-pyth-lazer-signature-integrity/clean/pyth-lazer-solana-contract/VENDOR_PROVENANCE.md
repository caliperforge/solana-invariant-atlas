# Vendored source - provenance

## Upstream

- **Repository:** `pyth-network/pyth-crosschain`
- **License:** Apache-2.0 (preserved on every vendored file via the
  `license = "Apache-2.0"` key in `Cargo.toml`; the atlas ships its
  own top-level Apache-2.0 LICENSE + NOTICE crediting the vendored
  crate).
- **Path in upstream:** `lazer/contracts/solana/programs/pyth-lazer-solana-contract/`
- **Pin (commit SHA):** `eb7f460ab8d1c73c6c8b4942891c9fe74a589121`
- **PR merged at pin:** [`pyth-network/pyth-crosschain#2250`](https://github.com/pyth-network/pyth-crosschain/pull/2250)
  "Lazer solana audit fixes"
- **Fetched:** 2026-07-06

## Files vendored (byte-exact from upstream)

- `Cargo.toml` (33 lines) - declares `license = "Apache-2.0"`,
  `repository = "https://github.com/pyth-network/pyth-crosschain"`,
  and the `anchor-lang = "0.30.1"` pin.
- `Xargo.toml` (2 lines)
- `src/lib.rs` (283 lines) - the Anchor program entrypoint and
  storage-model helpers.
- `src/signature.rs` (324 lines) - the security-critical helper. The
  post-fix state at `eb7f460` derives `message_offset` internally
  from `offsets.signature_offset.checked_sub(MAGIC_LEN)` and enforces
  `slice_eq(expected_message_data, message_data)` on lines
  ~215–237. That is the load-bearing check the planted twin removes.

## Vendored, not submoduled

We did not add `pyth-network/pyth-crosschain` as a git submodule
because the upstream is a ~500 MB Cargo/JS monorepo with many
unrelated packages; the crate we need is ~600 LOC. A vendored
snapshot at a pinned SHA is (a) subclass-real in the Cargo idiom
(both twins build the ACTUAL audited crate), (b) reproducible from
this file (the exact upstream URL and commit SHA are here), and (c)
does not drag the rest of the monorepo into the atlas repo. A
follow-up dispatch may swap this snapshot for a proper submodule if
the CEO decides the atlas should carry submodules; the twin sources
below are byte-identical to what the submodule would expose.

## Reproduce the vendor from scratch

```sh
COMMIT=eb7f460ab8d1c73c6c8b4942891c9fe74a589121
BASE="https://raw.githubusercontent.com/pyth-network/pyth-crosschain/${COMMIT}/lazer/contracts/solana/programs/pyth-lazer-solana-contract"

for f in Cargo.toml Xargo.toml src/lib.rs src/signature.rs; do
  mkdir -p "$(dirname "$f")"
  curl -sL "${BASE}/${f}" -o "$f"
done
```

Byte-for-byte diff to this vendored copy should be empty.

## No mutations under `clean/`

The clean twin holds the vendored source unmodified. The planted
twin is a sibling directory that carries a forked `src/signature.rs`
and a single-line helper-signature update in `src/lib.rs`; the
vendored snapshot here is never mutated.
