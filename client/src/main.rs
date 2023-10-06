use ecies_ed25519::{PublicKey, SecretKey};
use std::{env, fs, process};

mod helpers;
mod ixs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = helpers::Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        helpers::help();
        process::exit(1);
    });

    match config.mode.as_str() {
        "cipher" => {
            println!("encryption mode");
            let pubkey =
                PublicKey::from_bytes(&bs58::decode(config.recipient).into_vec().unwrap()).unwrap();

            println!("pubkey : {:#?}", pubkey);

            let mut csprng = rand::thread_rng();

            let encrypted = ecies_ed25519::encrypt(
                &pubkey,
                fs::read_to_string(config.message).unwrap().as_bytes(),
                &mut csprng,
            )
            .unwrap();

            println!("encrypted message length : {}", encrypted.len());
        }
        "decipher" => {
            println!("decryption mode");

            let secret_key =
                SecretKey::from_bytes(&helpers::string_u8(&config.recipient)[0..32]).unwrap();

            println!("secret key : {:#?}", secret_key);

            let encrypted = helpers::string_u8(&config.message);

            println!("encrypted message length : {}", encrypted.len());

            let decrypted = ecies_ed25519::decrypt(&secret_key, &encrypted);

            println!("decrypted message : {:#?}", decrypted);
        }
        _ => helpers::help(),
    }
}
