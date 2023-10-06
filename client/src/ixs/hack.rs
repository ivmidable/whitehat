use solana_program::{
    instruction::AccountMeta, instruction::Instruction, pubkey::Pubkey, system_program::ID,
};
use std::error::Error;

use crate::helpers::parse_pubkey;

const PROGRAM_ID: Pubkey = Pubkey::from(parse_pubkey(
    "SMBKnshAzPi9WZQEUyRxsm6PYZbyZEmxbxbaoMWNHkA".as_bytes(),
));

pub struct Hack {
    pub amount: u64,
}

pub fn hack_ix(
    signer: &str,
    payout: &str,
    protocol: &str,
    owner: &str,
    vulnerability: &str,
    id: u64,
    seed: u64,
    amount: u64,
) -> Result<Instruction, Box<dyn Error>> {
    let signer = parse_pubkey(signer.as_bytes());
    let payout = parse_pubkey(payout.as_bytes());
    let owner = Pubkey::from(parse_pubkey(owner.as_bytes()));
    // seeds = [b"protocol", owner.key().as_ref()]
    let (protocol, protocol_bump) =
        Pubkey::find_program_address(&[b"protocol", owner.as_ref()], &PROGRAM_ID);
    // seeds = [b"vulnerability", protocol.key().as_ref(), vulnerability.id.to_le_bytes().as_ref(), vulnerability.seed.to_le_bytes().as_ref()],
    let (vulnerability, vulnerability_bump) = Pubkey::find_program_address(
        &[
            b"vulnerability",
            protocol.as_ref(),
            id.to_be_bytes().as_ref(),
            seed.to_le_bytes().as_ref(),
        ],
        &PROGRAM_ID,
    );

    // seeds = [b"hack", protocol.key().as_ref(), amount.to_le_bytes().as_ref()],
    let (hack, hack_bump) = Pubkey::find_program_address(
        &[b"hack", protocol.as_ref(), amount.to_le_bytes().as_ref()],
        &PROGRAM_ID,
    );
    // seeds = [b"vault", protocol.key().as_ref()],
    let (vault, vault_bump) = Pubkey::find_program_address(&[b"vault", protocol.as_ref()], &ID);
    let accounts: Vec<AccountMeta> = vec![
        // signer
        // payout
        // protocol
        // vulnerability
        // hack
        // vault
        // systemProgram
        AccountMeta::new(Pubkey::from(signer), true),
        AccountMeta::new_readonly(Pubkey::from(payout), false),
        AccountMeta::new(protocol, false),
        AccountMeta::new_readonly(vulnerability, false),
        AccountMeta::new(hack, false),
        AccountMeta::new(vault, false),
        AccountMeta::new_readonly(Pubkey::from(ID), false),
    ];
    let data = Hack { amount };
    let ix = Instruction::new_with_borsh(PROGRAM_ID, &data, accounts);

    Ok(ix)
}
