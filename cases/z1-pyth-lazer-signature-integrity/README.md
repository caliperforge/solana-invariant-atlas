# C-Z1 - Pyth Lazer Solana signature verification integrity

> AI-disclosure banner: this case (source layout, tests, this README, scripts,
> and the paired scorecards) was authored with AI involvement, disclosed at the
> point of use per `AI_DISCLOSURE.md` at the atlas root.

## What class this is

Signature-verification integrity family. The invariant asserts that the
public key `verify_message` treats as the trusted signer equals the public
key the ed25519 precompile actually verified a signature over - a specific
class of "trusted-input decoupling" bug where a helper accepts a
caller-supplied offset instead of deriving it from the sysvar-visible
ed25519 instruction.

## Why this class matters

Signature-checking helpers that trust caller-supplied offsets into
their own instruction data can decouple the identity the helper
attributes a message to from the identity the underlying signature
precompile actually verified. Zellic reported an instance of this
class in the Pyth Lazer Solana contract; the Pyth Data Association
fixed it in PR #2250. The actionable specification below stands on its
own without the finding text.

## The subclass-real gate

Both twins build the ACTUAL `pyth-lazer-solana-contract` crate at the
post-fix commit `eb7f460ab8d1c73c6c8b4942891c9fe74a589121`
(Apache-2.0, `pyth-network/pyth-crosschain`). No fabricated demo
program.

- **Clean twin:** vendored snapshot of the crate at `eb7f460`, unmodified.
  Provenance headers preserved on every file. See
  [clean/pyth-lazer-solana-contract/VENDOR_PROVENANCE.md](clean/pyth-lazer-solana-contract/VENDOR_PROVENANCE.md).
- **Planted twin:** forked-single-file override of `src/signature.rs`
  and a matching helper-signature update in `src/lib.rs`. The override
  mechanically reverses the security-critical hunk of PR #2250 -
  re-introduces the `message_offset: u16` caller-supplied argument on
  `verify_message` and drops the byte-for-byte `slice_eq` check between
  the ed25519-instruction-visible message bytes and the caller-supplied
  `message_data` argument. Every other file is byte-identical to
  the clean twin (twin-symmetry).

## The single-hunk twin diff

`diff -r clean/pyth-lazer-solana-contract/src planted/pyth-lazer-solana-contract/src`
(excerpted; full diff mechanical-reverses the PR #2250 `signature.rs`
hunk):

```diff
--- clean/pyth-lazer-solana-contract/src/signature.rs
+++ planted/pyth-lazer-solana-contract/src/signature.rs
@@ pub fn verify_message signature @@
 pub fn verify_message(
     storage: &Storage,
     instructions_sysvar: &AccountInfo,
     message_data: &[u8],
     ed25519_instruction_index: u16,
     signature_index: u8,
+    message_offset: u16,  // planted: pre-eb7f460 caller-supplied offset
 ) -> Result<VerifiedMessage, SignatureVerificationError> {
@@ inside verify_message @@
-    let message_offset = offsets
-        .signature_offset
-        .checked_sub(MAGIC_LEN)
-        .ok_or(SignatureVerificationError::MessageOffsetOverflow)?;
-    let self_instruction = sysvar::instructions::load_instruction_at_checked(
-        self_instruction_index.into(),
-        instructions_sysvar,
-    )
-    .map_err(SignatureVerificationError::LoadInstructionAtFailed)?;
-    let message_end_offset = offsets
-        .message_data_offset
-        .checked_add(offsets.message_data_size)
-        .ok_or(SignatureVerificationError::MessageOffsetOverflow)?;
-    let expected_message_data = self_instruction
-        .data
-        .get(usize::from(message_offset)..usize::from(message_end_offset))
-        .ok_or(SignatureVerificationError::InvalidMessageOffset)?;
-    if !slice_eq(expected_message_data, message_data) {
-        return Err(SignatureVerificationError::InvalidMessageData);
-    }
+    // planted (single-hunk twin diff, the seeded specification violation):
+    // pre-fix behavior - trust the caller-supplied `message_offset` and skip
+    // the byte-for-byte `slice_eq` check between `expected_message_data`
+    // (fetched from self_instruction) and the caller-supplied `message_data`.
+    let expected_signature_offset = message_offset
+        .checked_add(MAGIC_LEN)
+        .ok_or(SignatureVerificationError::MessageOffsetOverflow)?;
+    if offsets.signature_offset != expected_signature_offset {
+        return Err(SignatureVerificationError::InvalidSignatureOffset);
+    }
```

Companion single-line update to `src/lib.rs` on the planted side:

```diff
--- clean/pyth-lazer-solana-contract/src/lib.rs
+++ planted/pyth-lazer-solana-contract/src/lib.rs
@@ verify_message dispatcher @@
     pub fn verify_message(
         ctx: Context<VerifyMessage>,
         message_data: Vec<u8>,
         ed25519_instruction_index: u16,
         signature_index: u8,
+        message_offset: u16,  // planted: caller-supplied offset threaded to signature::verify_message
     ) -> Result<VerifiedMessage> {
```

Everything else - every other source line in `lib.rs`, `Cargo.toml`,
`Xargo.toml` - is byte-identical between the twins.

## Property under test: `z1_verify_message_signer_matches_ed25519_program_check`

**Semantics.** After a `VerifyMessage` transaction, the public key the
contract treats as the trusted signer of `message_data` MUST equal the
public key the ed25519 precompile verified over the byte range identified
by the ed25519 instruction's own offsets. On the clean twin, the
byte-for-byte `slice_eq(expected_message_data, message_data)` check
enforces this (the precompile-visible bytes and the caller-supplied
bytes are forced to match). On the planted twin, a mixed-signer
sequence exists in which the two identities decouple: the caller
directs `verify_message` to read PK1 from `message_data`, while the
ed25519 precompile verified a signature by a different key over
different bytes.

Marker on drift: `INVARIANT VIOLATED z1_verify_message_signer_matches_ed25519_program_check`.

Companion invariant `z1_verify_message_pass_iff_ed25519_pass` (a
defensive: `verify_message` succeeds iff the ed25519 precompile
succeeded and the derived signer is a trusted signer with an unexpired
lease) holds on both twins on well-formed inputs.

## The three legs

1. **Property-based leg** (`cargo test --release --test z1_invariant_signer_matches`)
   - `solana-program-test` / LiteSVM drives a house-shape stateful
   sequence (initialize storage → set trusted signers {PK1,
   PK_untrusted} → well-formed and mixed-signer verify calls). Clean:
   zero markers, rc=0. Planted: markers surface on the mixed-signer
   sequence.
2. **Deterministic regression leg** (`cargo test --release --test
   z1_signature_bypass`) - plays the fixed mixed-signer sequence
   documented in the PR #2250 regression test (`test_wrong_message`)
   as the reference shape: the `message` payload contains two copies
   of the header + signature + pubkey - one authored by a trusted
   signer with a payload the caller wants attributed to that signer,
   the second (referenced by the ed25519 instruction's offsets) signed
   by a different key over a different payload. Clean: prints
   `regression: clean pass`, rc=0. Planted: prints
   `INVARIANT VIOLATED z1_verify_message_signer_matches_ed25519_program_check ...`,
   rc=1. Stops at the FIRST invariant violation; it detects, it does
   not maximize.
3. **Trident-shape secondary leg** (`trident-tests/z1-signature-integrity/`)
   - a Trident v0.12.0 scaffold wraps the same invariant (firm-shape
   overlap for the Solana-Anchor fuzzer archetype). Scaffold
   schema-checked in CI; runtime execution deferred consistent with
   the atlas's existing Trident deferred-runtime pattern.

## Reproduce locally

Pinned rails (atlas convention; see [`../../rust-toolchain.toml`](../../rust-toolchain.toml)
and [`../../docs/toolchain.md`](../../docs/toolchain.md)):
anchor-lang `= 0.30.1` on the vendored crate, solana-program-test /
solana-sdk `= 1.18.26`, solana-cli 2.1.21, platform-tools v1.52. The
vendored crate's `Cargo.toml` records the anchor-lang pin the upstream
project used at the audit commit.

```sh
# From the atlas root, with the pinned solana-cli on PATH:
export PATH=~/.local/share/solana/install/releases/2.1.21/solana-release/bin:$PATH
bash scripts/check-solana-pin.sh

# Build the clean twin program.
cargo build-sbf \
  --tools-version v1.52 \
  --manifest-path cases/z1-pyth-lazer-signature-integrity/clean/pyth-lazer-solana-contract/Cargo.toml

# Property-based leg on the clean twin.
(cd cases/z1-pyth-lazer-signature-integrity/tests \
 && CASE_TWIN=clean cargo test --release --test z1_invariant_signer_matches -- --nocapture)

# Deterministic regression leg on the clean twin.
(cd cases/z1-pyth-lazer-signature-integrity/tests \
 && CASE_TWIN=clean cargo test --release --test z1_signature_bypass -- --nocapture)

# Build the planted twin program.
cargo build-sbf \
  --tools-version v1.52 \
  --manifest-path cases/z1-pyth-lazer-signature-integrity/planted/pyth-lazer-solana-contract/Cargo.toml

# Re-run both legs against the planted twin. Expect the INVARIANT VIOLATED
# marker and rc=1 on the regression, and marker(s) on the property leg.
(cd cases/z1-pyth-lazer-signature-integrity/tests \
 && CASE_TWIN=planted cargo test --release --test z1_signature_bypass -- --nocapture; \
    test $? -ne 0 || { echo 'regression should have tripped on planted'; exit 1; })
```

## Provenance

The finding this case encodes was reported by Zellic in the [Pyth Lazer
Solana Application Security Assessment,
2025-01-17](https://github.com/Zellic/publications/blob/master/Pyth%20Lazer%20Solana%20-%20Zellic%20Audit%20Report.pdf)
as Finding 3.1 "Signature bypass" (Severity Critical, Impact Critical,
Likelihood Low). The fix was implemented by the Pyth Data Association
in
[`pyth-network/pyth-crosschain#2250`](https://github.com/pyth-network/pyth-crosschain/pull/2250)
"Lazer solana audit fixes," merged as commit
[`eb7f460`](https://github.com/pyth-network/pyth-crosschain/commit/eb7f460ab8d1c73c6c8b4942891c9fe74a589121).
The specific remediation added a byte-for-byte check that the
`message_data` argument matches the bytes referenced by the ed25519
signature-verification instruction's offsets, closing the decoupling
between "the public key `verify_message` treats as the signer" and
"the public key the ed25519 program actually verified." This case is
a defender-side regression fixture: our harness builds the vendored
`pyth-lazer-solana-contract` crate at commit `eb7f460` and
mechanically reverses the fix hunk in the planted twin. It does not
reproduce the finding against any deployed Pyth Lazer program.

**Credit chain:**
- **Finding + audit:** Zellic (report cover: Maik Robert, Avraham
  Weinstock, engagement manager Jacob Goreski, Chad McDonald, Pedro
  Moura).
- **Fix:** Pyth Data Association, PR #2250, merge commit
  [`eb7f460`](https://github.com/pyth-network/pyth-crosschain/commit/eb7f460ab8d1c73c6c8b4942891c9fe74a589121)
  (Apache-2.0).
- **Firm-shape overlap credit (secondary):** Ackee Blockchain, for
  Trident v0.12.0 as the Solana-Anchor stateful-invariant fuzzer
  archetype whose harness shape informed this case's secondary leg.
  The property-based primary leg runs on `solana-program-test` /
  LiteSVM to match the atlas's existing invariant-atlas primitives;
  the Trident wrapper is a scaffold at
  `trident-tests/z1-signature-integrity/` and is schema-checked in
  CI on the same discipline the atlas already uses for the
  `collateral_mint_ref` Trident cell (runtime execution deferred
  consistent with the atlas Trident-cell CI shape).
- **Taxonomy source:** none. Zellic 3.1 is a first-instance finding,
  not part of a named public taxonomy article.

## Honest scope

- The twin is a specification carrier for the signature-verification
  integrity sub-shape. It does not model any specific deployed Pyth
  Lazer program's business-logic surface beyond the invariant under
  test.
- The invariant class is signature-verification integrity for helpers
  that reference sysvar-visible offsets, not a claim about all
  signature-check bugs.
- The property leg finds the class violation reliably inside the
  house per-cell budget (5-minute wall-clock cap; the mixed-signer
  shape trips within a few tens of attempts); no engine or throughput
  claim is implied by the run.

## Paired scorecards

- [`../../docs/scorecards/z1-pyth-lazer-signature-integrity/z1_clean.md`](../../docs/scorecards/z1-pyth-lazer-signature-integrity/z1_clean.md)
- [`../../docs/scorecards/z1-pyth-lazer-signature-integrity/z1_planted.md`](../../docs/scorecards/z1-pyth-lazer-signature-integrity/z1_planted.md)
