# CITATION - C-Z1 (Pyth Lazer Solana signature integrity)

## Vendored source

**Repository:** [`pyth-network/pyth-crosschain`](https://github.com/pyth-network/pyth-crosschain)
**Path:** `lazer/contracts/solana/programs/pyth-lazer-solana-contract/`
**Pin:** `eb7f460ab8d1c73c6c8b4942891c9fe74a589121` (7-char shorthand: `eb7f460`)
**License:** Apache-2.0 (repo-level LICENSE; per-file license status
inherited from the crate's `Cargo.toml` `license = "Apache-2.0"` key.
Upstream sources at this pin do not carry file-level SPDX headers;
the vendored files are byte-identical to upstream and inherit the same
convention).
**Fetched:** 2026-07-06.

The clean twin at `clean/pyth-lazer-solana-contract/` is a byte-exact
snapshot of that upstream directory at the pinned commit. The planted
twin at `planted/pyth-lazer-solana-contract/` differs from the clean
twin ONLY in `src/signature.rs` and a single-line helper-signature
update in `src/lib.rs`; all other files are byte-identical to the
clean twin.

## Finding

**Audit:** [Pyth Lazer Solana - Zellic Audit Report, 2025-01-17](https://github.com/Zellic/publications/blob/master/Pyth%20Lazer%20Solana%20-%20Zellic%20Audit%20Report.pdf)
**Finding:** 3.1 "Signature bypass"
**Severity:** Critical / Impact Critical / Likelihood Low / Status Fixed

## Fix

**PR:** [`pyth-network/pyth-crosschain#2250`](https://github.com/pyth-network/pyth-crosschain/pull/2250)
"Lazer solana audit fixes"
**Merge commit:** [`eb7f460ab8d1c73c6c8b4942891c9fe74a589121`](https://github.com/pyth-network/pyth-crosschain/commit/eb7f460ab8d1c73c6c8b4942891c9fe74a589121)
**Merged to:** `main` on 2025-01-24.

## No upstream code mutated

This citation carries no discovery claim against upstream Pyth Lazer.
The vendored source is unmodified in the clean twin. The planted twin
lives ONLY under `planted/` and is a defender-side regression fixture
that mechanically reverses the PR #2250 hunk in our teaching-scale
harness. It is not a reproduction of the finding against any deployed
Pyth Lazer program.
