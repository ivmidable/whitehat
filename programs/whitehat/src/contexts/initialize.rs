use crate::state::Analytics;
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"auth"],
        bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,
    #[account(
        seeds = [b"vault"],
        bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"analytics"],
        bump,
        space = Analytics::LEN
    )]
    pub analytics: Account<'info, Analytics>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &BTreeMap<String, u8>) -> Result<()> {
        let analytics = &mut self.analytics;

        // pub protocols : u64,
        // pub vulnerabilities: u64,
        // pub hacks: u64,
        // pub sol_hacked: u64,
        // pub paid: u64,
        // pub created_at: i64,
        // pub auth_bump: u8,
        // pub vault_bump: u8,
        // pub state_bump: u8,

        analytics.admin = self.admin.key();
        analytics.protocols = 0;
        analytics.vulnerabilities = 0;
        analytics.hacks = 0;
        analytics.sol_recovered = 0;
        analytics.sol_paid = 0;
        analytics.fees = 0;
        analytics.created_at = Clock::get()?.unix_timestamp;
        analytics.auth_bump = *bumps.get("auth").unwrap();
        analytics.vault_bump = *bumps.get("vault").unwrap();
        analytics.state_bump = *bumps.get("analytics").unwrap();

        Ok(())
    }
}
