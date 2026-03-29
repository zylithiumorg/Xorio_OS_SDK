//! Smart Contract interaction example for Xorion Web3 OS SDK

use xorion_sdk::contract::{Contract, ContractCall};
use xorion_sdk::client::Client;
use xorion_sdk::config::Config;
use xorion_sdk::error::XorionResult;
use alloy_primitives::{Address, U256};

#[tokio::main]
async fn main() -> XorionResult<()> {
    println!("=== Xorion Smart Contract Example ===\n");

    // Create Ethereum client
    let config = Config::default();
    let client = Client::new(&config)?;

    // Example: ERC20 Token Contract
    // Replace with actual contract address
    let token_address: Address = "0x1234567890123456789012345678901234567890".parse()?;
    
    println!("Creating contract instance at: {:?}", token_address);
    
    // ERC20 ABI (simplified)
    let erc20_abi = r#"[
        function name() view returns (string),
        function symbol() view returns (string),
        function decimals() view returns (uint8),
        function totalSupply() view returns (uint256),
        function balanceOf(address account) view returns (uint256),
        function transfer(address to, uint256 amount) returns (bool)
    ]"#;

    let contract = Contract::new(token_address, erc20_abi, &client)?;
    println!("Contract instance created!");

    // Example: Read contract data (would work with real RPC)
    println!("\nAttempting to read contract data...");
    
    // Note: These calls would work with a real RPC connection
    match contract.call_function("name", &[]).await {
        Ok(result) => println!("Token Name: {:?}", result),
        Err(e) => println!("Could not fetch token name (expected without real RPC): {}", e),
    }

    match contract.call_function("symbol", &[]).await {
        Ok(result) => println!("Token Symbol: {:?}", result),
        Err(e) => println!("Could not fetch token symbol: {}", e),
    }

    // Example: Prepare a transfer transaction
    println!("\nPreparing transfer transaction...");
    let recipient: Address = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".parse()?;
    let amount = U256::from(1000);
    
    let tx_data = contract.encode_function_call("transfer", &[
        alloy_sol_types::SolValue::abi_encode(&recipient).into(),
        alloy_sol_types::SolValue::abi_encode(&amount).into(),
    ])?;
    
    println!("Encoded transaction data: 0x{}", hex::encode(&tx_data));

    println!("\n=== Example Complete ===");
    Ok(())
}
