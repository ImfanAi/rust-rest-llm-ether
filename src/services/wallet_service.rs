use crate::errors::{AppError, AppResult};
use crate::models::Account;
use crate::utils;
use secp256k1::{rand::rngs, PublicKey, SecretKey, Secp256k1};
use serde_json;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::str::FromStr;
use tiny_keccak::keccak256;
use tracing::info;
use web3::types::Address;

pub struct WalletService {
    secp: Secp256k1<secp256k1::All>,
}

impl WalletService {
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
        }
    }

    /// Generate a new cryptographic keypair
    pub fn generate_keypair(&self) -> AppResult<(SecretKey, PublicKey)> {
        let mut rng = rngs::JitterRng::new_with_timer(utils::get_nstime);
        let keypair = self.secp.generate_keypair(&mut rng);
        Ok(keypair)
    }

    /// Convert public key to Ethereum address
    pub fn public_key_to_address(&self, public_key: &PublicKey) -> Address {
        let public_key_bytes = public_key.serialize_uncompressed();
        debug_assert_eq!(public_key_bytes[0], 0x04);
        let hash = keccak256(&public_key_bytes[1..]);
        Address::from_slice(&hash[12..])
    }

    /// Create a new account
    pub fn create_account(&self) -> AppResult<Account> {
        let (secret_key, public_key) = self.generate_keypair()?;
        let address = self.public_key_to_address(&public_key);
        
        let account = Account::new(
            &secret_key.to_string(),
            &public_key.to_string(),
            &format!("{:?}", address),
        );

        info!("New account created with address: {}", account.public_address);
        Ok(account)
    }

    /// Save account to file
    pub fn save_account(&self, account: &Account, file_path: &str) -> AppResult<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .map_err(|e| AppError::WalletCreationFailed(format!("Failed to create file: {}", e)))?;

        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, account)
            .map_err(|e| AppError::WalletCreationFailed(format!("Failed to serialize account: {}", e)))?;

        info!("Account saved to: {}", file_path);
        Ok(())
    }

    /// Load account from file
    pub fn load_account(&self, file_path: &str) -> AppResult<Account> {
        if !utils::path_exists(file_path) {
            return Err(AppError::WalletNotFound);
        }

        let file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .map_err(|e| AppError::WalletLoadFailed(format!("Failed to open file: {}", e)))?;

        let buf_reader = BufReader::new(file);
        let account: Account = serde_json::from_reader(buf_reader)
            .map_err(|e| AppError::WalletLoadFailed(format!("Failed to deserialize account: {}", e)))?;

        info!("Account loaded from: {}", file_path);
        Ok(account)
    }

    /// Initialize wallet - load existing or create new
    pub fn initialize_wallet(&self, file_path: &str) -> AppResult<Account> {
        if utils::path_exists(file_path) {
            info!("Loading existing wallet from: {}", file_path);
            self.load_account(file_path)
        } else {
            info!("Creating new wallet...");
            let account = self.create_account()?;
            self.save_account(&account, file_path)?;
            Ok(account)
        }
    }

    /// Get secret key from account
    pub fn get_secret_key(&self, account: &Account) -> AppResult<SecretKey> {
        SecretKey::from_str(&account.secret_key)
            .map_err(|e| AppError::InvalidPrivateKey(e.to_string()))
    }

    /// Get public key from account
    pub fn get_public_key(&self, account: &Account) -> AppResult<PublicKey> {
        PublicKey::from_str(&account.public_key)
            .map_err(|e| AppError::InvalidPublicKey(e.to_string()))
    }

    /// Validate account integrity
    pub fn validate_account(&self, account: &Account) -> AppResult<bool> {
        let secret_key = self.get_secret_key(account)?;
        let public_key = self.get_public_key(account)?;
        
        // Verify that the public key matches the secret key
        let derived_public_key = PublicKey::from_secret_key(&self.secp, &secret_key);
        if derived_public_key != public_key {
            return Err(AppError::ValidationError("Public key doesn't match secret key".to_string()));
        }

        // Verify that the address matches the public key
        let derived_address = self.public_key_to_address(&public_key);
        let expected_address = format!("{:?}", derived_address);
        if expected_address != account.public_address {
            return Err(AppError::ValidationError("Address doesn't match public key".to_string()));
        }

        Ok(true)
    }
}

impl Default for WalletService {
    fn default() -> Self {
        Self::new()
    }
}