use bitcoin::util::bip32::{ExtendedPrivKey, DerivationPath, ChildNumber};
use bitcoin::util::address::Address;
use bitcoin::network::constants::Network;
use bip39::{Mnemonic, Language, Seed};
use rand::rngs::OsRng;
use std::time::Instant;

fn get_addresses_from_public_key(public_key: &bitcoin::util::key::PublicKey) -> Vec<String> {
    vec![
        // 1. P2PKH
        Address::p2pkh(public_key, Network::Bitcoin).to_string(),

        // 2. P2SH
        Address::p2shwpkh(public_key, Network::Bitcoin).unwrap().to_string(),

        // 3. P2WPKH (SegWit)
        Address::p2wpkh(public_key, Network::Bitcoin).unwrap().to_string(),

        // 4. P2WSH (SegWit)
        Address::p2wsh(public_key, Network::Bitcoin).unwrap().to_string(),
    ]
}