use miden_objects::{
    accounts::{AccountHeader, AccountId},
    AccountError, Word,
};

use super::memory::{
    ACCT_CODE_COMMITMENT_OFFSET, ACCT_DATA_MEM_SIZE, ACCT_ID_AND_NONCE_OFFSET, ACCT_ID_IDX,
    ACCT_NONCE_IDX, ACCT_STORAGE_COMMITMENT_OFFSET, ACCT_VAULT_ROOT_OFFSET,
};

// STACK OUTPUTS
// ================================================================================================

/// The index of the word at which the final account nonce is stored on the output stack.
pub const OUTPUT_NOTES_COMMITMENT_WORD_IDX: usize = 0;

/// The index of the word at which the final account hash is stored on the output stack.
pub const FINAL_ACCOUNT_HASH_WORD_IDX: usize = 1;

/// The index of the item at which the expiration block height is stored on the output stack.
pub const EXPIRATION_BLOCK_ELEMENT_IDX: usize = 8;

// ACCOUNT HEADER EXTRACTOR
// ================================================================================================

/// Parses the account header data returned by the VM into individual account component commitments.
/// Returns a tuple of account ID, vault root, storage commitment, code commitment, and nonce.
pub fn parse_final_account_header(elements: &[Word]) -> Result<AccountHeader, AccountError> {
    if elements.len() != ACCT_DATA_MEM_SIZE {
        return Err(AccountError::HeaderDataIncorrectLength(elements.len(), ACCT_DATA_MEM_SIZE));
    }

    let id = AccountId::try_from(elements[ACCT_ID_AND_NONCE_OFFSET as usize][ACCT_ID_IDX])?;
    let nonce = elements[ACCT_ID_AND_NONCE_OFFSET as usize][ACCT_NONCE_IDX];
    let vault_root = elements[ACCT_VAULT_ROOT_OFFSET as usize].into();
    let storage_commitment = elements[ACCT_STORAGE_COMMITMENT_OFFSET as usize].into();
    let code_commitment = elements[ACCT_CODE_COMMITMENT_OFFSET as usize].into();

    Ok(AccountHeader::new(id, nonce, vault_root, storage_commitment, code_commitment))
}
