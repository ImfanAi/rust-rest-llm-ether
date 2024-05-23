use crate::ether;
use crate::utils;
use anyhow::{anyhow, Result};

pub async fn init_wallet(path: &str) -> Result<ether::Account> {
    let config_existed: bool = utils::path_exists(path);

    if config_existed {
        if let Ok(loaded_account) = ether::Account::from_file(path) {
            println!("loaded_wallet: {:?}", loaded_account);
            return Ok(loaded_account);
        } else {
            // Handle error loading account from file
            return Err(anyhow!("Error loading account from file"));
        }
    } else {
        let (secret_key, pub_key) = ether::generate_keypair();
        println!("secret key: {}", &secret_key.to_string());
        println!("public key: {}", &pub_key.to_string());
        let new_account = ether::Account::new(&secret_key, &pub_key);
        println!("crypto_wallet: {:?}", &new_account);

        if let Err(e) = new_account.save_to_file(path) {
            eprintln!("Error saving account information: {}", e);
            // Handle error saving account to file
        }

        return Ok(new_account);
    }
}
