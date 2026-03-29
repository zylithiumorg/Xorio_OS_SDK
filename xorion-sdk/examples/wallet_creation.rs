//! Wallet creation example for Xorion Web3 OS SDK

use xorion_sdk::wallet::{Wallet, WalletType};
use xorion_sdk::error::XorionResult;

fn main() -> XorionResult<()> {
    println!("=== Xorion Wallet Creation Example ===\n");

    // Create Ethereum wallet
    println!("Creating Ethereum wallet...");
    let eth_wallet = Wallet::new(WalletType::Ethereum)?;
    println!("Ethereum Address: 0x{}", hex::encode(eth_wallet.address()));
    println!("Wallet Type: {:?}", eth_wallet.wallet_type());

    println!("\n---\n");

    // Create Solana wallet
    println!("Creating Solana wallet...");
    let sol_wallet = Wallet::new(WalletType::Solana)?;
    println!("Solana Public Key: {}", bs58::encode(sol_wallet.address()).into_string());
    println!("Wallet Type: {:?}", sol_wallet.wallet_type());

    println!("\n=== Example Complete ===");
    Ok(())
}
