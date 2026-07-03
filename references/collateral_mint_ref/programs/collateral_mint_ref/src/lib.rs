// collateral_mint_ref.
//
// Synthetic Anchor collateralized-mint bank. A single Bank config PDA
// records the authorized collateral mint and the authorized receipt
// mint; users deposit SPL tokens of the authorized collateral mint
// into a bank vault, then mint 1:1 receipt tokens against the pending
// deposit balance the bank tracks.
//
// Property under test (`collateral_authority`): receipt tokens are
// only ever minted against deposits drawn from the authorized
// collateral mint recorded on the bank at initialization time. The
// clean twin enforces this via a mint-equality constraint on
// `deposit`'s collateral_account (see the `Deposit` accounts struct
// below). The planted twin drops that one constraint line and nothing
// else; see the twin at `references/collateral_mint_ref_planted/`.
//
// This program is a specification carrier for the missing-Anchor-
// constraint family (mint-authority sub-shape); it is entirely our
// own code and cites no upstream program.
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

declare_id!("Co1LMintRef11111111111111111111111111111111");

/// Seed for the Bank config PDA.
pub const BANK_SEED: &[u8] = b"bank";
/// Seed for the bank-authority PDA (owns the bank vault; mint authority
/// for the receipt mint).
pub const BANK_AUTHORITY_SEED: &[u8] = b"bank_authority";

#[program]
pub mod collateral_mint_ref {
    use super::*;

    pub fn initialize_bank(
        ctx: Context<InitializeBank>,
        collateral_mint: Pubkey,
        receipt_mint: Pubkey,
    ) -> Result<()> {
        let bank = &mut ctx.accounts.bank;
        bank.collateral_mint = collateral_mint;
        bank.receipt_mint = receipt_mint;
        bank.bank_authority_bump = ctx.bumps.bank_authority;
        bank.pending_receipts = 0;
        bank.total_receipts_minted = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, BankError::InvalidAmount);

        // anchor-lang 1.0.x CpiContext::new takes a Pubkey program id
        // (changed from the 0.30.x AccountInfo signature).
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.key(),
            Transfer {
                from: ctx.accounts.collateral_account.to_account_info(),
                to: ctx.accounts.bank_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, amount)?;

        let bank = &mut ctx.accounts.bank;
        bank.pending_receipts = bank
            .pending_receipts
            .checked_add(amount)
            .ok_or(BankError::Overflow)?;
        Ok(())
    }

    pub fn mint_receipts(ctx: Context<MintReceipts>, amount: u64) -> Result<()> {
        require!(amount > 0, BankError::InvalidAmount);
        require!(
            amount <= ctx.accounts.bank.pending_receipts,
            BankError::InsufficientPending
        );

        // Signer seeds for the bank-authority PDA, which holds mint
        // authority over the receipt mint. Bind the bump byte to a
        // stack local so the seed slice borrow stays valid for the
        // duration of the CPI.
        let bump = ctx.accounts.bank.bank_authority_bump;
        let bump_arr = [bump];
        let seeds: &[&[u8]] = &[BANK_AUTHORITY_SEED, &bump_arr];
        let signer_seeds: &[&[&[u8]]] = &[seeds];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            MintTo {
                mint: ctx.accounts.receipt_mint.to_account_info(),
                to: ctx.accounts.receipt_account.to_account_info(),
                authority: ctx.accounts.bank_authority.to_account_info(),
            },
            signer_seeds,
        );
        token::mint_to(cpi_ctx, amount)?;

        let bank = &mut ctx.accounts.bank;
        bank.pending_receipts = bank
            .pending_receipts
            .checked_sub(amount)
            .ok_or(BankError::Underflow)?;
        bank.total_receipts_minted = bank
            .total_receipts_minted
            .checked_add(amount)
            .ok_or(BankError::Overflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeBank<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Bank::INIT_SPACE,
        seeds = [BANK_SEED],
        bump,
    )]
    pub bank: Account<'info, Bank>,

    /// CHECK: derivation-only PDA that owns the bank vault and holds
    /// mint authority over the receipt mint. Not initialized on-chain
    /// by this program; the bump is recorded on Bank for later CPIs.
    #[account(seeds = [BANK_AUTHORITY_SEED], bump)]
    pub bank_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [BANK_SEED], bump)]
    pub bank: Account<'info, Bank>,

    // Clean: the mint-equality constraint binds `collateral_account`
    // to the collateral mint the bank was initialized with. The
    // planted twin drops this line; that is the entire diff.
    #[account(
        mut,
        constraint = collateral_account.mint == bank.collateral_mint @ BankError::WrongCollateralMint,
    )]
    pub collateral_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub bank_vault: Account<'info, TokenAccount>,

    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct MintReceipts<'info> {
    #[account(mut, seeds = [BANK_SEED], bump)]
    pub bank: Account<'info, Bank>,

    #[account(mut, address = bank.receipt_mint)]
    pub receipt_mint: Account<'info, Mint>,

    #[account(mut)]
    pub receipt_account: Account<'info, TokenAccount>,

    /// CHECK: bank-authority PDA; signs the receipt mint_to via
    /// derived seeds.
    #[account(seeds = [BANK_AUTHORITY_SEED], bump = bank.bank_authority_bump)]
    pub bank_authority: UncheckedAccount<'info>,

    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct Bank {
    pub collateral_mint: Pubkey,
    pub receipt_mint: Pubkey,
    pub bank_authority_bump: u8,
    pub pending_receipts: u64,
    pub total_receipts_minted: u64,
}

#[error_code]
pub enum BankError {
    #[msg("Invalid amount (must be non-zero)")]
    InvalidAmount,
    #[msg("collateral_account.mint does not match bank.collateral_mint")]
    WrongCollateralMint,
    #[msg("Requested receipt amount exceeds pending deposit balance")]
    InsufficientPending,
    Overflow,
    Underflow,
}
