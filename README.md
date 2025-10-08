# Rust Ethereum Wallet Server

A secure Rust-based Ethereum wallet server with REST API capabilities.

## Recent Improvements ✨

### 1. Security Fixes 🔒
- **Removed private key logging** - Private keys are no longer printed to console
- **Implemented secure Display trait** - Account struct only shows public information when printed
- **Added proper error handling** - Better error messages without exposing sensitive data

### 2. Configuration Management ⚙️
- **Environment-based configuration** - Use environment variables with `APP_` prefix
- **Configuration file support** - Load settings from `config.toml`
- **Flexible configuration loading** - Environment variables override file settings

### 3. Improved Logging 📝
- **Structured logging with tracing** - Better log formatting and levels
- **Informative startup messages** - Clear indication of wallet and server status
- **Error tracking** - Proper error logging throughout the application

## Configuration

### Using Environment Variables
```bash
export APP_SERVER_HOST=0.0.0.0
export APP_SERVER_PORT=8080
export APP_ETHEREUM_RPC_URL=wss://mainnet.infura.io/ws/v3/YOUR_API_KEY
export APP_WALLET_CONFIG_FILE=my_wallet.json
```

### Using Configuration File
Create a `config.toml` file:
```toml
[server]
host = "0.0.0.0"
port = 3000

[ethereum]
rpc_url = "wss://mainnet.infura.io/ws/v3/YOUR_API_KEY"
network_id = 1

[wallet]
config_file = "account_config.json"
```

### Using Custom Config File
```bash
export CONFIG_FILE=custom_config.toml
```

## Running the Server

```bash
# Build the project
cargo build --release

# Run with default configuration
cargo run

# Run with environment variables
APP_SERVER_PORT=8080 cargo run

# Run with custom config file
CONFIG_FILE=production.toml cargo run
```

## Project Structure

```
src/
├── main.rs          # Application entry point with improved error handling
├── config.rs        # Configuration management (NEW)
├── ether.rs         # Ethereum wallet functionality with security fixes
├── init.rs          # Wallet initialization with proper logging
└── utils.rs         # Utility functions
```

## Security Features

- ✅ Private keys never logged to console
- ✅ Secure account display (public info only)
- ✅ Configuration via environment variables
- ✅ Proper error handling without data leakage

## Next Steps

The following improvements are planned:
1. Extended API endpoints (balance, transactions)
2. Custom error types
3. Database integration
4. Password protection for wallet files
5. HD wallet support
6. Comprehensive testing

## Dependencies

- **axum** - Web framework
- **secp256k1** - Cryptographic operations
- **web3** - Ethereum blockchain interaction
- **config** - Configuration management
- **tracing** - Structured logging
- **tokio** - Async runtime