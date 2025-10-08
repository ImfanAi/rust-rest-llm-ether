use crate::{config::AppConfig, ether};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};
use web3::{
    types::Address,
    Web3,
};

// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub account: ether::Account,
    pub web3: Option<Arc<Web3<web3::transports::WebSocket>>>,
}

// API Response types
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct AccountInfo {
    pub public_key: String,
    pub address: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub address: String,
    pub balance_wei: String,
    pub balance_eth: f64,
}

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub to: String,
    pub amount_eth: f64,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub transaction_hash: String,
    pub from: String,
    pub to: String,
    pub amount_eth: f64,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

// API Routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/account", get(get_account_info))
        .route("/balance", get(get_balance))
        .route("/balance/:address", get(get_balance_for_address))
        .route("/transaction/send", post(send_transaction))
        .with_state(state)
}

// Health check endpoint
async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::success("Ethereum Wallet Server is running"))
}

// Get account information
async fn get_account_info(
    State(state): State<AppState>,
) -> Json<ApiResponse<AccountInfo>> {
    let account_info = AccountInfo {
        public_key: state.account.public_key.clone(),
        address: state.account.public_address.clone(),
    };
    
    Json(ApiResponse::success(account_info))
}

// Get wallet balance
async fn get_balance(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<BalanceResponse>>, StatusCode> {
    if let Some(web3) = &state.web3 {
        match state.account.get_balance(web3).await {
            Ok(balance_wei) => {
                match state.account.get_balance_in_eth(web3).await {
                    Ok(balance_eth) => {
                        let response = BalanceResponse {
                            address: state.account.public_address.clone(),
                            balance_wei: balance_wei.to_string(),
                            balance_eth,
                        };
                        Ok(Json(ApiResponse::success(response)))
                    }
                    Err(e) => {
                        error!("Failed to convert balance to ETH: {}", e);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            }
            Err(e) => {
                error!("Failed to get balance: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        error!("Web3 connection not available");
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

// Get balance for any address
async fn get_balance_for_address(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<BalanceResponse>>, StatusCode> {
    if let Some(web3) = &state.web3 {
        match address.parse::<Address>() {
            Ok(addr) => {
                match web3.eth().balance(addr, None).await {
                    Ok(balance_wei) => {
                        let balance_eth = crate::utils::wei_to_eth(balance_wei);
                        let response = BalanceResponse {
                            address: address,
                            balance_wei: balance_wei.to_string(),
                            balance_eth,
                        };
                        Ok(Json(ApiResponse::success(response)))
                    }
                    Err(e) => {
                        error!("Failed to get balance for address {}: {}", address, e);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            }
            Err(e) => {
                error!("Invalid address format {}: {}", address, e);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    } else {
        error!("Web3 connection not available");
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

// Send transaction
async fn send_transaction(
    State(state): State<AppState>,
    Json(req): Json<TransactionRequest>,
) -> Result<Json<ApiResponse<TransactionResponse>>, StatusCode> {
    if let Some(web3) = &state.web3 {
        // Parse destination address
        let to_address = match req.to.parse::<Address>() {
            Ok(addr) => addr,
            Err(e) => {
                error!("Invalid destination address {}: {}", req.to, e);
                return Err(StatusCode::BAD_REQUEST);
            }
        };

        // Get secret key
        let secret_key = match state.account.get_secret_key() {
            Ok(key) => key,
            Err(e) => {
                error!("Failed to get secret key: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        // Create transaction
        let transaction = ether::create_eth_transaction(to_address, req.amount_eth);

        // Sign and send transaction
        match ether::sign_and_send(web3, transaction, &secret_key).await {
            Ok(tx_hash) => {
                info!("Transaction sent successfully: {:?}", tx_hash);
                let response = TransactionResponse {
                    transaction_hash: format!("{:?}", tx_hash),
                    from: state.account.public_address.clone(),
                    to: req.to,
                    amount_eth: req.amount_eth,
                };
                Ok(Json(ApiResponse::success(response)))
            }
            Err(e) => {
                error!("Failed to send transaction: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        error!("Web3 connection not available");
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}