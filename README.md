# Xorion Web3 OS SDK

A comprehensive Rust SDK for interacting with Ethereum and Solana blockchains.

## Features

- **Multi-Chain Support**: Ethereum and Solana integration
- **Wallet Management**: Create and manage wallets for both chains
- **Smart Contracts**: Deploy and interact with smart contracts
- **Token Operations**: ERC20, ERC721 token transfers
- **DeFi Integration**: Swap, liquidity, and DEX interactions
- **RPC Clients**: High-performance RPC clients for both chains
- **Transaction Signing**: Secure transaction signing with multiple signer types

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
xorion-sdk = { git = "https://github.com/YOUR_USERNAME/xorion-web3-os" }
```

## Quick Start

```rust
use xorion_sdk::wallet::Wallet;
use xorion_sdk::ethereum::wallet::EthereumWallet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an Ethereum wallet
    let wallet = EthereumWallet::new_random()?;
    println!("Address: {}", wallet.address());
    
    Ok(())
}
```

## Examples

Check out the [examples](xorion-sdk/examples/) directory for more usage examples:

- `wallet_creation.rs` - Creating wallets
- `rpc_integration.rs` - RPC client usage
- `smart_contracts.rs` - Smart contract interactions
- `token_transfers.rs` - Token operations
- `defi_interactions.rs` - DeFi protocol interactions

## Structure

```
xorion-sdk/
├── src/
│   ├── lib.rs           # Main library entry point
│   ├── error.rs         # Error types
│   ├── config.rs        # Configuration
│   ├── wallet.rs        # Wallet traits
│   ├── client.rs        # RPC clients
│   ├── transaction.rs   # Transaction handling
│   ├── balance.rs       # Balance queries
│   ├── ethereum/        # Ethereum-specific modules
│   ├── solana/          # Solana-specific modules
│   ├── contract/        # Smart contract modules
│   ├── tokens/          # Token modules
│   ├── defi/            # DeFi modules
│   └── signing/         # Signing modules
├── examples/            # Usage examples
└── tests/               # Integration and unit tests
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
