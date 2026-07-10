//! Shared test-harness helpers for the C-Z1 case.
//!
//! The two test binaries - `tests/z1_signature_bypass.rs` (deterministic
//! regression) and `tests/z1_invariant_signer_matches.rs` (property-based
//! sequence over solana-program-test / LiteSVM) - share the fixtures below.
//! Neither binary owns any twin-specific knowledge: the twin selection is
//! done at Cargo-manifest build time via `scripts/select-twin.sh`, and
//! the `pyth_lazer_solana_contract` crate they link against is the twin
//! whose path currently sits in `Cargo.toml`.
//!
//! Framing-discipline note: this crate uses "trusted" / "untrusted" /
//! "mixed-signer" vocabulary. No attacker persona, no gain computation,
//! no extraction ledger. Handlers stop at the FIRST invariant violation.

use {
    anchor_lang::{prelude::AccountMeta, InstructionData},
    pyth_lazer_solana_contract::{ed25519_program_args, ANCHOR_DISCRIMINATOR_BYTES},
    solana_program_test::{BanksClient, BanksClientError, ProgramTest},
    solana_sdk::{
        account::Account,
        ed25519_program,
        hash::Hash,
        instruction::Instruction,
        pubkey::{Pubkey, PUBKEY_BYTES},
        signature::Keypair,
        signer::Signer,
        system_instruction, system_program, sysvar,
        transaction::Transaction,
    },
    std::env,
};

/// Invariant marker string. On a twin where the signer the contract
/// treats as trusted decouples from the pubkey the ed25519 precompile
/// actually verified, tests print this string verbatim to stdout and
/// return `Err`. The atlas CI grep asserts on this exact string.
pub const INV_MARKER: &str = "INVARIANT VIOLATED z1_verify_message_signer_matches_ed25519_program_check";

/// Companion marker for the ed25519-pass-iff-verify-pass invariant.
/// Held by both twins on well-formed inputs; asserted defensively.
pub const INV_MARKER_COMPANION: &str = "INVARIANT VIOLATED z1_verify_message_pass_iff_ed25519_pass";

/// Whether the linked `pyth_lazer_solana_contract` crate is the planted
/// twin. Set via the `CASE_TWIN` env var at test time and threaded to
/// the deterministic regression bin, which asserts on it.
pub fn is_planted_twin() -> bool {
    matches!(
        env::var("CASE_TWIN").as_deref(),
        Ok("planted") | Ok("PLANTED")
    )
}

pub fn program_test() -> ProgramTest {
    if env::var("SBF_OUT_DIR").is_err() {
        // Point at the twin's build output; scripts/build-both.sh populates
        // both trees. If the tests are run manually, set SBF_OUT_DIR
        // explicitly to override.
        let twin = if is_planted_twin() { "planted" } else { "clean" };
        env::set_var(
            "SBF_OUT_DIR",
            format!(
                "{}/../{}/target/sbf-solana-solana/release",
                env::var("CARGO_MANIFEST_DIR").unwrap(),
                twin,
            ),
        );
    }
    ProgramTest::new(
        "pyth_lazer_solana_contract",
        pyth_lazer_solana_contract::ID,
        None,
    )
}

pub struct Setup {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
}

impl Setup {
    pub async fn with_program_test(program_test: ProgramTest) -> Self {
        let (banks_client, payer, recent_blockhash) = program_test.start().await;
        Self {
            banks_client,
            payer,
            recent_blockhash,
        }
    }

    pub async fn new() -> Self {
        Self::with_program_test(program_test()).await
    }

    pub async fn create_treasury(&mut self) -> Pubkey {
        let treasury =
            Pubkey::create_with_seed(&self.payer.pubkey(), "treasury", &system_program::ID)
                .unwrap();
        let mut tx = Transaction::new_with_payer(
            &[system_instruction::create_account_with_seed(
                &self.payer.pubkey(),
                &treasury,
                &self.payer.pubkey(),
                "treasury",
                10_000_000,
                0,
                &system_program::ID,
            )],
            Some(&self.payer.pubkey()),
        );
        tx.sign(&[&self.payer], self.recent_blockhash);
        self.banks_client.process_transaction(tx).await.unwrap();
        treasury
    }

    pub async fn initialize(&mut self, top_authority: Pubkey, treasury: Pubkey) {
        let mut tx = Transaction::new_with_payer(
            &[Instruction::new_with_bytes(
                pyth_lazer_solana_contract::ID,
                &pyth_lazer_solana_contract::instruction::Initialize {
                    top_authority,
                    treasury,
                }
                .data(),
                vec![
                    AccountMeta::new(self.payer.pubkey(), true),
                    AccountMeta::new(pyth_lazer_solana_contract::STORAGE_ID, false),
                    AccountMeta::new_readonly(system_program::ID, false),
                ],
            )],
            Some(&self.payer.pubkey()),
        );
        tx.sign(&[&self.payer], self.recent_blockhash);
        self.banks_client.process_transaction(tx).await.unwrap();
    }

    pub async fn set_trusted(&mut self, top_authority: &Keypair, trusted: Pubkey, expires_at: i64) {
        let mut tx = Transaction::new_with_payer(
            &[Instruction::new_with_bytes(
                pyth_lazer_solana_contract::ID,
                &pyth_lazer_solana_contract::instruction::Update {
                    trusted_signer: trusted,
                    expires_at,
                }
                .data(),
                vec![
                    AccountMeta::new(top_authority.pubkey(), true),
                    AccountMeta::new(pyth_lazer_solana_contract::STORAGE_ID, false),
                ],
            )],
            Some(&self.payer.pubkey()),
        );
        tx.sign(&[&self.payer, top_authority], self.recent_blockhash);
        self.banks_client.process_transaction(tx).await.unwrap();
    }
}

/// Build a `VerifyMessage` transaction with the given ed25519-args and
/// message payload. On the CLEAN twin the `.data` (VerifyMessage) does
/// not carry `message_offset`; on the PLANTED twin it does. We encode
/// via `pyth_lazer_solana_contract::instruction::VerifyMessage`, which
/// is generated by anchor-lang from whichever twin sits at the path
/// dependency, so this call resolves to the twin-appropriate wire
/// format automatically.
pub fn verify_message_tx(
    payer: &Keypair,
    ed25519_args: &[pyth_lazer_solana_contract::Ed25519SignatureOffsets],
    message: &[u8],
    treasury: Pubkey,
    _message_offset_for_planted: u16,
    recent_blockhash: Hash,
) -> Transaction {
    let ix_verify = build_verify_ix(payer, message, treasury, _message_offset_for_planted);
    let ix_ed25519 = Instruction::new_with_bytes(
        ed25519_program::ID,
        &ed25519_program_args(ed25519_args),
        vec![],
    );
    let mut tx =
        Transaction::new_with_payer(&[ix_ed25519, ix_verify], Some(&payer.pubkey()));
    tx.sign(&[payer], recent_blockhash);
    tx
}

/// Build the contract-side `VerifyMessage` instruction. The wire shape
/// is emitted by whichever twin's IDL macros are in scope: clean drops
/// the `message_offset` field, planted carries it. Callers pass the
/// planted-side offset unconditionally; the clean twin's `Discriminator`
/// deserializer simply ignores trailing bytes.
#[cfg(not(feature = "twin_planted_wire"))]
fn build_verify_ix(
    payer: &Keypair,
    message: &[u8],
    treasury: Pubkey,
    _message_offset: u16,
) -> Instruction {
    Instruction::new_with_bytes(
        pyth_lazer_solana_contract::ID,
        &pyth_lazer_solana_contract::instruction::VerifyMessage {
            message_data: message.to_vec(),
            ed25519_instruction_index: 0,
            signature_index: 0,
        }
        .data(),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(pyth_lazer_solana_contract::STORAGE_ID, false),
            AccountMeta::new(treasury, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(sysvar::instructions::ID, false),
        ],
    )
}

#[cfg(feature = "twin_planted_wire")]
fn build_verify_ix(
    payer: &Keypair,
    message: &[u8],
    treasury: Pubkey,
    message_offset: u16,
) -> Instruction {
    // Enable when linking against the planted twin: the IDL adds
    // `message_offset` to the VerifyMessage struct.
    Instruction::new_with_bytes(
        pyth_lazer_solana_contract::ID,
        &pyth_lazer_solana_contract::instruction::VerifyMessage {
            message_data: message.to_vec(),
            ed25519_instruction_index: 0,
            signature_index: 0,
            message_offset,
        }
        .data(),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(pyth_lazer_solana_contract::STORAGE_ID, false),
            AccountMeta::new(treasury, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(sysvar::instructions::ID, false),
        ],
    )
}

/// Send the verify tx and surface the raw `BanksClient` result. Returns
/// `Ok(())` iff the ed25519 pre-compile and the contract's
/// `verify_message` both succeeded. Preserves the transaction-error
/// shape for the caller to inspect.
pub async fn send_verify_message(
    setup: &mut Setup,
    ed25519_args: &[pyth_lazer_solana_contract::Ed25519SignatureOffsets],
    message: &[u8],
    treasury: Pubkey,
    message_offset: u16,
) -> Result<(), BanksClientError> {
    let tx = verify_message_tx(
        &setup.payer,
        ed25519_args,
        message,
        treasury,
        message_offset,
        setup.recent_blockhash,
    );
    setup.banks_client.process_transaction(tx).await
}

/// Sanity re-export so the test binaries do not have to double-import.
pub use solana_sdk::{
    account, hash, instruction, pubkey, signature, signer, system_transaction,
    sysvar as sdk_sysvar, transaction,
};

// Retained account-import shape for parity with upstream test1.rs.
#[allow(dead_code)]
pub fn placeholder_account(len: usize) -> Account {
    Account {
        lamports: 0,
        data: vec![0u8; len],
        owner: system_program::ID,
        executable: false,
        rent_epoch: u64::MAX,
    }
}
#[allow(dead_code)]
pub const PLACEHOLDER_PUBKEY_BYTES: usize = PUBKEY_BYTES;
#[allow(dead_code)]
pub const PLACEHOLDER_DISCRIM_BYTES: usize = ANCHOR_DISCRIMINATOR_BYTES;
