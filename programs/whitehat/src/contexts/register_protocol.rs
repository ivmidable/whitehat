use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Analytics, Protocol},
};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(name: String, percent: u8)]
pub struct RegisterProtocol<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account()]
    pub encryption: SystemAccount<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"protocol", owner.key().as_ref()],
        bump,
        space = Protocol::LEN
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(
        seeds = [b"auth", protocol.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe
    pub auth: UncheckedAccount<'info>,
    #[account(
        seeds = [b"vault", protocol.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"analytics"],
        bump = analytics.state_bump,
    )]
    pub analytics: Account<'info, Analytics>,
    pub system_program: Program<'info, System>,
}

impl<'info> RegisterProtocol<'info> {
    pub fn register_protocol(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        name: String,
        percent: u8,
    ) -> Result<()> {
        if name.len() > MAX_PROTOCOL_LENGTH {
            return err!(ErrorCode::ProtocolNameTooLong);
        } else if name.len() == 0 {
            return err!(ErrorCode::ProtocolNameEmpty);
        }

        // pub owner: Pubkey,
        // pub sol_vault: Pubkey,
        // pub name: String,
        // pub percent: u8,
        // pub paid : u64,
        // pub vulnerabilities: u64,
        // pub hacks: u64,
        // pub approved: u64,
        // pub created_at: i64,
        // pub bump: u8,

        let protocol = &mut self.protocol;
        protocol.owner = self.owner.key();
        protocol.encryption = self.encryption.key();
        protocol.name = name;
        protocol.percent = percent;
        protocol.paid = 0;
        protocol.vulnerabilities = 0;
        protocol.hacks = 0;
        protocol.auth_bump = *bumps.get("auth").unwrap();
        protocol.vault_bump = *bumps.get("vault").unwrap();
        protocol.state_bump = *bumps.get("protocol").unwrap();
        protocol.created_at = Clock::get()?.unix_timestamp;

        Ok(())
    }
    pub fn update_analytics(&mut self) -> Result<()> {
        let analytics = &mut self.analytics;
        analytics.protocols += 1;
        Ok(())
    }
}
