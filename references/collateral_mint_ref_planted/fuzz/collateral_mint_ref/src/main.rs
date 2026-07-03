// invariant_collateral_authority
//
// Hand-authored on the `render_crucible_balance` ledger pattern from
// `experiments/cf-invariants-anchor/crates/cf-invariants-anchor-emit/
// src/lib.rs`. Registry/emit generation of this sub-shape rides
// T-satlas-03; this file is written directly so T-satlas-02 does not
// block on it.
//
// Target: Crucible v0.2.0 (asymmetric-research/crucible).
// Class:  access_control, missing-Anchor-constraint family,
//         mint-authority sub-shape.
// Source: Hand-authored (T-satlas-02). No AI suggestion in this fixture.
//
// Property (`collateral_authority`): total receipts minted equals the
// fixture-tracked sum of receipt amounts that were minted against
// deposits drawn from the AUTHORIZED collateral mint the bank was
// initialized with. The fixture also configures a SECOND mint the bank
// never authorized, so the fuzz walk naturally exercises the wrong-mint
// path.
//
// Fixture-side ledger fields:
//   `expected_pending_authorized: u128` - pending deposit balance the
//       bank should recognize, per the fixture, that traces back to
//       authorized-mint deposits.
//   `expected_receipts: u128` - total receipts that should have been
//       minted, per the fixture, given only authorized-mint deposits.
//
// The invariant snapshots after each action:
//   bank.total_receipts_minted == expected_receipts.
//
// On the clean twin, unauthorized deposits bounce off the mint-equality
// constraint and never enter `bank.pending_receipts`, so the two counters
// stay in lock-step and the invariant holds. On the planted twin, the
// constraint is absent, unauthorized deposits accumulate into
// `bank.pending_receipts`, and any subsequent `mint_receipts` call drives
// `bank.total_receipts_minted` past `expected_receipts` - the invariant
// trips and the FUZZ_FINDING carries the marker string
// `INVARIANT VIOLATED collateral_authority`.

#![allow(unused_imports)]

use crucible_fuzzer::anchor_lang::system_program;
use crucible_fuzzer::*;
// `::` prefix disambiguates the program crate from a `collateral_mint_ref`
// module re-exported via `crucible_fuzzer::*` (rustc E0659 otherwise).
use ::collateral_mint_ref::*;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use std::rc::Rc;

const INITIAL_LAMPORTS: u64 = 10_000_000_000;
const INITIAL_TOKEN_BALANCE: u64 = 1_000_000_000;

#[derive(Clone)]
struct CollateralMintFixture {
    ctx: TestContext,
    program_id: Pubkey,
    payer: Rc<Keypair>,
    user: Rc<Keypair>,
    bank_pda: Pubkey,
    bank_authority: Pubkey,
    bank_authority_bump: u8,
    authorized_mint: Pubkey,
    unauthorized_mint: Pubkey,
    receipt_mint: Pubkey,
    /// Bank-owned SPL token account for the AUTHORIZED collateral mint.
    /// The bank was initialized with `authorized_mint` as its collateral
    /// mint; this is the vault the authorized deposit path targets.
    bank_vault_authorized: Pubkey,
    /// Bank-owned SPL token account for the UNAUTHORIZED second mint.
    /// The bank never authorized this mint, but the account is still
    /// owned by the bank_authority PDA. On the clean twin, Deposit's
    /// mint-equality constraint rejects a call that reaches for this
    /// pair; on the planted twin the call completes and drives the
    /// pending_receipts / expected_receipts drift.
    bank_vault_wrong_mint: Pubkey,
    user_authorized_ta: Pubkey,
    user_unauthorized_ta: Pubkey,
    user_receipt_ta: Pubkey,
    /// Fixture-side ledger: pending balance the bank should recognize
    /// per authorized-mint deposits alone. Walked on every successful
    /// authorized deposit / mint_receipts action.
    expected_pending_authorized: u128,
    /// Fixture-side ledger: total receipts the bank should have minted
    /// given only authorized-mint deposits.
    expected_receipts: u128,
}

#[fuzz_fixture]
impl CollateralMintFixture {
    pub fn setup() -> Self {
        let mut ctx = TestContext::new();
        let program_id = Pubkey::new_from_array(ID.to_bytes());
        ctx.add_program(&program_id, "../../target/deploy/collateral_mint_ref.so")
            .unwrap();

        let payer = Rc::new(Keypair::new());
        let user = Rc::new(Keypair::new());
        for kp in [&payer, &user] {
            ctx.create_account()
                .pubkey(kp.pubkey())
                .lamports(INITIAL_LAMPORTS)
                .owner(system_program::ID)
                .create()
                .unwrap();
        }

        let (bank_pda, _) =
            Pubkey::find_program_address(&[BANK_SEED], &program_id);
        let (bank_authority, bank_authority_bump) =
            Pubkey::find_program_address(&[BANK_AUTHORITY_SEED], &program_id);

        // Two SPL mints: one authorized (the bank was initialized with
        // it), one that the bank never authorized. The fuzz walk uses
        // both to exercise the wrong-mint path.
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
        // Receipt mint owned by the bank-authority PDA.
        let receipt_mint = ctx
            .create_mint()
            .pubkey(Keypair::new().pubkey())
            .mint_authority(bank_authority)
            .decimals(6)
            .create()
            .unwrap();

        // Two bank-owned SPL token accounts: one for the authorized
        // collateral mint (the vault the bank was initialized around)
        // and one for the second mint the bank never authorized. Both
        // are owned by the bank-authority PDA. Their existence is what
        // lets the planted twin's wrong-mint deposit complete: the SPL
        // token program will only transfer between accounts of the
        // same mint, so the wrong-mint path needs a wrong-mint vault.
        let bank_vault_authorized = ctx
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

        // User token accounts: one for each collateral mint, one for
        // the receipt mint. The two collateral TAs start pre-funded so
        // the fuzz walk can immediately deposit.
        let user_authorized_ta = ctx
            .create_token_account()
            .pubkey(Keypair::new().pubkey())
            .mint(authorized_mint)
            .token_owner(user.pubkey())
            .amount(INITIAL_TOKEN_BALANCE)
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

        // Bank initialization records `authorized_mint` as the
        // authorized collateral mint and `receipt_mint` as the receipt
        // mint. Anything else is unauthorized by definition.
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
            .signers(&[&*payer])
            .send()
            .unwrap();

        Self {
            ctx,
            program_id,
            payer,
            user,
            bank_pda,
            bank_authority,
            bank_authority_bump,
            authorized_mint,
            unauthorized_mint,
            receipt_mint,
            bank_vault_authorized,
            bank_vault_wrong_mint,
            user_authorized_ta,
            user_unauthorized_ta,
            user_receipt_ta,
            expected_pending_authorized: 0,
            expected_receipts: 0,
        }
    }

    /// Deposit `amount` of the AUTHORIZED collateral mint. Walks the
    /// fixture ledger's `expected_pending_authorized` on success.
    pub fn action_deposit_authorized(
        &mut self,
        #[range(1..100_000)] amount: u64,
    ) -> bool {
        let ok = self
            .ctx
            .program(self.program_id)
            .call(instruction::Deposit { amount })
            .accounts(accounts::Deposit {
                bank: self.bank_pda,
                collateral_account: self.user_authorized_ta,
                bank_vault: self.bank_vault_authorized,
                user: self.user.pubkey(),
                token_program: spl_token::ID,
            })
            .signers(&[&*self.user])
            .send()
            .map(|o| o.is_success())
            .unwrap_or(false);
        if ok {
            self.expected_pending_authorized = self
                .expected_pending_authorized
                .saturating_add(amount as u128);
        }
        ok
    }

    /// Deposit `amount` of the UNAUTHORIZED second mint. On the clean
    /// twin, the mint-equality constraint on `Deposit.collateral_account`
    /// rejects the call; on the planted twin, the call succeeds and the
    /// bank's `pending_receipts` grows even though the fixture ledger
    /// treats this as an unauthorized deposit that should not back
    /// receipt minting.
    pub fn action_deposit_wrong_mint(
        &mut self,
        #[range(1..100_000)] amount: u64,
    ) -> bool {
        // We intentionally do NOT update `expected_pending_authorized`
        // here regardless of outcome - the fixture ledger only tracks
        // authorized-mint pending balance. On the clean twin the call
        // fails; on the planted twin it succeeds and drives the drift
        // the invariant is designed to catch.
        self.ctx
            .program(self.program_id)
            .call(instruction::Deposit { amount })
            .accounts(accounts::Deposit {
                bank: self.bank_pda,
                collateral_account: self.user_unauthorized_ta,
                bank_vault: self.bank_vault_wrong_mint,
                user: self.user.pubkey(),
                token_program: spl_token::ID,
            })
            .signers(&[&*self.user])
            .send()
            .map(|o| o.is_success())
            .unwrap_or(false)
    }

    /// Mint `amount` receipt tokens against the bank's pending balance.
    /// Fixture ledger only credits receipts against pending balance
    /// that traces back to authorized-mint deposits; any excess is a
    /// class violation and drives the invariant to fail.
    pub fn action_mint_receipts(
        &mut self,
        #[range(1..100_000)] amount: u64,
    ) -> bool {
        let ok = self
            .ctx
            .program(self.program_id)
            .call(instruction::MintReceipts { amount })
            .accounts(accounts::MintReceipts {
                bank: self.bank_pda,
                receipt_mint: self.receipt_mint,
                receipt_account: self.user_receipt_ta,
                bank_authority: self.bank_authority,
                user: self.user.pubkey(),
                token_program: spl_token::ID,
            })
            .signers(&[&*self.user])
            .send()
            .map(|o| o.is_success())
            .unwrap_or(false);
        if ok {
            // Fixture ledger: credit only the portion of `amount` that
            // is backed by authorized-mint pending balance. Any excess
            // is what the class violation lets through - the invariant
            // trips when `bank.total_receipts_minted` walks past this.
            let credited = (amount as u128).min(self.expected_pending_authorized);
            self.expected_pending_authorized -= credited;
            self.expected_receipts = self.expected_receipts.saturating_add(credited);
        }
        ok
    }
}

// collateral_authority invariant.
//
// After every action, the bank's on-chain `total_receipts_minted` must
// equal the fixture-side `expected_receipts` ledger. Any drift means
// the program credited a receipt against collateral it should have
// rejected at the constraint layer - the class violation.
//
// The assertion message carries the marker string CI greps for. The
// exact form is `INVARIANT VIOLATED collateral_authority`; Crucible
// prints it as part of the `[FUZZ_FINDING]` summary line.
#[invariant_test]
fn invariant_collateral_authority(fixture: &mut CollateralMintFixture) {
    let bank: Bank = fixture
        .ctx
        .read_anchor_account::<Bank>(&fixture.bank_pda)
        .expect("bank PDA initialized in setup");
    fuzz_assert_eq!(
        bank.total_receipts_minted as u128,
        fixture.expected_receipts,
        "INVARIANT VIOLATED collateral_authority: total_receipts_minted={} expected_receipts={}",
        bank.total_receipts_minted,
        fixture.expected_receipts
    );
}
