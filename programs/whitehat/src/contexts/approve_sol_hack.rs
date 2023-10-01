use crate::state::{SolHack, Protocol};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct ApproveSolHack<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        seeds = [b"auth", protocol.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", protocol.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        mut,
        has_one = owner,
        seeds = [b"protocol", owner.key().as_ref()],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(mut)]
    pub payout: SystemAccount<'info>,
    #[account(
        mut,
        has_one = protocol,
        seeds = [b"hack", protocol.key().as_ref(), hack.amount.to_le_bytes().as_ref(), hack.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub hack: Account<'info, SolHack>,
    pub system_program: Program<'info, System>,
}

impl<'info> ApproveSolHack<'info> {
    pub fn approve_sol_hack(&mut self) -> Result<()> {
        let hack = &mut self.hack;

        let protocol = &mut self.protocol;

        // pub payout: Pubkey,
        // pub protocol: Pubkey,
        // pub value: u64,
        // pub reviewed: bool,
        // pub created_at: i64,
        // pub bump: u8,
        // pub seed: u64,

        let amount = protocol.percent * hack.amount / 100;
        protocol.paid += amount;

        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.payout.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.protocol.to_account_info().key.as_ref(),
            &[self.protocol.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx_cpi = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer(ctx_cpi, amount)
    }
}
