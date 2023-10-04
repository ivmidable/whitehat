use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Analytics {
    pub admin: Pubkey,
    pub protocols: u64,
    pub vulnerabilities: u64,
    pub hacks: u64,
    pub sol_recovered: u64,
    pub sol_paid: u64,
    pub fees: u64,
    pub created_at: i64,
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Analytics {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + 8 * 7 // protocols, vulnerabilities, hacks, sol_recovered, sol_paid, created_at
        + BUMP_LENGTH * 3; // bump
}
