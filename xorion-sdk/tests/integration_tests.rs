//! Integration tests for Xorion Web3 OS SDK

use xorion_sdk::wallet::{Wallet, WalletType};
use xorion_sdk::config::{Config, Chain};
use xorion_sdk::client::Client;
use xorion_sdk::error::XorionResult;

#[test]
fn test_wallet_creation_ethereum() -> XorionResult<()> {
    let wallet = Wallet::new(WalletType::Ethereum)?;
    assert_eq!(wallet.address().len(), 20);
    assert_eq!(wallet.wallet_type(), WalletType::Ethereum);
    Ok(())
}

#[test]
fn test_wallet_creation_solana() -> XorionResult<()> {
    let wallet = Wallet::new(WalletType::Solana)?;
    assert_eq!(wallet.address().len(), 32);
    assert_eq!(wallet.wallet_type(), WalletType::Solana);
    Ok(())
}

#[test]
fn test_config_creation() {
    let config = Config::default();
    assert_eq!(config.chain(), Chain::Ethereum);
    
    let sol_config = Config::default().with_chain(Chain::Solana);
    assert_eq!(sol_config.chain(), Chain::Solana);
}

#[tokio::test]
async fn test_client_creation() -> XorionResult<()> {
    let config = Config::default();
    let client = Client::new(&config)?;
    assert_eq!(client.chain(), Chain::Ethereum);
    Ok(())
}

#[tokio::test]
async fn test_solana_client_creation() -> XorionResult<()> {
    let config = Config::default().with_chain(Chain::Solana);
    let client = Client::new(&config)?;
    assert_eq!(client.chain(), Chain::Solana);
    Ok(())
}

// Note: These integration tests would require actual RPC endpoints to fully test
// For now, they verify that the SDK structures can be created and used

#[test]
fn test_error_handling() {
    use xorion_sdk::error::XorionError;
    
    let invalid_address_err = XorionError::InvalidAddress("invalid".to_string());
    assert!(matches!(invalid_address_err, XorionError::InvalidAddress(_)));
    
    let rpc_err = XorionError::RpcError("connection failed".to_string());
    assert!(matches!(rpc_err, XorionError::RpcError(_)));
}
