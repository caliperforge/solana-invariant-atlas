// invariant_pending_receipts_conservation
//
// Emitted by cf-invariants-anchor v0.2.0 for the balance_conservation class.
// Target: Crucible v0.2.0 (asymmetric-research/crucible).
// Source: Heuristic (suggester v0.2.0). No AI suggestion in this candidate.
//
// Bank.pending_receipts == fixture-tracked sum of deposits − sum of withdrawals
//
// Fixture-side bookkeeping field: `expected_pending_receipts: u128` - walked
// through every action and asserted against `Bank.pending_receipts`
// after each step.

#![allow(unused_imports)]

use crucible_fuzzer::anchor_lang::system_program;
use crucible_fuzzer::*;
// `::` prefix disambiguates the program crate from a `vault_ref`
// module re-exported via `crucible_fuzzer::*` (rustc E0659 otherwise).
use ::collateral_mint_ref::*;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use std::rc::Rc;

const INITIAL_BALANCE: u64 = 10_000_000_000;

#[derive(Clone)]
struct CollateralmintrefFixture {
    ctx: TestContext,
    program_id: Pubkey,
    depositor: Rc<Keypair>,
    vault_pda: Pubkey,
    /// Fixture-side ledger. Walked through every action; asserted
    /// against on-chain `Bank.pending_receipts` after each step.
    expected_pending_receipts: u128,
}

#[fuzz_fixture]
impl CollateralmintrefFixture {
    pub fn setup() -> Self {
        let mut ctx = TestContext::new();
        let program_id = Pubkey::new_from_array(ID.to_bytes());
        ctx.add_program(&program_id, "../../target/deploy/collateral_mint_ref.so")
            .unwrap();

        let depositor = Rc::new(Keypair::new());
        ctx.create_account()
            .pubkey(depositor.pubkey())
            .lamports(INITIAL_BALANCE)
            .owner(system_program::ID)
            .create()
            .unwrap();

        let (vault_pda, _) = Pubkey::find_program_address(
            &[b"vault", depositor.pubkey().as_ref()],
            &program_id,
        );

        ctx.program(program_id)
            .call(instruction::Initialize {})
            .accounts(accounts::Initialize {
                vault: vault_pda,
                depositor: depositor.pubkey(),
                system_program: system_program::ID,
            })
            .signers(&[&*depositor])
            .send()
            .unwrap();

        Self {
            ctx,
            program_id,
            depositor,
            vault_pda,
            expected_pending_receipts: 0,
        }
    }

    pub fn action_deposit(&mut self, #[range(1..1_000_000)] amount: u64) -> bool {
        let ok = self.ctx
            .program(self.program_id)
            .call(instruction::Deposit { amount })
            .accounts(accounts::Deposit {
                vault: self.vault_pda,
                depositor: self.depositor.pubkey(),
                system_program: system_program::ID,
            })
            .signers(&[&*self.depositor])
            .send()
            .map(|o| o.is_success())
            .unwrap_or(false);
        if ok {
            // Mirror the on-chain bookkeeping move.
            self.expected_pending_receipts = self.expected_pending_receipts.saturating_add(amount as u128);
        }
        ok
    }

    pub fn action_withdraw(&mut self, #[range(1..1_000_000)] amount: u64) -> bool {
        let ok = self.ctx
            .program(self.program_id)
            .call(instruction::Withdraw { amount })
            .accounts(accounts::Withdraw {
                vault: self.vault_pda,
                depositor: self.depositor.pubkey(),
            })
            .signers(&[&*self.depositor])
            .send()
            .map(|o| o.is_success())
            .unwrap_or(false);
        if ok {
            self.expected_pending_receipts = self.expected_pending_receipts.saturating_sub(amount as u128);
        }
        ok
    }
}

// Balance-conservation invariant.
//
// After every action, the on-chain `Bank.pending_receipts` must equal
// the fixture-side ledger (`expected_pending_receipts`). Any drift indicates
// the program's bookkeeping has decoupled from the lamports it
// actually moved - the classic conservation violation.
#[invariant_test]
fn invariant_pending_receipts_conservation(fixture: &mut CollateralmintrefFixture) {
    let vault: Bank = fixture
        .ctx
        .read_anchor_account::<Bank>(&fixture.vault_pda)
        .expect("vault PDA initialized in setup");
    fuzz_assert_eq!(
        vault.pending_receipts as u128,
        fixture.expected_pending_receipts,
        "Bank.pending_receipts drift: on-chain={} expected={}",
        vault.pending_receipts,
        fixture.expected_pending_receipts
    );
}
