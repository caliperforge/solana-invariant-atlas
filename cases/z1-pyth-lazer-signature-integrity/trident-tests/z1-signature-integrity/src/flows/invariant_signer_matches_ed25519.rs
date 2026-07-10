// invariant_signer_matches_ed25519
//
// Trident v0.12.0 flow encoding of the C-Z1 property:
//   the public key `verify_message` treats as the trusted signer of
//   `message_data` MUST equal the public key the ed25519 precompile
//   verified over the byte range identified by the ed25519 instruction's
//   own offsets.
//
// Sticky flag: `mixed_signer_success_observed`. Stays `false` on the
// clean twin (byte-for-byte `slice_eq` check catches the mismatch).
// Trips on the planted twin (check absent). Marker:
//   INVARIANT VIOLATED z1_verify_message_signer_matches_ed25519_program_check

#![allow(unused_imports)]

use trident_fuzz::prelude::*;
use ::pyth_lazer_solana_contract::*;

pub struct Z1SignatureIntegrityTridentFixture {
    /// Sticky flag: set to `true` if any mixed-signer flow returned a
    /// contract-side success where the derived signer differed from the
    /// pubkey the ed25519 precompile actually verified. Trips the z1
    /// marker.
    pub mixed_signer_success_observed: bool,
}

impl Z1SignatureIntegrityTridentFixture {
    #[init]
    pub fn init(&mut self) {
        self.mixed_signer_success_observed = false;
    }

    /// Base probe: well-formed single-block verify. Both twins accept
    /// on well-formed input; no flag is toggled.
    #[flow]
    pub fn flow_well_formed_verify(&mut self, pk_seed: u64) {
        let _ = program_call_verify_well_formed(pk_seed);
    }

    /// Class-shape probe: two-block payload where the ed25519
    /// instruction's offsets reference block B (a different signer)
    /// while the caller-supplied `message_offset` steers the
    /// contract-side reader into block A. A correct program rejects
    /// this shape (`InvalidMessageData`). A program missing the
    /// byte-for-byte check accepts it and the sticky flag trips.
    #[flow]
    pub fn flow_mixed_signer_verify(&mut self, pk_a_seed: u64, pk_b_seed: u64) {
        let (contract_ok, contract_signer, ed25519_signer) =
            program_call_verify_mixed_signer(pk_a_seed, pk_b_seed);
        if contract_ok && contract_signer != ed25519_signer {
            self.mixed_signer_success_observed = true;
        }
    }

    /// Truncated-payload probe: payload shorter than the ed25519
    /// instruction's `message_data_size`. Both twins should reject; no
    /// marker on either.
    #[flow]
    pub fn flow_truncated_payload_verify(&mut self, pk_seed: u64) {
        let _ = program_call_verify_truncated(pk_seed);
    }

    /// Whole-run invariant check.
    pub fn invariant_signer_matches_ed25519(&self) {
        assert_eq_marker(
            self.mixed_signer_success_observed,
            false,
            "INVARIANT VIOLATED z1_verify_message_signer_matches_ed25519_program_check",
        );
    }
}

// The three `program_call_*` helpers are provided by the Trident
// scaffold's runtime shim in the atlas library (see
// `experiments/solana-invariant-atlas/library/src/`). Their bodies are
// consistent with the primary property-based leg at
// `../../tests/tests/z1_invariant_signer_matches.rs`, so the two
// harnesses walk comparable state spaces.
extern "Rust" {
    fn program_call_verify_well_formed(pk_seed: u64) -> bool;
    fn program_call_verify_mixed_signer(pk_a_seed: u64, pk_b_seed: u64) -> (bool, [u8; 32], [u8; 32]);
    fn program_call_verify_truncated(pk_seed: u64) -> bool;
}
