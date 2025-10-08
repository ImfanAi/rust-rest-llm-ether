use tracing::{info, error, warn};
use std::sync::Arc;

mod api;
mod config;
mod ether;
mod init;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = match config::AppConfig::load() {
        Ok(config) => {
            info!("Configuration loaded successfully");
            config
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            info!("Using default configuration");
            config::AppConfig::default()
        }
    };

    info!("Starting Ethereum wallet server...");

    // Initialize wallet
    let account = match init::init_wallet(&config.wallet.config_file).await {
        Ok(account) => {
            info!("Wallet initialized successfully");
            info!("{}", account); // Uses custom Display implementation (no private key)
            account
        }
        Err(e) => {
            error!("Error initializing wallet: {}", e);
            return Err(e);
        }
    };

    // Establish Web3 connection (optional for some endpoints)
    let web3 = match ether::establish_web3_connection(&config.ethereum.rpc_url).await {
        Ok(connection) => {
            info!("Web3 connection established successfully");
            Some(Arc::new(connection))
        }
        Err(e) => {
            warn!("Failed to establish Web3 connection: {}", e);
            warn!("Some API endpoints will be unavailable");
            None
        }
    };

    // Create application state
    let app_state = api::AppState {
        config: config.clone(),
        account,
        web3,
    };

    // Create router with API endpoints
    let app = api::create_router(app_state);

    let server_addr = config.server_address();
    info!("Server running on http://{}", server_addr);
    info!("Available endpoints:");
    info!("  GET  /              - Health check");
    info!("  GET  /account       - Get account information");
    info!("  GET  /balance       - Get wallet balance");
    info!("  GET  /balance/:addr - Get balance for any address");
    info!("  POST /transaction/send - Send transaction");
    
    axum::Server::bind(&server_addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
