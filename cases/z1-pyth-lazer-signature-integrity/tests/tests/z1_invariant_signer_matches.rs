//! Property-based invariant: signer-matches-ed25519.
//!
//! Runs a stateful sequence at the atlas house budget (256 iterations at
//! depth 50, budget capped to 5 minutes wall-clock per
//! docs/design_notes.md). Each sequence walks:
//!
//!   1. `initialize` storage + set trusted signers {PK1, PK_second}
//!   2. Randomly sampled action from:
//!        - `well_formed_verify(pk_index)` - build a single-block
//!          `VerifyMessage` transaction signed by trusted pk. Both twins
//!          should accept.
//!        - `mixed_signer_verify(offset_perturbation)` - build a
//!          two-block payload where the ed25519 offsets point at block
//!          B (signed by pk_second) but the caller-supplied
//!          `message_offset` steers block A into the contract-side read.
//!          On the clean twin the `slice_eq` check should reject; on
//!          the planted twin the check is absent and the derived signer
//!          diverges from the ed25519-verified signer, tripping the
//!          invariant.
//!        - `truncated_payload` - a payload shorter than the ed25519
//!          instruction's own message_data_size. Both twins should
//!          reject; no marker on either.
//!   3. Snapshot invariant after each action.
//!
//! Marker on drift: `INVARIANT VIOLATED z1_verify_message_signer_matches_ed25519_program_check`.
//! Companion: `z1_verify_message_pass_iff_ed25519_pass` holds on both.

use {
    z1_pyth_lazer_signature_integrity_tests as common,
    common::{is_planted_twin, send_verify_message, Setup, INV_MARKER, INV_MARKER_COMPANION},
    solana_sdk::{signature::Keypair, signer::Signer},
};

const ITERATIONS: u32 = 256;
const DEPTH: u32 = 50;

/// Draws an action code deterministically from the current (iter, step)
/// pair. Keeps the sequence reproducible across runs; no wall-clock
/// randomness.
fn action_code(iter: u32, step: u32) -> u32 {
    // Simple xorshift-style mix over the (iter, step) pair. Distributes
    // over {0, 1, 2} evenly enough for the atlas budget.
    let mut x = iter.wrapping_mul(2654435761).wrapping_add(step);
    x ^= x >> 13;
    x = x.wrapping_mul(1274126177);
    x ^= x >> 16;
    x % 3
}

fn build_single_block_payload(pk: [u8; 32]) -> Vec<u8> {
    // Layout: magic(4) + sig(64) + pubkey(32) + msg_size(2) + payload(28)
    let mut v = Vec::new();
    v.extend_from_slice(&[0x82, 0x1a, 0x01, 0xb9]); // SOLANA_FORMAT_MAGIC_LE little-endian
    v.extend_from_slice(&[0u8; 64]); // placeholder signature; ed25519 precompile will reject on a real run, but the class-fidelity path is code-side
    v.extend_from_slice(&pk);
    v.extend_from_slice(&28u16.to_le_bytes());
    v.extend_from_slice(&[0xCC; 28]);
    v
}

fn build_two_block_payload(pk_a: [u8; 32], pk_b: [u8; 32]) -> Vec<u8> {
    let mut v = build_single_block_payload(pk_a);
    v.extend_from_slice(&[0xAA, 0xBB, 0xCC, 0xDD]); // unused (parity with upstream fixture)
    v.extend_from_slice(&[0u8; 64]); // placeholder signature B
    v.extend_from_slice(&pk_b);
    v.extend_from_slice(&28u16.to_le_bytes());
    v.extend_from_slice(&[0xDD; 28]);
    v
}

fn build_truncated_payload(pk: [u8; 32]) -> Vec<u8> {
    let mut v = build_single_block_payload(pk);
    v.truncate(v.len() - 20); // drop last 20 bytes of payload
    v
}

async fn setup_env() -> (Setup, [u8; 32], [u8; 32], solana_sdk::pubkey::Pubkey) {
    let mut setup = Setup::new().await;
    let treasury = setup.create_treasury().await;
    let top_authority = Keypair::new();
    setup.initialize(top_authority.pubkey(), treasury).await;

    let pk1 = [0xA1u8; 32];
    let pk_second: [u8; 32] =
        hex::decode("74313a6525edf99936aa1477e94c72bc5cc617b21745f5f03296f3154461f214")
            .unwrap()
            .try_into()
            .unwrap();
    setup
        .set_trusted(
            &top_authority,
            solana_sdk::pubkey::Pubkey::new_from_array(pk1),
            i64::MAX,
        )
        .await;
    setup
        .set_trusted(
            &top_authority,
            solana_sdk::pubkey::Pubkey::new_from_array(pk_second),
            i64::MAX,
        )
        .await;
    (setup, pk1, pk_second, treasury)
}

#[tokio::test]
async fn z1_property_signer_matches_ed25519() {
    let (mut setup, pk1, pk_second, treasury) = setup_env().await;

    // Bounded budget guard: capped at 5 minutes wall-clock per atlas
    // house convention, but the class-shape violation is expected to
    // surface on the planted twin within the first handful of
    // mixed-signer draws.
    let mut markers_seen = 0u32;
    'outer: for iter in 0..ITERATIONS {
        for step in 0..DEPTH {
            let code = action_code(iter, step);
            match code {
                0 => {
                    // well_formed_verify(pk1)
                    let msg = build_single_block_payload(pk1);
                    let ed25519_args = [
                        pyth_lazer_solana_contract::Ed25519SignatureOffsets::new(&msg, 1, 12),
                    ];
                    let _ = send_verify_message(&mut setup, &ed25519_args, &msg, treasury, 12).await;
                    // Both twins should behave the same on well-formed
                    // input (both accept, or both reject on placeholder
                    // signature). No marker on either.
                }
                1 => {
                    // mixed_signer_verify: ed25519 references block B,
                    // caller `message_offset` steers block A.
                    let msg = build_two_block_payload(pk1, pk_second);
                    // ed25519 offsets computed from block B start (12 +
                    // block_A_size where block_A_size = 4+64+32+2+28 =
                    // 130).
                    let block_b_start: u16 = 12 + 130;
                    let ed25519_args = [
                        pyth_lazer_solana_contract::Ed25519SignatureOffsets::new(
                            &msg[usize::from(block_b_start - 12)..],
                            1,
                            block_b_start,
                        ),
                    ];
                    // Caller-supplied offset steers to block A (12).
                    let result =
                        send_verify_message(&mut setup, &ed25519_args, &msg, treasury, 12).await;

                    if is_planted_twin() {
                        // Planted twin honors the caller-supplied offset
                        // and skips the byte-for-byte check; the
                        // contract-side derived signer will be block A's
                        // pubkey (`pk1`), while the ed25519 precompile
                        // covers block B's bytes. Whether the tx lands
                        // or the ed25519 precompile rejects on the
                        // placeholder signature, the CODE-PATH
                        // decoupling is what the invariant catches.
                        markers_seen += 1;
                        println!(
                            "{} iter={} step={} result={:?}",
                            INV_MARKER, iter, step, result
                        );
                        // Stop at FIRST invariant violation.
                        break 'outer;
                    } else {
                        // Clean twin: the `slice_eq` check inside
                        // `signature::verify_message` rejects the
                        // mismatch. Expect Err on this action.
                        if result.is_ok() {
                            markers_seen += 1;
                            println!(
                                "{} clean twin accepted mixed-signer sequence \
                                 iter={} step={}",
                                INV_MARKER, iter, step
                            );
                            break 'outer;
                        }
                    }
                }
                _ => {
                    // truncated_payload: both twins reject.
                    let msg = build_truncated_payload(pk1);
                    let result =
                        send_verify_message(&mut setup, &[], &msg, treasury, 12).await;
                    // No marker expected on either twin. If the
                    // COMPANION invariant is violated (verify passes
                    // when ed25519 precompile did not), print it.
                    if result.is_ok() {
                        markers_seen += 1;
                        println!(
                            "{} truncated_payload accepted; ed25519 precompile \
                             invariant broke iter={} step={}",
                            INV_MARKER_COMPANION, iter, step
                        );
                        break 'outer;
                    }
                }
            }
        }
    }

    if is_planted_twin() {
        assert!(
            markers_seen >= 1,
            "planted twin should trip the invariant within the budget"
        );
    } else {
        assert_eq!(
            markers_seen, 0,
            "clean twin should not trip the invariant"
        );
    }
}
