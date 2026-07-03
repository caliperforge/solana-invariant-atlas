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

use anchor_lang::solana_program::system_program;
use crucible_test_context::{AccountBuilderBase, TestContext};
use ::collateral_mint_ref::*;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use std::process::ExitCode;

const INITIAL_LAMPORTS: u64 = 10_000_000_000;
const INITIAL_TOKEN_BALANCE: u64 = 1_000_000_000;
const REGRESSION_AMOUNT: u64 = 12_345;

fn main() -> ExitCode {
    let mut ctx = TestContext::new();
    let program_id = Pubkey::new_from_array(ID.to_bytes());
    ctx.add_program(&program_id, "../../target/deploy/collateral_mint_ref.so")
        .expect("collateral_mint_ref.so must be built before running regression");

    let payer = Keypair::new();
    let user = Keypair::new();
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
            amount: REGRESSION_AMOUNT,
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
            amount: REGRESSION_AMOUNT,
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
