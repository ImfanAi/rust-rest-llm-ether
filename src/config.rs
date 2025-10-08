use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub ethereum: EthereumConfig,
    pub wallet: WalletConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthereumConfig {
    pub rpc_url: String,
    pub network_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletConfig {
    pub config_file: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            ethereum: EthereumConfig {
                rpc_url: "wss://mainnet.infura.io/ws/v3/YOUR_API_KEY".to_string(),
                network_id: 1, // Mainnet
            },
            wallet: WalletConfig {
                config_file: "account_config.json".to_string(),
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let settings = config::Config::builder()
            // Start with default values
            .add_source(config::Config::try_from(&AppConfig::default())?)
            // Add environment variables (with prefix APP_)
            .add_source(config::Environment::with_prefix("APP"))
            // Add configuration file if it exists
            .add_source(config::File::with_name("config.toml").required(false));

        // Check for custom config file in environment
        let settings = if let Ok(config_path) = env::var("CONFIG_FILE") {
            settings.add_source(config::File::with_name(&config_path).required(false))
        } else {
            settings
        };

        let config: AppConfig = settings.build()?.try_deserialize()?;
        Ok(config)
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}