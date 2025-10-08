# Rust Ethereum Wallet Server

A professional, production-ready Rust-based Ethereum wallet server with REST API capabilities, built using modern OOP architecture and industry best practices.

## 🏗️ Architecture Overview

This project follows a **clean, object-oriented architecture** with proper separation of concerns:

- **Service Layer Pattern** for business logic
- **Handler-Based Routing** for HTTP endpoints
- **Unified State Management** with dependency injection
- **Custom Error Handling** with HTTP response mapping
- **Configuration Management** with environment support

## 📁 Project Structure

```
src/
├── main.rs                    # Application entry point & server setup
├── config.rs                  # Configuration management system
├── state.rs                   # Unified application state container
├── utils.rs                   # Utility functions (conversions, etc.)
│
├── errors/                    # Error handling system
│   └── mod.rs                 # Custom AppError enum with HTTP mapping
│
├── models/                    # Data models & DTOs
│   └── mod.rs                 # Account, API responses, transaction models
│
├── services/                  # Business logic layer (OOP)
│   ├── mod.rs                 # Service exports
│   ├── wallet_service.rs      # Wallet operations & cryptography
│   └── web3_service.rs        # Blockchain interactions
│
└── handlers/                  # HTTP request handlers
    ├── mod.rs                 # Handler exports
    ├── account_handler.rs     # Account & network endpoints
    └── wallet_handler.rs      # Wallet & transaction endpoints
```

## 🎯 Key Features

### ✅ Security Features
- **No private key exposure** in logs or responses
- **Secure account validation** with cryptographic verification
- **Type-safe error handling** without data leakage
- **Professional logging** with structured output

### ✅ OOP Design Patterns
- **WalletService**: Encapsulates wallet operations
  - Key generation and validation
  - Account creation and persistence
  - Cryptographic operations
- **Web3Service**: Handles blockchain interactions
  - Connection management
  - Balance queries and transactions
  - Gas estimation and network operations

### ✅ Configuration Management
- **Environment variable support** (`APP_` prefix)
- **Configuration file loading** (`config.toml`)
- **Graceful fallbacks** to default settings
- **Runtime configuration validation**

### ✅ Professional API Design
- **RESTful endpoints** with proper HTTP methods
- **Structured JSON responses** with timestamps
- **Comprehensive error handling** with appropriate status codes
- **Type-safe request/response models**

## 🚀 API Endpoints

### Health & Information
```
GET  /              - Health check
GET  /health        - System health status
GET  /network       - Blockchain network information
```

### Account Management
```
GET  /account       - Get wallet account information
```

### Balance Operations
```
GET  /balance       - Get wallet balance (Wei + ETH)
GET  /balance/:addr - Get balance for any Ethereum address
```

### Transaction Operations
```
POST /transaction/send - Send Ethereum transaction
GET  /gas-price     - Get current network gas price
GET  /estimate-gas/:to/:amount - Estimate gas for transaction
```

## ⚙️ Configuration

### Using Environment Variables
```bash
export APP_SERVER_HOST=0.0.0.0
export APP_SERVER_PORT=8080
export APP_ETHEREUM_RPC_URL=wss://mainnet.infura.io/ws/v3/YOUR_API_KEY
export APP_ETHEREUM_NETWORK_ID=1
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
network_id = 1  # 1=Mainnet, 5=Goerli, 11155111=Sepolia

[wallet]
config_file = "account_config.json"
```

### Using Custom Config File
```bash
export CONFIG_FILE=production.toml
```

## 🛠️ Installation & Usage

### Prerequisites
- Rust 1.70+ with Cargo
- Access to Ethereum RPC endpoint (Infura, Alchemy, etc.)

### Build & Run
```bash
# Clone the repository
git clone https://github.com/ImfanAi/rust-rest-llm-ether.git
cd rust-rest-llm-ether

# Build the project
cargo build --release

# Run with default configuration
cargo run

# Run with environment variables
APP_SERVER_PORT=8080 cargo run

# Run with custom config file
CONFIG_FILE=production.toml cargo run
```

## 🧪 Example Usage

### Check Server Health
```bash
curl http://localhost:3000/health
```

### Get Account Information
```bash
curl http://localhost:3000/account
```

### Check Wallet Balance
```bash
curl http://localhost:3000/balance
```

### Send Transaction
```bash
curl -X POST http://localhost:3000/transaction/send \
  -H "Content-Type: application/json" \
  -d '{
    "to": "0x742c4C8d4c0d0F8e8C8C8C8C8C8C8C8C8C8C8C8C",
    "amount_eth": 0.001,
    "gas_price": 20000000000,
    "gas_limit": 21000
  }'
```

## 🏆 Technical Improvements

### From Legacy to Professional
- ❌ **Removed**: Monolithic `ether.rs`, `api.rs`, `init.rs`
- ✅ **Added**: Modular service layer with clear responsibilities
- ✅ **Enhanced**: Type-safe error handling with HTTP mapping
- ✅ **Improved**: Configuration management with environment support
- ✅ **Secured**: Eliminated private key logging and exposure

### Code Quality
- **SOLID Principles**: Single responsibility, dependency injection
- **Clean Architecture**: Proper layer separation
- **Type Safety**: Comprehensive error handling
- **Documentation**: Clear structure and usage examples
- **Testing Ready**: Modular design for easy unit testing

## 🔧 Dependencies

### Core Framework
- **axum**: Modern web framework for Rust
- **tokio**: Async runtime
- **serde**: Serialization/deserialization

### Blockchain & Crypto
- **web3**: Ethereum client library
- **secp256k1**: Cryptographic operations
- **tiny-keccak**: Keccak hashing

### Configuration & Logging
- **config**: Configuration management
- **tracing**: Structured logging
- **chrono**: Date/time handling

## 🚀 Future Enhancements

### Planned Features
- [ ] **Database Integration** (PostgreSQL/SQLite)
- [ ] **Comprehensive Testing** (Unit + Integration)
- [ ] **API Documentation** (OpenAPI/Swagger)
- [ ] **Authentication System** (JWT/API Keys)
- [ ] **Rate Limiting** & request validation
- [ ] **Monitoring & Metrics** (Prometheus)
- [ ] **Docker Support** with multi-stage builds
- [ ] **HD Wallet Support** (BIP32/39/44)

### Production Ready Features
- [x] Professional OOP architecture
- [x] Secure key management
- [x] Environment-based configuration
- [x] Structured logging
- [x] Error handling with HTTP mapping
- [x] Type-safe API design

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## ⚠️ Security Notice

- Never expose private keys in logs or API responses
- Use environment variables for sensitive configuration
- Always validate input data before processing
- Keep RPC endpoints and API keys secure
- Use HTTPS in production environments