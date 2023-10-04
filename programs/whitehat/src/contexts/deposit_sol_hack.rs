use crate::state::{Protocol, SolHack, Vulnerability};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DepositSolHack<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account()]
    pub payout: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"protocol", protocol.owner.as_ref()],
        bump = protocol.state_bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(
        mut,
        seeds = [b"vault", protocol.key().as_ref()],
        bump = protocol.vault_bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        has_one = payout,
        seeds = [b"vulnerability", protocol.key().as_ref(), vulnerability.id.to_le_bytes().as_ref(), vulnerability.seed.to_le_bytes().as_ref()],
        bump = vulnerability.bump,
    )]
    pub vulnerability: Account<'info, Vulnerability>,
    #[account(
        init,
        payer = signer,
        seeds = [b"hack", protocol.key().as_ref(), amount.to_le_bytes().as_ref()],
        bump,
        space = SolHack::LEN
    )]
    pub hack: Account<'info, SolHack>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositSolHack<'info> {
    pub fn deposit_sol_hack(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        amount: u64,
    ) -> Result<()> {
        let hack = &mut self.hack;

        // pub payout: Pubkey,
        // pub protocol: Pubkey,
        // pub value: u64,
        // pub reviewed: bool,
        // pub created_at: i64,
        // pub bump: u8,
        // pub seed: u64,

        hack.payout = self.payout.key();
        hack.protocol = self.protocol.key();
        hack.amount = amount;
        hack.bump = *bumps.get("hack").unwrap();
        hack.created_at = Clock::get()?.unix_timestamp;

        let protocol = &mut self.protocol;

        protocol.hacks += 1;

        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(cpi, amount)
    }
}
