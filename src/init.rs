use crate::ether;
use crate::utils;
use anyhow::{anyhow, Result};
use tracing::{info, error};

pub async fn init_wallet(path: &str) -> Result<ether::Account> {
    let config_existed: bool = utils::path_exists(path);

    if config_existed {
        info!("Loading existing wallet from: {}", path);
        if let Ok(loaded_account) = ether::Account::from_file(path) {
            info!("Wallet loaded successfully");
            return Ok(loaded_account);
        } else {
            error!("Error loading account from file: {}", path);
            return Err(anyhow!("Error loading account from file"));
        }
    } else {
        info!("Creating new wallet...");
        let (secret_key, pub_key) = ether::generate_keypair();
        // Security: Never log private keys in production
        info!("Generated new keypair");
        info!("Public key: {}", &pub_key.to_string());
        let new_account = ether::Account::new(&secret_key, &pub_key);
        info!("New wallet created with address: {}", &new_account.public_address);

        if let Err(e) = new_account.save_to_file(path) {
            error!("Error saving account information: {}", e);
            return Err(anyhow!("Failed to save account to file: {}", e));
        }

        info!("Wallet saved to: {}", path);
        return Ok(new_account);
    }
}
