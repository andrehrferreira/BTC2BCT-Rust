use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use bitcoin::blockdata;
use bitcoin::network::constants::Network::Bitcoin;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::key;
use bitcoin::Address;
use rand::rngs::OsRng;

fn load_wallets(file_path: &str) -> HashSet<String> {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap()).collect()
}

fn generate_and_check_wallets(
    wallets: Arc<HashSet<String>>
) -> Result<Vec<(key::PrivateKey, Address)>, Box<dyn std::error::Error>> {
    let mut found_wallets = Vec::new();
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let (secret_key, _) = secp.generate_keypair(&mut rng);

    let private = key::PrivateKey {
        compressed: false,
        key: secret_key,
        network: Bitcoin,
    };

    let public = private.public_key(&secp);

    let pub_bytes = &public.to_bytes()[..];
    let uncompressed_p2pk = blockdata::script::Builder::new()
        .push_slice(&pub_bytes)
        .push_opcode(blockdata::opcodes::all::OP_CHECKSIG)
        .into_script();

    // Gerar endereço P2PKH (Pay-to-Public-Key-Hash)
    let p2pkh = Address::p2pkh(&public, Bitcoin);

    // Gerar endereço P2WPKH (Pay-to-Witness-Public-Key-Hash)
    let p2wpkh = Address::p2wpkh(&public, Bitcoin);

    // Criar script para P2WSH (Pay-to-Witness-Script-Hash)
    let p2wsh = Address::p2wsh(&uncompressed_p2pk, Bitcoin);

    // Criar script para P2SH (Pay-to-Script-Hash)
    let p2sh = Address::p2sh(&uncompressed_p2pk, Bitcoin);

    // Verificar se algum endereço corresponde às carteiras pré-carregadas
    for address in &[p2pkh, p2sh, p2wpkh, p2wsh] {
        if wallets.contains(&address.to_string()) {
            found_wallets.push((private.clone(), address.clone()));
        }
    }

    Ok(found_wallets)
}

fn save_found_wallets(file_path: &str, wallets: Vec<(key::PrivateKey, Address)>) {
    let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
    for (private_key, address) in wallets {
        writeln!(file, "Address: {}, Private Key: {}", address, private_key.to_wif()).unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wallets = Arc::new(load_wallets("data/wallets.txt"));

    let mut handles = vec![];
    let wallet_count = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let wallets = Arc::clone(&wallets);
        let wallet_count = Arc::clone(&wallet_count);

        let handle = thread::spawn(move || {
            let mut start_time = Instant::now();

            loop {
                let found_wallets = generate_and_check_wallets(wallets.clone()).unwrap();

                let mut count = wallet_count.lock().unwrap();
                *count += 10;

                let elapsed = start_time.elapsed();
                if elapsed >= Duration::from_secs(60) {
                    let wallets_per_minute = *count as f64 / elapsed.as_secs_f64();

                    if wallets_per_minute > 100 as f64 {
                        println!("Wallets per minute: {:.2}", wallets_per_minute * 60.0);
                    }
                    

                    // Reinicia o contador e o tempo
                    *count = 0;
                    start_time = Instant::now();
                }

                if !found_wallets.is_empty() {
                    save_found_wallets("data/found_wallets.txt", found_wallets);
                    println!("Matching wallet(s) found and saved!");
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
