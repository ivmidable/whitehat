use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct SolHack {
    pub payout: Pubkey,
    pub protocol: Pubkey,
    pub amount: u64,
    pub created_at: i64,
    pub bump: u8,
    pub seed: u64,
}

impl SolHack {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + 8
        + PUBLIC_KEY_LENGTH * 2 // payout, protocol
        + 8
        + 1 // reviewed
        + TIMESTAMP_LENGTH // created_at
        + BUMP_LENGTH // bump
        + 8; // seed
}
