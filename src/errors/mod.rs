use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    // Wallet related errors
    WalletNotFound,
    WalletCreationFailed(String),
    WalletLoadFailed(String),
    InvalidPrivateKey(String),
    InvalidPublicKey(String),
    
    // Web3 related errors
    Web3ConnectionFailed(String),
    Web3NotAvailable,
    InvalidAddress(String),
    TransactionFailed(String),
    BalanceQueryFailed(String),
    
    // Configuration errors
    ConfigurationError(String),
    
    // General errors
    InternalError(String),
    ValidationError(String),
    NotFound(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: u16,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::WalletNotFound => write!(f, "Wallet not found"),
            AppError::WalletCreationFailed(msg) => write!(f, "Wallet creation failed: {}", msg),
            AppError::WalletLoadFailed(msg) => write!(f, "Failed to load wallet: {}", msg),
            AppError::InvalidPrivateKey(msg) => write!(f, "Invalid private key: {}", msg),
            AppError::InvalidPublicKey(msg) => write!(f, "Invalid public key: {}", msg),
            AppError::Web3ConnectionFailed(msg) => write!(f, "Web3 connection failed: {}", msg),
            AppError::Web3NotAvailable => write!(f, "Web3 connection not available"),
            AppError::InvalidAddress(addr) => write!(f, "Invalid address: {}", addr),
            AppError::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
            AppError::BalanceQueryFailed(msg) => write!(f, "Balance query failed: {}", msg),
            AppError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(resource) => write!(f, "Resource not found: {}", resource),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            AppError::WalletNotFound => (StatusCode::NOT_FOUND, "WALLET_NOT_FOUND", self.to_string()),
            AppError::WalletCreationFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, "WALLET_CREATION_FAILED", self.to_string()),
            AppError::WalletLoadFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, "WALLET_LOAD_FAILED", self.to_string()),
            AppError::InvalidPrivateKey(_) => (StatusCode::BAD_REQUEST, "INVALID_PRIVATE_KEY", self.to_string()),
            AppError::InvalidPublicKey(_) => (StatusCode::BAD_REQUEST, "INVALID_PUBLIC_KEY", self.to_string()),
            AppError::Web3ConnectionFailed(_) => (StatusCode::SERVICE_UNAVAILABLE, "WEB3_CONNECTION_FAILED", self.to_string()),
            AppError::Web3NotAvailable => (StatusCode::SERVICE_UNAVAILABLE, "WEB3_NOT_AVAILABLE", self.to_string()),
            AppError::InvalidAddress(_) => (StatusCode::BAD_REQUEST, "INVALID_ADDRESS", self.to_string()),
            AppError::TransactionFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, "TRANSACTION_FAILED", self.to_string()),
            AppError::BalanceQueryFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, "BALANCE_QUERY_FAILED", self.to_string()),
            AppError::ConfigurationError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "CONFIGURATION_ERROR", self.to_string()),
            AppError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", self.to_string()),
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", self.to_string()),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND", self.to_string()),
        };

        let error_response = ErrorResponse {
            error: error_type.to_string(),
            message,
            code: status.as_u16(),
        };

        (status, Json(error_response)).into_response()
    }
}

// Convert from anyhow::Error
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

// Convert from web3::Error
impl From<web3::Error> for AppError {
    fn from(err: web3::Error) -> Self {
        AppError::Web3ConnectionFailed(err.to_string())
    }
}

// Convert from secp256k1::Error
impl From<secp256k1::Error> for AppError {
    fn from(err: secp256k1::Error) -> Self {
        AppError::InvalidPrivateKey(err.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;