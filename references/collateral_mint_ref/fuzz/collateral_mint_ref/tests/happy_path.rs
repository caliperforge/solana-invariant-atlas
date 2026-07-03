// Happy-path unit leg for collateral_mint_ref (clean twin).
//
// Verifies the well-behaved path: initialize the bank, deposit from the
// authorized collateral mint, mint receipts 1:1 against pending. The
// bank's `total_receipts_minted` is expected to equal the requested
// mint amount, and the fixture-side `expected_receipts` ledger walks
// alongside it.
//
// Runs on both twins (the fuzz crate is byte-identical between twins);
// on the planted twin the constraint is absent but the authorized
// happy path still behaves the same, so this test is unchanged there.
// The value of the unit leg is the clean-twin coverage over the
// happy path; the acceptance criterion only requires clean-twin pass.

use anchor_lang::solana_program::system_program;
use crucible_test_context::{AccountBuilderBase, TestContext};
use ::collateral_mint_ref::*;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;

const INITIAL_LAMPORTS: u64 = 10_000_000_000;
const INITIAL_TOKEN_BALANCE: u64 = 1_000_000_000;

#[test]
fn happy_path_authorized_deposit_then_mint_receipts() {
    let mut ctx = TestContext::new();
    let program_id = Pubkey::new_from_array(ID.to_bytes());
    ctx.add_program(&program_id, "../../target/deploy/collateral_mint_ref.so")
        .expect("collateral_mint_ref.so must be built before running happy_path");

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
    let receipt_mint = ctx
        .create_mint()
        .pubkey(Keypair::new().pubkey())
        .mint_authority(bank_authority)
        .decimals(6)
        .create()
        .unwrap();

    let bank_vault_authorized = ctx
        .create_token_account()
        .pubkey(Keypair::new().pubkey())
        .mint(authorized_mint)
        .token_owner(bank_authority)
        .amount(0)
        .create()
        .unwrap();
    let user_authorized_ta = ctx
        .create_token_account()
        .pubkey(Keypair::new().pubkey())
        .mint(authorized_mint)
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
        .unwrap();

    let amount: u64 = 42_000;
    ctx.program(program_id)
        .call(instruction::Deposit { amount })
        .accounts(accounts::Deposit {
            bank: bank_pda,
            collateral_account: user_authorized_ta,
            bank_vault: bank_vault_authorized,
            user: user.pubkey(),
            token_program: spl_token::ID,
        })
        .signers(&[&user])
        .send()
        .unwrap();

    ctx.program(program_id)
        .call(instruction::MintReceipts { amount })
        .accounts(accounts::MintReceipts {
            bank: bank_pda,
            receipt_mint,
            receipt_account: user_receipt_ta,
            bank_authority,
            user: user.pubkey(),
            token_program: spl_token::ID,
        })
        .signers(&[&user])
        .send()
        .unwrap();

    let bank: Bank = ctx.read_anchor_account::<Bank>(&bank_pda).unwrap();
    assert_eq!(bank.total_receipts_minted, amount);
    assert_eq!(bank.pending_receipts, 0);
}
