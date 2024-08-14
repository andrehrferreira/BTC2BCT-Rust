use bitcoin::secp256k1::Secp256k1;
use bitcoin::network::constants::Network;
use bitcoin::address::Address;
use bitcoin::bip32::{ExtendedPrivKey, DerivationPath};
use bitcoin::secp256k1::PublicKey;
use rand::rngs::OsRng;
use std::str::FromStr;

#[derive(Debug)]
struct Wallet {
    private_key: String,
    public_key: String,
    addresses: Vec<String>,
}

fn get_addresses_from_public_key(public_key: &PublicKey) -> Vec<String> {
    let compressed_pubkey = public_key.serialize();
    vec![
        // 1. P2PKH
        Address::p2pkh(&compressed_pubkey, Network::Bitcoin).to_string(),

        // 2. P2WPKH (SegWit)
        Address::p2wpkh(&compressed_pubkey, Network::Bitcoin).to_string(),
    ]
}

fn generate_wallets(wallet_count: usize) -> Vec<Wallet> {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &mut rng).unwrap();

    let mut wallets = Vec::new();

    for i in 0..wallet_count {
        let derivation_path = DerivationPath::from_str(&format!("m/44'/0'/0'/0/{}", i)).unwrap();
        let child_key = master_key.derive_priv(&secp, &derivation_path).unwrap();

        let public_key = PublicKey::from_secret_key(&secp, &child_key.private_key);
        let addresses = get_addresses_from_public_key(&public_key);

        wallets.push(Wallet {
            private_key: child_key.private_key.to_wif(),
            public_key: public_key.to_string(),
            addresses,
        });
    }

    wallets
}

fn main() {
    let wallet_count = 5; // Exemplo de quantidade de carteiras a serem geradas
    let wallets = generate_wallets(wallet_count);

    for (i, wallet) in wallets.iter().enumerate() {
        println!("Wallet {}: {:?}", i + 1, wallet);
    }
}
