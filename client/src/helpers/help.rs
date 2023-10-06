pub fn help() {
    println!(
        "
    whitehat client :

    usage : 
    
    ENCRYPT/DECRYPT

    - encrypt text for a public key (ed25519)

    wh -c <TXT_FILE> <BASE58_PUBKEY>

    example : 

    $    wh -c encrypt.txt ECa2gUofM8ZBJv7rRP1h6PgoNT8HyuC7nUkbnPchJsxu

    - decrypt text meant for your public key, with your private key (ed25519)

    wh -d <TXT_FILE> <KEYPAIR>

    example : 

    $    wh -d decrypt.txt keypair.json

    "
    );
}
