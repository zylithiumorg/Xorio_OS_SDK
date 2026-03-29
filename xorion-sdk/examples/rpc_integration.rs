//! RPC Integration example for Xorion Web3 OS SDK

use xorion_sdk::client::{Client, Chain};
use xorion_sdk::config::Config;
use xorion_sdk::error::XorionResult;

#[tokio::main]
async fn main() -> XorionResult<()> {
    println!("=== Xorion RPC Integration Example ===\n");

    // Create Ethereum client
    println!("Connecting to Ethereum RPC...");
    let eth_config = Config::default().with_chain(Chain::Ethereum);
    let eth_client = Client::new(&eth_config)?;
    
    println!("Ethereum Client created successfully!");
    println!("Chain: {:?}", eth_client.chain());

    // Get latest block number (example)
    match eth_client.get_block_number().await {
        Ok(block_num) => println!("Latest Ethereum Block: {}", block_num),
        Err(e) => println!("Could not fetch block number: {}", e),
    }

    println!("\n---\n");

    // Create Solana client
    println!("Connecting to Solana RPC...");
    let sol_config = Config::default().with_chain(Chain::Solana);
    let sol_client = Client::new(&sol_config)?;
    
    println!("Solana Client created successfully!");
    println!("Chain: {:?}", sol_client.chain());

    // Get slot (example)
    match sol_client.get_slot().await {
        Ok(slot) => println!("Current Solana Slot: {}", slot),
        Err(e) => println!("Could not fetch slot: {}", e),
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
