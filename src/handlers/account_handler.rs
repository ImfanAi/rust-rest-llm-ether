use crate::errors::AppResult;
use crate::models::{AccountInfo, ApiResponse};
use crate::state::AppState;
use axum::{extract::State, response::Json};

pub async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::success("Ethereum Wallet Server is running"))
}

pub async fn get_account_info(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<AccountInfo>>> {
    let account = state.account.read().await;
    let account_info = account.to_account_info();
    Ok(Json(ApiResponse::success(account_info)))
}

pub async fn get_network_info(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<crate::models::NetworkInfo>>> {
    let web3_service = state.web3_service.read().await;
    let network_info = web3_service.get_network_info().await?;
    Ok(Json(ApiResponse::success(network_info)))
}