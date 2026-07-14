// Deterministic regression leg for the collateral_authority invariant.
//
// Plays a fixed action sequence and prints the marker
// `INVARIANT VIOLATED collateral_authority` with rc=1 on a divergence
// between `bank.total_receipts_minted` and the fixture ledger. Stops
// at the FIRST invariant violation; it detects, it does not maximize.
//
// The sequence, per T-satlas-02 acceptance criteria:
//   1. initialize_bank (setup already does this)
//   2. deposit(amount) drawn from the unauthorized second mint
//   3. mint_receipts(amount)
//   4. assert bank.total_receipts_minted == expected_receipts
//
// On the clean twin, step 2 fails at Anchor's mint-equality constraint,
// so bank.pending_receipts stays 0, step 3 fails at the pending-balance
// require!(), and both counters stay at 0. The regression exits 0 with
// no marker.
//
// On the planted twin, step 2 succeeds (constraint removed),
// bank.pending_receipts becomes `amount`, step 3 mints `amount` receipt
// tokens, and bank.total_receipts_minted = amount while the fixture
// ledger's expected_receipts = 0. The regression prints the marker and
// exits 1.
//
// Seeded reachability: if `REACHABILITY_SEED` is set (16 hex chars,
// optional 0x prefix), the deposit amount + payer/user secret keys are
// derived from that seed via `StdRng`. The sequence and assertion are
// unchanged. This lets `ci/reachability_leg.sh` rotate through the
// canonical 16-seed set and certify the class fires on every seed. If
// `REACHABILITY_SEED` is absent, the fixed values below apply and
// existing developer flow (`cargo run --release --bin regression`)
// remains unchanged.

use anchor_lang::solana_program::system_program;
use crucible_test_context::{AccountBuilderBase, TestContext};
use ::collateral_mint_ref::*;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use std::process::ExitCode;

const INITIAL_LAMPORTS: u64 = 10_000_000_000;
const INITIAL_TOKEN_BALANCE: u64 = 1_000_000_000;
const REGRESSION_AMOUNT_FIXED: u64 = 12_345;

/// Parse `REACHABILITY_SEED` into a 32-byte seed. Accepts an optional
/// `0x` prefix + exactly 16 hex chars (64 bits of entropy). The 8-byte
/// value is tiled four times to fill the 32-byte seed; distinct inputs
/// always yield distinct expanded seeds. Mirrors the Soroban-lane
/// `parse_seed_env` shape so a reader can diff the two legs and see
/// only the harness delta.
fn parse_seed_env() -> Option<[u8; 32]> {
    let raw = std::env::var("REACHABILITY_SEED").ok()?;
    let hex_str = raw.trim().trim_start_matches("0x").trim_start_matches("0X");
    assert_eq!(
        hex_str.len(),
        16,
        "REACHABILITY_SEED must be exactly 16 hex chars (64-bit); got {:?}",
        raw
    );
    let mut base = [0u8; 8];
    for i in 0..8 {
        base[i] = u8::from_str_radix(&hex_str[i * 2..i * 2 + 2], 16)
            .expect("REACHABILITY_SEED must be valid hexadecimal");
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = base[i % 8];
    }
    Some(out)
}

fn keypair_from_rng(rng: &mut StdRng) -> Keypair {
    let mut secret = [0u8; 32];
    rng.fill_bytes(&mut secret);
    Keypair::new_from_array(secret)
}

fn main() -> ExitCode {
    let (regression_amount, payer, user, seeded) = match parse_seed_env() {
        Some(seed) => {
            let mut rng = StdRng::from_seed(seed);
            // Draw amount from the same range the fuzz strategy uses
            // (1..=1_000_000) so seeded inputs stay inside the class
            // trigger's valid range. Payer / user keypairs are derived
            // from the same RNG so a given seed reproduces byte-for-byte.
            let amount = rng.gen_range(1u64..=1_000_000u64);
            let payer = keypair_from_rng(&mut rng);
            let user = keypair_from_rng(&mut rng);
            eprintln!(
                "regression: REACHABILITY_SEED consumed; amount={} payer={} user={}",
                amount,
                payer.pubkey(),
                user.pubkey(),
            );
            (amount, payer, user, true)
        }
        None => {
            eprintln!(
                "regression: REACHABILITY_SEED not set; using fixed amount={} and fresh Keypair::new()",
                REGRESSION_AMOUNT_FIXED
            );
            (REGRESSION_AMOUNT_FIXED, Keypair::new(), Keypair::new(), false)
        }
    };
    let _ = seeded;

    let mut ctx = TestContext::new();
    let program_id = Pubkey::new_from_array(ID.to_bytes());
    ctx.add_program(&program_id, "../../target/deploy/collateral_mint_ref.so")
        .expect("collateral_mint_ref.so must be built before running regression");
    for kp in [&payer, &user] {
        ctx.create_account()
            .pubkey(kp.pubkey())
            .lamports(INITIAL_LAMPORTS)
            .owner(system_program::ID)
            .create()
            .unwrap();
    }

    let (bank_pda, _) = Pubkey::find_program_address(&[BANK_SEED], &program_id);
    let (bank_authority, _) =
        Pubkey::find_program_address(&[BANK_AUTHORITY_SEED], &program_id);

    let authorized_mint = ctx
        .create_mint()
        .pubkey(Keypair::new().pubkey())
        .mint_authority(payer.pubkey())
        .decimals(6)
        .create()
        .unwrap();
    let unauthorized_mint = ctx
        .create_mint()
        .pubkey(Keypair::new().pubkey())
        .mint_authority(payer.pubkey())
        .decimals(6)
        .create()
        .unwrap();
    let receipt_mint = ctx
        .create_mint()
        .pubkey(Keypair::new().pubkey())
        .mint_authority(bank_authority)
        .decimals(6)
        .create()
        .unwrap();

    // Two bank-owned SPL token accounts, one per mint. The unauthorized
    // second vault is what lets the wrong-mint deposit path complete on
    // the planted twin (SPL token transfer requires same-mint accounts).
    let _bank_vault_authorized = ctx
        .create_token_account()
        .pubkey(Keypair::new().pubkey())
        .mint(authorized_mint)
        .token_owner(bank_authority)
        .amount(0)
        .create()
        .unwrap();
    let bank_vault_wrong_mint = ctx
        .create_token_account()
        .pubkey(Keypair::new().pubkey())
        .mint(unauthorized_mint)
        .token_owner(bank_authority)
        .amount(0)
        .create()
        .unwrap();
    let user_unauthorized_ta = ctx
        .create_token_account()
        .pubkey(Keypair::new().pubkey())
        .mint(unauthorized_mint)
        .token_owner(user.pubkey())
        .amount(INITIAL_TOKEN_BALANCE)
        .create()
        .unwrap();
    let user_receipt_ta = ctx
        .create_token_account()
        .pubkey(Keypair::new().pubkey())
        .mint(receipt_mint)
        .token_owner(user.pubkey())
        .amount(0)
        .create()
        .unwrap();

    // Step 1: initialize.
    ctx.program(program_id)
        .call(instruction::InitializeBank {
            collateral_mint: authorized_mint,
            receipt_mint,
        })
        .accounts(accounts::InitializeBank {
            bank: bank_pda,
            bank_authority,
            payer: payer.pubkey(),
            system_program: system_program::ID,
        })
        .signers(&[&payer])
        .send()
        .expect("initialize_bank must succeed");

    // Step 2: deposit from the unauthorized mint into its matching
    // bank_vault. Clean twin's mint-equality constraint on
    // collateral_account rejects here; planted twin has the constraint
    // removed and the deposit completes.
    let _ = ctx
        .program(program_id)
        .call(instruction::Deposit {
            amount: regression_amount,
        })
        .accounts(accounts::Deposit {
            bank: bank_pda,
            collateral_account: user_unauthorized_ta,
            bank_vault: bank_vault_wrong_mint,
            user: user.pubkey(),
            token_program: spl_token::ID,
        })
        .signers(&[&user])
        .send();

    // Step 3: mint_receipts against whatever pending balance the bank
    // now sees. Clean twin: pending=0, this fails. Planted twin:
    // pending=REGRESSION_AMOUNT, this succeeds.
    let _ = ctx
        .program(program_id)
        .call(instruction::MintReceipts {
            amount: regression_amount,
        })
        .accounts(accounts::MintReceipts {
            bank: bank_pda,
            receipt_mint,
            receipt_account: user_receipt_ta,
            bank_authority,
            user: user.pubkey(),
            token_program: spl_token::ID,
        })
        .signers(&[&user])
        .send();

    // Step 4: read the bank and compare against the fixture ledger.
    // Fixture ledger: no authorized-mint deposit was made, so
    // expected_receipts = 0.
    let expected_receipts: u64 = 0;
    let bank: Bank = ctx
        .read_anchor_account::<Bank>(&bank_pda)
        .expect("bank PDA readable after regression sequence");

    if bank.total_receipts_minted != expected_receipts {
        println!(
            "INVARIANT VIOLATED collateral_authority: total_receipts_minted={} expected_receipts={}",
            bank.total_receipts_minted, expected_receipts
        );
        // Stops at the FIRST violation; detects, does not maximize.
        return ExitCode::from(1);
    }

    println!(
        "regression: clean pass (total_receipts_minted={}, expected_receipts={})",
        bank.total_receipts_minted, expected_receipts
    );
    ExitCode::SUCCESS
}
