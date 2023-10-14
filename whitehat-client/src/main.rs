use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    instruction::AccountMeta, instruction::Instruction, message::Message, pubkey::Pubkey,
    system_program::ID,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use std::{env, fs, str::FromStr};

const COMMITMENT: CommitmentConfig = CommitmentConfig::confirmed();

const URL: &str = "https://api.devnet.solana.com";

pub fn keypair(file: &str) -> Keypair {
    let bytes = string_u8(&file);
    println!("{:#?}", &bytes.len());
    Keypair::from_bytes(bytes.as_slice()).unwrap()
}

pub fn parse_pubkey(slice: &[u8]) -> [u8; 32] {
    println!("slice length : {:#?}", &slice.len());
    slice.try_into().expect("error parsing pubkey")
}

pub fn string_u8(path: &str) -> Vec<u8> {
    let file = fs::read_to_string(path).expect("Should have been able to read the file");

    let trim = file
        .replace("[", "")
        .replace("]", "")
        .replace(" ", "")
        .replace("\n", "");

    let split: Vec<&str> = trim.split(",").collect();

    let mut result: Vec<u8> = Vec::new();

    for x in split {
        if x.len() > 0 {
            result.push(x.to_owned().parse::<u8>().unwrap())
        }
    }

    println!("result : {:#?}", result.len());

    result
}

pub fn initialize(
    commitment_config: CommitmentConfig,
    wallet_signer: &dyn Signer,
    rpc_client: &RpcClient,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let program_id: Pubkey = Pubkey::from_str("WHATz4jFpiMbaz578KCU8188Ni3AT5ktxqqFLn4CTkd").unwrap();

    #[derive(BorshDeserialize, BorshSerialize, Debug)]
    pub struct Initialize {}
    let (auth, auth_bump) = Pubkey::find_program_address(&[b"auth"], &program_id);
    let (vault, vault_bump) = Pubkey::find_program_address(&[b"vault"], &program_id);
    let (analytics, analytics_bump) = Pubkey::find_program_address(&[b"analytics"], &program_id);

    let data = Initialize {};


    let instruction = Instruction::new_with_borsh(
        program_id,
        &{},
        vec![
            AccountMeta::new_readonly(auth, false),
            AccountMeta::new_readonly(vault, false),
            AccountMeta::new(analytics, false),
            AccountMeta::new_readonly(Pubkey::from(ID), false),
        ],
    );
    let tx = submit_transaction(rpc_client, wallet_signer, instruction, commitment_config);

    match tx {
        Ok(sig) => {
            println!("{:#?}", sig);
            Ok(sig)
        },
        Err(err) => {
            println!("{:#?}", err);
            Err(err)
        }
    }
}

pub fn register_protocol(
    name: String,
    percent: u64,

    commitment_config: CommitmentConfig,
    wallet_signer: &dyn Signer,
    rpc_client: &RpcClient,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let program_id: Pubkey = Pubkey::from(parse_pubkey("WHATz4jFpiMbaz578KCU8188Ni3AT5ktxqqFLn4CTkd".as_bytes()));

    #[derive(BorshDeserialize, BorshSerialize, Debug)]
    pub struct RegisterProtocol {
        pub name: String,
        pub percent: u64,
    }
    let (protocol, protocol_bump) =
        Pubkey::find_program_address(&[b"protocol", wallet_signer.pubkey().as_ref()], &program_id);
    let (auth, auth_bump) =
        Pubkey::find_program_address(&[b"auth", protocol.as_ref()], &program_id);
    let (vault, vault_bump) =
        Pubkey::find_program_address(&[b"vault", protocol.as_ref()], &program_id);
    let (analytics, analytics_bump) = Pubkey::find_program_address(&[b"analytics"], &program_id);

    let data = RegisterProtocol { name, percent };

    let instruction = Instruction::new_with_borsh(
        program_id,
        &{},
        vec![
            AccountMeta::new(protocol, false),
            AccountMeta::new_readonly(auth, false),
            AccountMeta::new_readonly(vault, false),
            AccountMeta::new(analytics, false),
            AccountMeta::new_readonly(Pubkey::from(ID), false),
        ],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)
}

pub fn report_vulnerability(
    payout: &str,
    message: Vec<u8>,
    id: u64,
    seed: u64,
    owner: Pubkey,
    commitment_config: CommitmentConfig,
    wallet_signer: &dyn Signer,
    rpc_client: &RpcClient,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let program_id: Pubkey = Pubkey::from(parse_pubkey("WHATz4jFpiMbaz578KCU8188Ni3AT5ktxqqFLn4CTkd".as_bytes()));

    #[derive(BorshDeserialize, BorshSerialize, Debug)]
    pub struct ReportVulnerability {
        pub message: Vec<u8>,
        pub id: u64,
        pub seed: u64,
    }
    let payout = parse_pubkey(payout.as_bytes());
    let (protocol, protocol_bump) =
        Pubkey::find_program_address(&[b"protocol", owner.as_ref()], &program_id);
    let (vulnerability, vulnerability_bump) = Pubkey::find_program_address(
        &[
            b"vulnerability",
            protocol.as_ref(),
            id.to_le_bytes().as_ref(),
            seed.to_le_bytes().as_ref(),
        ],
        &program_id,
    );

    let data = ReportVulnerability { message, id, seed };

    let instruction = Instruction::new_with_borsh(
        program_id,
        &{},
        vec![
            AccountMeta::new_readonly(Pubkey::from(payout), false),
            AccountMeta::new(protocol, false),
            AccountMeta::new(vulnerability, false),
            AccountMeta::new_readonly(Pubkey::from(ID), false),
        ],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)
}

pub fn approve_vulnerability(
    owner: Pubkey,
    id: u64,
    seed: u64,
    commitment_config: CommitmentConfig,
    wallet_signer: &dyn Signer,
    rpc_client: &RpcClient,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let program_id: Pubkey = Pubkey::from(parse_pubkey("WHATz4jFpiMbaz578KCU8188Ni3AT5ktxqqFLn4CTkd".as_bytes()));
    #[derive(BorshDeserialize, BorshSerialize, Debug)]
    pub struct ApproveVulnerability {}
    let (protocol, protocol_bump) =
        Pubkey::find_program_address(&[b"protocol", owner.as_ref()], &program_id);
    let (vulnerability, vulnerability_bump) = Pubkey::find_program_address(
        &[
            b"vulnerability",
            protocol.as_ref(),
            id.to_le_bytes().as_ref(),
            seed.to_le_bytes().as_ref(),
        ],
        &program_id,
    );
    let (analytics, analytics_bump) = Pubkey::find_program_address(&[b"analytics"], &program_id);

    let data = ApproveVulnerability {};

    let instruction = Instruction::new_with_borsh(
        program_id,
        &{},
        vec![
            AccountMeta::new_readonly(protocol, false),
            AccountMeta::new(vulnerability, false),
            AccountMeta::new(analytics, false),
        ],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)
}

pub fn deposit_sol_hack(
    payout: &str,
    amount: u64,
    owner: Pubkey,
    id: u64,
    seed: u64,
    commitment_config: CommitmentConfig,
    wallet_signer: &dyn Signer,
    rpc_client: &RpcClient,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let program_id: Pubkey = Pubkey::from(parse_pubkey("WHATz4jFpiMbaz578KCU8188Ni3AT5ktxqqFLn4CTkd".as_bytes()));
    #[derive(BorshDeserialize, BorshSerialize, Debug)]
    pub struct DepositSolHack {
        pub amount: u64,
    }
    let payout = parse_pubkey(payout.as_bytes());
    let (protocol, protocol_bump) =
        Pubkey::find_program_address(&[b"protocol", owner.as_ref()], &program_id);
    let (vault, vault_bump) =
        Pubkey::find_program_address(&[b"vault", protocol.as_ref()], &program_id);
    let (vulnerability, vulnerability_bump) = Pubkey::find_program_address(
        &[
            b"vulnerability",
            protocol.as_ref(),
            id.to_le_bytes().as_ref(),
            seed.to_le_bytes().as_ref(),
        ],
        &program_id,
    );
    let (hack, hack_bump) = Pubkey::find_program_address(
        &[b"hack", protocol.as_ref(), amount.to_le_bytes().as_ref()],
        &program_id,
    );

    let data = DepositSolHack { amount };

    let instruction = Instruction::new_with_borsh(
        program_id,
        &{},
        vec![
            AccountMeta::new_readonly(Pubkey::from(payout), false),
            AccountMeta::new(protocol, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(vulnerability, false),
            AccountMeta::new(hack, false),
            AccountMeta::new_readonly(Pubkey::from(ID), false),
        ],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)
}

pub fn approve_sol_hack(
    payout: &str,

    amount: u64,
    commitment_config: CommitmentConfig,
    wallet_signer: &dyn Signer,
    rpc_client: &RpcClient,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let program_id: Pubkey = Pubkey::from(parse_pubkey("WHATz4jFpiMbaz578KCU8188Ni3AT5ktxqqFLn4CTkd".as_bytes()));

    #[derive(BorshDeserialize, BorshSerialize, Debug)]
    pub struct ApproveSolHack {}
    let (protocol, protocol_bump) =
    Pubkey::find_program_address(&[b"protocol", wallet_signer.pubkey().as_ref()], &program_id);
    let (auth, auth_bump) =
        Pubkey::find_program_address(&[b"auth", protocol.as_ref()], &program_id);
    let (vault, vault_bump) =
        Pubkey::find_program_address(&[b"vault", protocol.as_ref()], &program_id);
    let (fees, fees_bump) = Pubkey::find_program_address(&[b"vault"], &program_id);

    let payout = parse_pubkey(payout.as_bytes());
    let (hack, hack_bump) = Pubkey::find_program_address(
        &[b"hack", protocol.as_ref(), amount.to_le_bytes().as_ref()],
        &program_id,
    );
    let (analytics, analytics_bump) = Pubkey::find_program_address(&[b"analytics"], &program_id);

    let data = ApproveSolHack {};

    let instruction = Instruction::new_with_borsh(
        program_id,
        &{},
        vec![
            AccountMeta::new_readonly(auth, false),
            AccountMeta::new(vault, false),
            AccountMeta::new(fees, false),
            AccountMeta::new(protocol, false),
            AccountMeta::new(Pubkey::from(payout), false),
            AccountMeta::new(hack, false),
            AccountMeta::new(analytics, false),
            AccountMeta::new_readonly(Pubkey::from(ID), false),
        ],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)
}

pub fn submit_transaction(
    rpc_client: &RpcClient,
    wallet_signer: &dyn Signer,
    instruction: Instruction,
    commitment_config: CommitmentConfig,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let mut transaction =
        Transaction::new_unsigned(Message::new(&[instruction], Some(&wallet_signer.pubkey())));
    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .map_err(|err| format!("error: unable to get recent blockhash: {}", err))?;
    transaction
        .try_sign(&vec![wallet_signer], recent_blockhash)
        .map_err(|err| format!("error: failed to sign transaction: {}", err))?;
    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, commitment_config)
        .map_err(|err| format!("error: send transaction: {}", err))?;
    Ok(signature)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let rpc_client = RpcClient::new(URL);

    let wallet_signer = keypair(args[2].as_str());

    match args[1].as_str() {
        "initialize" => {
            let sig = initialize(COMMITMENT, &wallet_signer, &rpc_client).unwrap();
        }
        "register_protocol" => {
            let name = args[3].as_str().to_string();
            let percent = args[4].as_str().parse::<u64>().unwrap();
            let sig =
                register_protocol(name, percent, COMMITMENT, &wallet_signer, &rpc_client).unwrap();
        }
        "report_vulnerability" => {
            let payout = args[3].as_str();
            let owner: Pubkey = Pubkey::from(parse_pubkey(args[4].as_str().as_bytes()));

            let message = string_u8(args[5].as_str());
            let id = args[6].as_str().parse::<u64>().unwrap();
            let seed = args[7].as_str().parse::<u64>().unwrap();
            let sig = report_vulnerability(
                payout,
                message,
                id,
                seed,
                owner,
                COMMITMENT,
                &wallet_signer,
                &rpc_client,
            )
            .unwrap();
        }
        "approve_vulnerability" => {
            let owner: Pubkey = Pubkey::from(parse_pubkey(args[3].as_str().as_bytes()));
            let id: u64 = args[4].as_str().parse::<u64>().unwrap();
            let seed: u64 = args[5].as_str().parse::<u64>().unwrap();

            let sig =
                approve_vulnerability(owner, id, seed, COMMITMENT, &wallet_signer, &rpc_client)
                    .unwrap();
        }
        "deposit_sol_hack" => {
            let payout = args[3].as_str();
            let owner: Pubkey = Pubkey::from(parse_pubkey(args[4].as_str().as_bytes()));
            let id: u64 = args[5].as_str().parse::<u64>().unwrap();
            let seed: u64 = args[6].as_str().parse::<u64>().unwrap();

            let amount = args[7].as_str().parse::<u64>().unwrap();
            let sig = deposit_sol_hack(
                payout,
                amount,
                owner,
                id,
                seed,
                COMMITMENT,
                &wallet_signer,
                &rpc_client,
            )
            .unwrap();
        }
        "approve_sol_hack" => {
            let payout = args[3].as_str();
            let amount: u64 = args[4].as_str().parse::<u64>().unwrap();

            let sig =
                approve_sol_hack(payout, amount, COMMITMENT, &wallet_signer, &rpc_client).unwrap();
        }
        _ => println!("something went wrong !"),
    }
}
