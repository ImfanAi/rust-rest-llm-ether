use crate::errors::AppResult;
use crate::models::{ApiResponse, BalanceInfo, TransactionInfo, TransactionRequest};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::Json,
};
use tracing::info;

pub async fn get_wallet_balance(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<BalanceInfo>>> {
    let web3_service = state.web3_service.read().await;
    let account = state.account.read().await;
    
    let balance_info = web3_service.get_balance(&account.public_address).await?;
    Ok(Json(ApiResponse::success(balance_info)))
}

pub async fn get_address_balance(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<BalanceInfo>>> {
    let web3_service = state.web3_service.read().await;
    let balance_info = web3_service.get_balance(&address).await?;
    Ok(Json(ApiResponse::success(balance_info)))
}

pub async fn send_transaction(
    State(state): State<AppState>,
    Json(request): Json<TransactionRequest>,
) -> AppResult<Json<ApiResponse<TransactionInfo>>> {
    let web3_service = state.web3_service.read().await;
    let account = state.account.read().await;
    
    // Get secret key for signing
    let secret_key = state.wallet_service.get_secret_key(&account)?;
    
    // Send transaction
    let transaction_info = web3_service
        .send_transaction(&request, &secret_key, &account.public_address)
        .await?;
    
    info!("Transaction sent: {}", transaction_info.transaction_hash);
    Ok(Json(ApiResponse::success(transaction_info)))
}

pub async fn estimate_gas(
    Path((to, amount)): Path<(String, String)>,
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<u64>>> {
    let web3_service = state.web3_service.read().await;
    let account = state.account.read().await;
    
    let amount_eth: f64 = amount.parse()
        .map_err(|_| crate::errors::AppError::ValidationError("Invalid amount format".to_string()))?;
    
    let gas_estimate = web3_service
        .estimate_gas(&to, amount_eth, &account.public_address)
        .await?;
    
    Ok(Json(ApiResponse::success(gas_estimate)))
}

pub async fn get_gas_price(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<u64>>> {
    let web3_service = state.web3_service.read().await;
    let gas_price = web3_service.get_gas_price().await?;
    Ok(Json(ApiResponse::success(gas_price)))
}