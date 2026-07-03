// invariant_collateral_authority
//
// Emitted by cf-invariants-anchor-emit for the access_control.collateral_authority class.
// Target: Trident v0.12.0 (ackee-blockchain/trident).
// Source: Heuristic (suggester v0.2.0). No AI suggestion in this candidate.
//
// mint_receipts rejects when invoked by anyone other than the authorized signer
//
// Fixture-side flags: sticky booleans that stay `false` on a correct
// program. Any observed unauthorized-signer or wrong-mint success is
// recorded via Trident's `#[flow]` handlers and asserted after every
// flow via the base `assert_eq_marker` helper. The probe flow name is
// `flow_wrong_signer_mint_receipts` per Trident's own naming convention.

#![allow(unused_imports)]

use trident_fuzz::prelude::*;
use ::collateral_mint_ref::*;

/// Cap on the fuzzer's per-flow amount range. Matches the Crucible leg's
/// range attribute so the two harnesses walk comparable state spaces.
const AMOUNT_CEILING: u64 = 1_000_000;

pub struct CollateralmintrefAccessTridentFixture {
    /// Sticky flag: set to `true` if the program ever accepted an
    /// unauthorized-signer call. Trips the access_control marker.
    pub unauthorized_success_observed: bool,
    /// Sticky flag: set to `true` if any flow_wrong_mint call
    /// returned success. Trips the collateral_authority marker.
    pub wrong_mint_success_observed: bool,
}

impl CollateralmintrefAccessTridentFixture {
    #[init]
    pub fn init(&mut self) {
        self.unauthorized_success_observed = false;
        self.wrong_mint_success_observed = false;
    }

    /// Base probe: signer-not-authorized attempt. Trident's own naming
    /// convention is `flow_wrong_signer_*`  - atlas emit follows it
    /// verbatim.
    #[flow]
    pub fn flow_wrong_signer_mint_receipts(&mut self, amount: u64) {
        let amount = amount % AMOUNT_CEILING;
        if amount == 0 { return; }
        let ok = program_call_with_unauthorized_signer("mint_receipts", amount);
        if ok {
            self.unauthorized_success_observed = true;
        }
    }
    /// Sub-shape probe: attempt the deposit path with a `collateral_account`
    /// whose mint does not match the bank's authorized collateral mint.
    /// A correct program rejects the call (mint-equality constraint);
    /// a program missing that constraint accepts it and the sticky flag
    /// trips. Marker: `INVARIANT VIOLATED collateral_authority`.
    #[flow]
    pub fn flow_wrong_mint(&mut self, amount: u64) {
        let amount = amount % AMOUNT_CEILING;
        if amount == 0 { return; }
        let ok = program_call_with_wrong_mint("deposit", amount);
        if ok {
            self.wrong_mint_success_observed = true;
        }
    }


    /// Whole-run invariant check.
    pub fn invariant_collateral_authority(&self) {
        assert_eq_marker(
            self.unauthorized_success_observed,
            false,
            "INVARIANT VIOLATED access_control",
        );
        // Sub-shape: collateral_authority. Sticky flag must stay clean.
        assert_eq_marker(
            self.wrong_mint_success_observed,
            false,
            "INVARIANT VIOLATED collateral_authority",
        );
    }
}
