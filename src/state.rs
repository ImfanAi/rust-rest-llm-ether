use crate::config::AppConfig;
use crate::models::Account;
use crate::services::{WalletService, Web3Service};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub wallet_service: Arc<WalletService>,
    pub web3_service: Arc<RwLock<Web3Service>>,
    pub account: Arc<RwLock<Account>>,
    pub config: AppConfig,
}