use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

// Module declarations
mod config;
mod errors;
mod handlers;
mod models;
mod services;
mod state;
mod utils;

use config::AppConfig;
use errors::AppResult;
use models::Account;
use services::{WalletService, Web3Service};
use state::AppState;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting Ethereum Wallet Server...");

    // Load configuration
    let config = load_configuration().await?;

    // Initialize services
    let wallet_service = Arc::new(WalletService::new());
    let mut web3_service = Web3Service::new(
        config.ethereum.rpc_url.clone(),
        config.ethereum.network_id,
    );

    // Initialize wallet
    let account = initialize_wallet(&wallet_service, &config).await?;
    let account = Arc::new(RwLock::new(account));

    // Establish Web3 connection
    if let Err(e) = web3_service.connect().await {
        warn!("Failed to establish Web3 connection: {}", e);
        warn!("Some API endpoints will be unavailable");
    }
    let web3_service = Arc::new(RwLock::new(web3_service));

    // Create and start server
    let app = create_router(wallet_service, web3_service, account, config.clone()).await;
    start_server(app, &config).await?;

    Ok(())
}

async fn load_configuration() -> AppResult<AppConfig> {
    match AppConfig::load() {
        Ok(config) => {
            info!("Configuration loaded successfully");
            Ok(config)
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            info!("Using default configuration");
            Ok(AppConfig::default())
        }
    }
}

async fn initialize_wallet(
    wallet_service: &WalletService,
    config: &AppConfig,
) -> AppResult<Account> {
    let account = wallet_service.initialize_wallet(&config.wallet.config_file)?;
    
    // Validate account integrity
    if let Err(e) = wallet_service.validate_account(&account) {
        error!("Account validation failed: {}", e);
        return Err(e);
    }

    info!("Wallet initialized successfully");
    info!("Account address: {}", account.public_address);
    Ok(account)
}

async fn create_router(
    wallet_service: Arc<WalletService>,
    web3_service: Arc<RwLock<Web3Service>>,
    account: Arc<RwLock<Account>>,
    config: AppConfig,
) -> Router {
    let app_state = AppState {
        wallet_service,
        web3_service,
        account,
        config,
    };

    Router::new()
        // Health and info endpoints
        .route("/", get(handlers::account_handler::health_check))
        .route("/health", get(handlers::account_handler::health_check))
        .route("/network", get(handlers::account_handler::get_network_info))
        
        // Account endpoints
        .route("/account", get(handlers::account_handler::get_account_info))
        
        // Wallet endpoints
        .route("/balance", get(handlers::wallet_handler::get_wallet_balance))
        .route("/balance/:address", get(handlers::wallet_handler::get_address_balance))
        .route("/gas-price", get(handlers::wallet_handler::get_gas_price))
        .route("/estimate-gas/:to/:amount", get(handlers::wallet_handler::estimate_gas))
        
        // Transaction endpoints
        .route("/transaction/send", post(handlers::wallet_handler::send_transaction))
        
        // Shared state
        .with_state(app_state)
}

async fn start_server(app: Router, config: &AppConfig) -> AppResult<()> {
    let server_addr = config.server_address();
    
    info!("Server starting on http://{}", server_addr);
    info!("Available endpoints:");
    info!("  GET  /              - Health check");
    info!("  GET  /health        - Health check");
    info!("  GET  /network       - Network information");
    info!("  GET  /account       - Account information");
    info!("  GET  /balance       - Wallet balance");
    info!("  GET  /balance/:addr - Balance for any address");
    info!("  GET  /gas-price     - Current gas price");
    info!("  GET  /estimate-gas/:to/:amount - Estimate gas for transaction");
    info!("  POST /transaction/send - Send transaction");

    axum::Server::bind(&server_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .map_err(|e| errors::AppError::InternalError(format!("Server error: {}", e)))?;

    Ok(())
}
