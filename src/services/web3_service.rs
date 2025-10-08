use crate::errors::{AppError, AppResult};
use crate::models::{BalanceInfo, NetworkInfo, TransactionInfo, TransactionRequest, TransactionStatus};
use crate::utils;
use secp256k1::SecretKey;
use std::str::FromStr;
use tracing::{info, warn, error};
use web3::{
    transports::WebSocket,
    types::{Address, TransactionParameters, CallRequest},
    Web3,
};

pub struct Web3Service {
    connection: Option<Web3<WebSocket>>,
    network_id: u64,
    rpc_url: String,
}

impl Web3Service {
    pub fn new(rpc_url: String, network_id: u64) -> Self {
        Self {
            connection: None,
            network_id,
            rpc_url,
        }
    }

    /// Establish connection to Ethereum network
    pub async fn connect(&mut self) -> AppResult<()> {
        match web3::transports::WebSocket::new(&self.rpc_url).await {
            Ok(transport) => {
                self.connection = Some(Web3::new(transport));
                info!("Web3 connection established to: {}", self.rpc_url);
                Ok(())
            }
            Err(e) => {
                error!("Failed to connect to Web3: {}", e);
                Err(AppError::Web3ConnectionFailed(e.to_string()))
            }
        }
    }

    /// Check if connection is available
    pub fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    /// Get network information
    pub async fn get_network_info(&self) -> AppResult<NetworkInfo> {
        let web3 = self.connection.as_ref()
            .ok_or(AppError::Web3NotAvailable)?;

        let block_number = match web3.eth().block_number().await {
            Ok(block) => Some(block.as_u64()),
            Err(e) => {
                warn!("Failed to get block number: {}", e);
                None
            }
        };

        let network_name = match self.network_id {
            1 => "Mainnet",
            3 => "Ropsten",
            4 => "Rinkeby",
            5 => "Goerli",
            11155111 => "Sepolia",
            _ => "Unknown",
        };

        Ok(NetworkInfo {
            network_id: self.network_id,
            network_name: network_name.to_string(),
            rpc_url: self.rpc_url.clone(),
            block_number,
        })
    }

    /// Get balance for an address
    pub async fn get_balance(&self, address: &str) -> AppResult<BalanceInfo> {
        let web3 = self.connection.as_ref()
            .ok_or(AppError::Web3NotAvailable)?;

        let addr = Address::from_str(address)
            .map_err(|e| AppError::InvalidAddress(format!("{}: {}", address, e)))?;

        let balance_wei = web3.eth().balance(addr, None).await
            .map_err(|e| AppError::BalanceQueryFailed(e.to_string()))?;

        let balance_eth = utils::wei_to_eth(balance_wei);

        Ok(BalanceInfo {
            address: address.to_string(),
            balance_wei: balance_wei.to_string(),
            balance_eth,
            network_id: self.network_id,
        })
    }

    /// Create transaction parameters
    pub fn create_transaction(&self, to: &str, amount_eth: f64, _gas_price: Option<u64>, _gas_limit: Option<u64>) -> AppResult<TransactionParameters> {
        let to_address = Address::from_str(to)
            .map_err(|e| AppError::InvalidAddress(format!("{}: {}", to, e)))?;

        let tx = TransactionParameters {
            to: Some(to_address),
            value: utils::eth_to_wei(amount_eth),
            ..Default::default()
        };

        // TODO: Add gas customization
        // if let Some(gas_price) = gas_price {
        //     tx.gas_price = Some(U256::from(gas_price));
        // }
        // if let Some(gas_limit) = gas_limit {
        //     tx.gas = Some(U256::from(gas_limit));
        // }

        Ok(tx)
    }

    /// Sign and send transaction
    pub async fn send_transaction(
        &self,
        request: &TransactionRequest,
        secret_key: &SecretKey,
        from_address: &str,
    ) -> AppResult<TransactionInfo> {
        let web3 = self.connection.as_ref()
            .ok_or(AppError::Web3NotAvailable)?;

        let transaction = self.create_transaction(
            &request.to,
            request.amount_eth,
            request.gas_price,
            request.gas_limit,
        )?;

        let signed = web3
            .accounts()
            .sign_transaction(transaction.clone(), secret_key)
            .await
            .map_err(|e| AppError::TransactionFailed(format!("Failed to sign transaction: {}", e)))?;

        let tx_hash = web3
            .eth()
            .send_raw_transaction(signed.raw_transaction)
            .await
            .map_err(|e| AppError::TransactionFailed(format!("Failed to send transaction: {}", e)))?;

        info!("Transaction sent successfully: {:?}", tx_hash);

        Ok(TransactionInfo {
            transaction_hash: format!("{:?}", tx_hash),
            from: from_address.to_string(),
            to: request.to.clone(),
            amount_eth: request.amount_eth,
            gas_price: transaction.gas_price.map(|gp| gp.to_string()),
            gas_limit: None, // TODO: Fix gas limit extraction
            status: TransactionStatus::Pending,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Estimate gas for transaction
    pub async fn estimate_gas(&self, to: &str, amount_eth: f64, from: &str) -> AppResult<u64> {
        let web3 = self.connection.as_ref()
            .ok_or(AppError::Web3NotAvailable)?;

        let to_address = Address::from_str(to)
            .map_err(|e| AppError::InvalidAddress(format!("{}: {}", to, e)))?;
        
        let from_address = Address::from_str(from)
            .map_err(|e| AppError::InvalidAddress(format!("{}: {}", from, e)))?;

        let tx = CallRequest {
            from: Some(from_address),
            to: Some(to_address),
            value: Some(utils::eth_to_wei(amount_eth)),
            ..Default::default()
        };

        let gas_estimate = web3.eth().estimate_gas(tx, None).await
            .map_err(|e| AppError::TransactionFailed(format!("Gas estimation failed: {}", e)))?;

        Ok(gas_estimate.as_u64())
    }

    /// Get current gas price
    pub async fn get_gas_price(&self) -> AppResult<u64> {
        let web3 = self.connection.as_ref()
            .ok_or(AppError::Web3NotAvailable)?;

        let gas_price = web3.eth().gas_price().await
            .map_err(|e| AppError::Web3ConnectionFailed(format!("Failed to get gas price: {}", e)))?;

        Ok(gas_price.as_u64())
    }
}