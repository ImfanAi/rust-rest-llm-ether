use serde::{Deserialize, Serialize};

// Account model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
}

// API Request/Response models
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct AccountInfo {
    pub public_key: String,
    pub address: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize)]
pub struct BalanceInfo {
    pub address: String,
    pub balance_wei: String,
    pub balance_eth: f64,
    pub network_id: u64,
}

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub to: String,
    pub amount_eth: f64,
    pub gas_price: Option<u64>,
    pub gas_limit: Option<u64>,
}

#[derive(Serialize)]
pub struct TransactionInfo {
    pub transaction_hash: String,
    pub from: String,
    pub to: String,
    pub amount_eth: f64,
    pub gas_price: Option<String>,
    pub gas_limit: Option<u64>,
    pub status: TransactionStatus,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Serialize)]
pub struct NetworkInfo {
    pub network_id: u64,
    pub network_name: String,
    pub rpc_url: String,
    pub block_number: Option<u64>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl Account {
    pub fn new(secret_key: &str, public_key: &str, public_address: &str) -> Self {
        Self {
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            public_address: public_address.to_string(),
        }
    }

    pub fn to_account_info(&self) -> AccountInfo {
        AccountInfo {
            public_key: self.public_key.clone(),
            address: self.public_address.clone(),
            created_at: Some(chrono::Utc::now()),
        }
    }
}