//! DeFi interactions example for Xorion Web3 OS SDK

use xorion_sdk::defi::{SwapRouter, LiquidityPool, DEX};
use xorion_sdk::client::Client;
use xorion_sdk::config::Config;
use xorion_sdk::error::XorionResult;
use alloy_primitives::{Address, U256};

#[tokio::main]
async fn main() -> XorionResult<()> {
    println!("=== Xorion DeFi Interactions Example ===\n");

    // Create client
    let config = Config::default();
    let client = Client::new(&config)?;

    // ===== Swap Example (Uniswap-like) =====
    println!("--- Token Swap Operations ---");
    
    // Uniswap V2 Router address (example)
    let router_address: Address = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse()?;
    let router = SwapRouter::new(router_address, &client);
    
    println!("Swap Router initialized at: {:?}", router_address);
    
    // Define tokens
    let token_in: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?; // WETH
    let token_out: Address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse()?; // USDC
    let amount_in = U256::from(1_000_000_000_000_000_000u128); // 1 ETH
    
    println!("\nPreparing swap...");
    println!("  From: {:?} (WETH)", token_in);
    println!("  To: {:?} (USDC)", token_out);
    println!("  Amount In: {} wei", amount_in);
    
    // Encode swap transaction
    let min_amount_out = U256::from(0); // In production, calculate proper minimum
    let deadline = 9999999999u64; // Far future
    
    let tx_data = router.encode_swap_exact_tokens_for_tokens(
        token_in,
        token_out,
        amount_in,
        min_amount_out,
        deadline,
    )?;
    
    println!("  Encoded swap data: 0x{}", hex::encode(&tx_data));

    println!("\n---\n");

    // ===== Liquidity Pool Example =====
    println!("--- Liquidity Pool Operations ---");
    
    // Example pool address
    let pool_address: Address = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc".parse()?;
    let pool = LiquidityPool::new(pool_address, &client);
    
    println!("Liquidity Pool initialized at: {:?}", pool_address);
    
    // Get pool reserves (would work with real RPC)
    match pool.get_reserves().await {
        Ok((reserve0, reserve1)) => {
            println!("  Reserve 0: {}", reserve0);
            println!("  Reserve 1: {}", reserve1);
        },
        Err(e) => println!("  Could not fetch reserves: {}", e),
    }
    
    // Calculate price impact
    let trade_amount = U256::from(100_000_000_000_000_000u128); // 0.1 ETH
    match pool.calculate_price_impact(trade_amount).await {
        Ok(impact) => println!("  Price Impact for {} wei: {}%", trade_amount, impact),
        Err(e) => println!("  Could not calculate price impact: {}", e),
    }

    println!("\n---\n");

    // ===== DEX Aggregator Example =====
    println!("--- DEX Aggregator Operations ---");
    
    let dex = DEX::new(&client);
    
    // Find best route for swap
    println!("Finding best swap route...");
    match dex.find_best_route(token_in, token_out, amount_in).await {
        Ok(route) => {
            println!("  Best Route Found!");
            println!("  Expected Output: {}", route.expected_output);
            println!("  Price Impact: {}%", route.price_impact);
            println!("  Route Hops: {:?}", route.hops);
        },
        Err(e) => println!("  Could not find route: {}", e),
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
