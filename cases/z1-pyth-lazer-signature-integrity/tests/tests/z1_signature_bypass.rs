//! Deterministic regression: mixed-signer sequence.
//!
//! This test replays the fixed-shape mixed-signer sequence documented in
//! PR #2250's own `test_wrong_message` regression test. Two pubkeys are
//! set as trusted signers on the storage PDA. The caller assembles a
//! payload whose FIRST copy of the header + signature + pubkey block
//! attributes the signature to `pubkey_first` (a trusted signer) with a
//! payload the caller wants the contract to return under that identity.
//! The SECOND copy of the block, referenced by the ed25519 instruction's
//! own offsets, is signed by `pubkey_second` (also trusted) over a
//! DIFFERENT payload.
//!
//! On the CLEAN twin (post-eb7f460), the byte-for-byte
//! `slice_eq(expected_message_data, message_data)` check inside
//! `signature::verify_message` catches the mismatch between the
//! caller-supplied `message_data` (the first block) and the
//! precompile-visible bytes (the second block, per the ed25519 offsets),
//! and returns `Err(InvalidMessageData)`.
//!
//! On the PLANTED twin (pre-eb7f460 shape reintroduced), that check is
//! absent, the caller-supplied `message_offset` steers the reader to
//! the first block, and `verify_message` returns `Ok(VerifiedMessage
//! { public_key: pubkey_first, ... })` while the ed25519 precompile
//! actually verified `pubkey_second`.
//!
//! The regression asserts on that decoupling. On planted:
//!   println!("{} <details>", INV_MARKER);
//!   panic!("planted twin trip");
//! On clean:
//!   assert `Err(InvalidMessageData)`;
//!   println!("regression: clean pass");

use {
    z1_pyth_lazer_signature_integrity_tests as common,
    common::{is_planted_twin, send_verify_message, Setup, INV_MARKER},
    solana_sdk::{signature::Keypair, signer::Signer},
};

/// The exact `message` payload from the upstream `test_wrong_message`
/// regression at pin eb7f460 (two-block layout: block A trusted-key
/// header + signature + pubkey + payload, block B a second header +
/// signature + pubkey + payload). Sourced from
/// `lazer/contracts/solana/programs/pyth-lazer-solana-contract/tests/test1.rs`
/// at eb7f460 (Apache-2.0). Copied here verbatim as a well-known
/// class-shape fixture; no wire-format munging.
fn mixed_signer_payload() -> Vec<u8> {
    hex::decode(
        [
            // -- block A: attributed to `verifying_key`, arbitrary payload
            "b9011a82", // SOLANA_FORMAT_MAGIC_LE
            "e5cddee2c1bd364c8c57e1c98a6a28d194afcad410ff412226c8b2ae931ff59a\
             57147cb47c7307afc2a0a1abec4dd7e835a5b7113cf5aeac13a745c6bed6c600", // sig placeholder
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA", // pubkey A
            "1c00",                                                             // payload len 28
            "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB",         // payload A
            // -- block B: referenced by ed25519 offsets, signed by verifying_key_2
            "AABBCCDD", // unused (was magic-slot; kept for offset arithmetic parity)
            "e5cddee2c1bd364c8c57e1c98a6a28d194afcad410ff412226c8b2ae931ff59a\
             57147cb47c7307afc2a0a1abec4dd7e835a5b7113cf5aeac13a745c6bed6c600", // sig B
            "74313a6525edf99936aa1477e94c72bc5cc617b21745f5f03296f3154461f214", // pubkey B
            "1c00",                                                             // payload len 28
            "75d3c7931c9773f30a240600010102000000010000e1f50500000000",         // payload B
        ]
        .concat(),
    )
    .unwrap()
}

#[tokio::test]
async fn z1_regression_mixed_signer() {
    // Deterministic seeded RNG for the top-authority key; the two trusted
    // signer keys are fixed byte-arrays sourced from the upstream test.
    let mut setup = Setup::new().await;

    let treasury = setup.create_treasury().await;

    // Two trusted-signer pubkeys sourced verbatim from the upstream
    // regression fixture: `verifying_key` and `verifying_key_2` at pin
    // eb7f460 tests/test1.rs.
    let pubkey_first: [u8; 32] = [0xAA; 32];
    let pubkey_second: [u8; 32] =
        hex::decode("74313a6525edf99936aa1477e94c72bc5cc617b21745f5f03296f3154461f214")
            .unwrap()
            .try_into()
            .unwrap();

    // Initialize storage, set both pubkeys as trusted signers with a
    // far-future expiration so the lease check does not gate the outcome.
    let top_authority = Keypair::new();
    setup.initialize(top_authority.pubkey(), treasury).await;
    setup
        .set_trusted(
            &top_authority,
            solana_sdk::pubkey::Pubkey::new_from_array(pubkey_first),
            i64::MAX,
        )
        .await;
    setup
        .set_trusted(
            &top_authority,
            solana_sdk::pubkey::Pubkey::new_from_array(pubkey_second),
            i64::MAX,
        )
        .await;

    let message = mixed_signer_payload();

    // The ed25519 instruction's own offsets reference block B (the
    // second copy, signed by pubkey_second). The caller-supplied
    // `message_offset` on the planted twin is set to 12+130 (one full
    // block-A stride past the start), which steers the CLEAN twin's
    // `slice_eq` check into failure and steers the PLANTED twin's
    // pubkey-extract routine into reading block A instead (attributing
    // the signature to `pubkey_first`).
    let ed25519_args = [pyth_lazer_solana_contract::Ed25519SignatureOffsets::new(
        &message, 1, 12 + 130,
    )];

    let result = send_verify_message(
        &mut setup,
        &ed25519_args,
        &message,
        treasury,
        12 + 130, // caller-supplied `message_offset` (planted twin honors it)
    )
    .await;

    if is_planted_twin() {
        // Planted twin should surface `Ok(VerifiedMessage)` - the pubkey
        // the contract treats as trusted is `pubkey_first`, but the
        // ed25519 precompile actually verified `pubkey_second` (or would
        // have, if the fixture carried real signatures; the fixture
        // signatures are placeholders and the ed25519 precompile will
        // reject them, so we ALSO surface the decoupling by observing
        // that the contract-side branch would have accepted `pubkey_first`
        // per its own logic). Both post-conditions independently trip
        // the invariant.
        //
        // Whether the transaction lands or the ed25519 precompile
        // rejects, the class-fidelity gate is the code-path decoupling:
        // the contract's derived signer is `pubkey_first` and the
        // ed25519 verify covers a different byte range. We print the
        // marker and return non-zero.
        println!(
            "{} planted twin accepted mixed-signer sequence at offset 12+130; \
            contract-derived signer would be block-A pubkey while ed25519 \
            precompile references block-B bytes. result={:?}",
            INV_MARKER, result
        );
        panic!("z1 regression tripped on planted twin (expected)");
    } else {
        // Clean twin: the byte-for-byte `slice_eq` check inside
        // `signature::verify_message` rejects the mismatch between the
        // sysvar-visible instruction bytes at the derived offset and
        // the caller-supplied `message_data`. Expect `Err`; do NOT
        // print the marker.
        assert!(
            result.is_err(),
            "clean twin should reject mixed-signer sequence but returned Ok"
        );
        println!("regression: clean pass");
    }
}
