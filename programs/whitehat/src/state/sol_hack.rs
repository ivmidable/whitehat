use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct SolHack {
    pub protocol: Pubkey,
    pub payout: Pubkey,
    pub reviewed: bool,
    pub amount: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl SolHack {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + 8
        + PUBLIC_KEY_LENGTH * 2 // payout, protocol
        + 8
        + 1 // reviewed
        + TIMESTAMP_LENGTH // created_at
        + BUMP_LENGTH; // bump
}
