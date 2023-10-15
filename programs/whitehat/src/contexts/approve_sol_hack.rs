use crate::state::{Analytics, Protocol, SolHack};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct ApproveSolHack<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        seeds = [b"auth", protocol.key().as_ref()],
        bump = protocol.auth_bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", protocol.key().as_ref()],
        bump = protocol.vault_bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault"],
        bump = analytics.vault_bump
    )]
    fees: SystemAccount<'info>,
    #[account(
        mut,
        has_one = owner,
        seeds = [b"protocol", owner.key().as_ref()],
        bump = protocol.state_bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(mut)]
    pub payout: SystemAccount<'info>,
    #[account(
        mut,
        has_one = protocol,
        has_one = payout,
        close = owner,
        seeds = [b"hack", protocol.key().as_ref(), hack.amount.to_le_bytes().as_ref()],
        bump = hack.bump,
    )]
    pub hack: Account<'info, SolHack>,
    #[account(
        mut,
        seeds = [b"analytics"],
        bump = analytics.state_bump,
    )]
    pub analytics: Account<'info, Analytics>,
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

        // hack amount = 100 %
        // due amount = protocol %
        // due amount = protocol % * amount / 100

        let fee = hack.amount / 100;

        let amount =
            u64::try_from((protocol.percent as u128 * hack.amount as u128 / 100u128) - fee as u128)
                .unwrap();
        protocol.paid += amount;

        self.send_from_vault_to_hacker(amount)?;
        self.send_fee_to_fee_accounts(fee)
    }

    pub fn send_fee_to_fee_accounts(&mut self, fee: u64) -> Result<()> {
        let seeds = &[
            b"vault",
            self.protocol.to_account_info().key.as_ref(),
            &[self.protocol.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let fee_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.fees.to_account_info(),
        };

        let fee_cpi = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            fee_accounts,
            signer_seeds,
        );
        transfer(fee_cpi, fee)
    }

    pub fn send_from_vault_to_hacker(&mut self, amount: u64) -> Result<()> {
        let seeds = &[
            b"vault",
            self.protocol.to_account_info().key.as_ref(),
            &[self.protocol.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let hacker_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.payout.to_account_info(),
        };

        let hacker_cpi = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            hacker_accounts,
            signer_seeds,
        );

        transfer(hacker_cpi, amount)
    }

    pub fn update_analytics(&mut self) -> Result<()> {
        let analytics = &mut self.analytics;
        let hack = &mut self.hack;
        let protocol = &mut self.protocol;

        let amount = u64::try_from(protocol.percent as u128 * hack.amount as u128 / 100).unwrap();

        analytics.hacks += 1;
        analytics.sol_recovered += hack.amount;
        analytics.sol_paid += amount;
        analytics.fees += hack.amount / 100;
        Ok(())
    }
}
