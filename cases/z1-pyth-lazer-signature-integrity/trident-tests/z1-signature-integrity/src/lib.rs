// Trident v0.12.0 harness entry for C-Z1 (Pyth Lazer signature integrity).
// Secondary leg: firm-shape overlap credit to Ackee (Trident) for the
// Solana-Anchor stateful-invariant fuzzer archetype. Additive-only.
//
// Wording note: Trident already ships invariant / stateful fuzzing for
// Solana. This crate does NOT ship a fuzzer; it exposes the
// class-shaped `#[flow]` methods so `trident fuzz` picks them up. The
// primary property-based leg for this case is
// `solana-program-test` / LiteSVM under `../../tests/`.

#![allow(unused_imports)]

use trident_fuzz::prelude::*;

pub mod flows {
    pub mod invariant_signer_matches_ed25519;
}
