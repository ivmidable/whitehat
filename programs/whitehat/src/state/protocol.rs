use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Protocol {
    pub owner: Pubkey,
    pub encryption: Pubkey,
    pub vault: Pubkey,
    pub name: String,
    pub percent: u8,
    pub paid: u64,
    pub vulnerabilities: u64,
    pub hacks: u64,
    pub created_at: i64,
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Protocol {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 3 // owner, encryption, vault
        + STRING_LENGTH_PREFIX
        + MAX_PROTOCOL_LENGTH
        + 1 // percent
        + 8 * 3 // paid, vulnerabilities, hacks
        + TIMESTAMP_LENGTH // created_at
        + BUMP_LENGTH * 3; // bump
}
