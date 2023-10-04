use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;

declare_id!("SMBKnshAzPi9WZQEUyRxsm6PYZbyZEmxbxbaoMWNHkA");

#[program]
pub mod whitehat {
    use super::*;

    // register a protocol, set a name and % paid to hackers
    pub fn register_protocol(
        ctx: Context<RegisterProtocol>,
        name: String,
        percent: u64,
    ) -> Result<()> {
        ctx.accounts.register_protocol(&ctx.bumps, name, percent)
    }

    // vulnerability report, text ecies encrypted off-chain for protocol owner pubkey
    pub fn report_vulnerability(
        ctx: Context<ReportVulnerability>,
        message: Vec<u8>,
        id: u64,
        seed: u64,
    ) -> Result<()> {
        ctx.accounts.report_vulnerability(&ctx.bumps, message, id, seed)
    }

    // turns reviewed from `false` to `true` on vulnerability pda, only protocol owner
    pub fn approve_vulnerability(ctx: Context<ApproveVulnerability>) -> Result<()> {
        ctx.accounts.approve_vulnerability()
    }

    // deposit from signer to protocol vault anonymously, hacker input payout adress through instruction accounts
    pub fn deposit_sol_hack(ctx: Context<DepositSolHack>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_sol_hack(&ctx.bumps, amount)
    }

    // (ONLY PROTOCOL OWNER) pay the hacker to inputed payout address for % set by protocol
    pub fn approve_sol_hack(ctx: Context<ApproveSolHack>) -> Result<()> {
        ctx.accounts.approve_sol_hack()
    }
}
