//! Token transfers example for Xorion Web3 OS SDK

use xorion_sdk::tokens::{ERC20, ERC721};
use xorion_sdk::client::Client;
use xorion_sdk::config::Config;
use xorion_sdk::error::XorionResult;
use alloy_primitives::{Address, U256};

#[tokio::main]
async fn main() -> XorionResult<()> {
    println!("=== Xorion Token Transfers Example ===\n");

    // Create client
    let config = Config::default();
    let client = Client::new(&config)?;

    // ===== ERC20 Token Example =====
    println!("--- ERC20 Token Operations ---");
    
    let token_address: Address = "0x1234567890123456789012345678901234567890".parse()?;
    let erc20 = ERC20::new(token_address, &client);
    
    println!("ERC20 Token initialized at: {:?}", token_address);
    
    // Get token info (would work with real RPC)
    match erc20.name().await {
        Ok(name) => println!("Token Name: {}", name),
        Err(e) => println!("Could not fetch name: {}", e),
    }
    
    match erc20.symbol().await {
        Ok(symbol) => println!("Token Symbol: {}", symbol),
        Err(e) => println!("Could not fetch symbol: {}", e),
    }
    
    match erc20.decimals().await {
        Ok(decimals) => println!("Token Decimals: {}", decimals),
        Err(e) => println!("Could not fetch decimals: {}", e),
    }

    // Prepare transfer
    let recipient: Address = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".parse()?;
    let amount = U256::from(100_000_000_000_000_000u128); // 0.1 tokens (assuming 18 decimals)
    
    println!("\nPreparing ERC20 transfer...");
    println!("  To: {:?}", recipient);
    println!("  Amount: {}", amount);
    
    let tx_data = erc20.encode_transfer(recipient, amount)?;
    println!("  Encoded data: 0x{}", hex::encode(&tx_data));

    println!("\n---\n");

    // ===== ERC721 NFT Example =====
    println!("--- ERC721 NFT Operations ---");
    
    let nft_address: Address = "0x9876543210987654321098765432109876543210".parse()?;
    let erc721 = ERC721::new(nft_address, &client);
    
    println!("ERC721 NFT initialized at: {:?}", nft_address);
    
    // Get NFT info
    match erc721.name().await {
        Ok(name) => println!("NFT Collection Name: {}", name),
        Err(e) => println!("Could not fetch name: {}", e),
    }
    
    match erc721.symbol().await {
        Ok(symbol) => println!("NFT Symbol: {}", symbol),
        Err(e) => println!("Could not fetch symbol: {}", e),
    }

    // Prepare NFT transfer
    let from: Address = "0x1111111111111111111111111111111111111111".parse()?;
    let to: Address = "0x2222222222222222222222222222222222222222".parse()?;
    let token_id = U256::from(42);
    
    println!("\nPreparing ERC721 transfer...");
    println!("  From: {:?}", from);
    println!("  To: {:?}", to);
    println!("  Token ID: {}", token_id);
    
    let tx_data = erc721.encode_transfer_from(from, to, token_id)?;
    println!("  Encoded data: 0x{}", hex::encode(&tx_data));

    println!("\n=== Example Complete ===");
    Ok(())
}
