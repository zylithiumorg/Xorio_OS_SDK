//! Xorion Web3 OS SDK
//! 
//! A comprehensive Rust SDK for interacting with Ethereum and Solana blockchains.
//! 
//! # Features
//! 
//! - **Multi-Chain Support**: Ethereum and Solana integration
//! - **Wallet Management**: Create and manage wallets for both chains
//! - **Smart Contracts**: Deploy and interact with smart contracts
//! - **Token Operations**: ERC20, ERC721 token transfers
//! - **DeFi Integration**: Swap, liquidity, and DEX interactions
//! - **RPC Clients**: High-performance RPC clients for both chains
//! 
//! # Example
//! 
//! ```rust,no_run
//! use xorion_sdk::ethereum::wallet::EthereumWallet;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let wallet = EthereumWallet::new_random()?;
//!     println!("Address: {}", wallet.address());
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod config;
pub mod wallet;
pub mod client;
pub mod transaction;
pub mod balance;

pub mod ethereum;
pub mod solana;
pub mod contract;
pub mod tokens;
pub mod defi;
pub mod signing;

pub use error::{XorionError, Result};
pub use config::Config;
pub use wallet::WalletTrait;
pub use client::RpcClient;
pub use transaction::Transaction;
pub use balance::Balance;

/// SDK version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the SDK with logging
pub fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_init_logging() {
        // This should not panic
        let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
            .try_init();
    }
}
