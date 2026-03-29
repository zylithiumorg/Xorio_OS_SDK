//! Liquidity pool management for DeFi

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Liquidity pool representation
pub struct LiquidityPool {
    pub pool_address: String,
    pub token0: String,
    pub token1: String,
    pub fee_tier: u32,
}

/// Liquidity position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPosition {
    pub pool_address: String,
    pub owner: String,
    pub liquidity: u128,
    pub token0_amount: u64,
    pub token1_amount: u64,
    pub unclaimed_fees_token0: Option<u64>,
    pub unclaimed_fees_token1: Option<u64>,
}

/// Add liquidity parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddLiquidityParams {
    pub token0: String,
    pub token1: String,
    pub amount0_desired: u64,
    pub amount1_desired: u64,
    pub amount0_min: u64,
    pub amount1_min: u64,
    pub recipient: String,
    pub deadline: u64,
}

impl AddLiquidityParams {
    /// Create new add liquidity parameters
    pub fn new(
        token0: String,
        token1: String,
        amount0: u64,
        amount1: u64,
        recipient: String,
    ) -> Self {
        AddLiquidityParams {
            token0,
            token1,
            amount0_desired: amount0,
            amount1_desired: amount1,
            amount0_min: 0,
            amount1_min: 0,
            recipient,
            deadline: 0,
        }
    }

    /// Set minimum amounts with slippage
    pub fn with_slippage(mut self, slippage_bps: u16) -> Self {
        self.amount0_min = self.amount0_desired * (10000 - slippage_bps as u64) / 10000;
        self.amount1_min = self.amount1_desired * (10000 - slippage_bps as u64) / 10000;
        self
    }
}

/// Remove liquidity parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveLiquidityParams {
    pub pool_address: String,
    pub liquidity: u128,
    pub amount0_min: u64,
    pub amount1_min: u64,
    pub recipient: String,
    pub deadline: u64,
}

impl LiquidityPool {
    /// Create a new liquidity pool instance
    pub fn new(
        pool_address: String,
        token0: String,
        token1: String,
        fee_tier: u32,
    ) -> Self {
        LiquidityPool {
            pool_address,
            token0,
            token1,
            fee_tier,
        }
    }

    /// Encode add liquidity transaction
    pub fn encode_add_liquidity(&self, params: &AddLiquidityParams) -> Result<String> {
        // Simplified encoding - in production use proper ABI
        Ok(format!(
            "0xe8e33700{:0>64}{:0>64}{:0>64}{:0>64}",
            params.amount0_desired,
            params.amount1_desired,
            params.amount0_min,
            params.amount1_min
        ))
    }

    /// Encode remove liquidity transaction
    pub fn encode_remove_liquidity(&self, params: &RemoveLiquidityParams) -> Result<String> {
        // Simplified encoding
        Ok(format!(
            "0x89afcb44{:0>64}",
            params.liquidity
        ))
    }

    /// Calculate optimal deposit amounts based on current ratio
    pub fn calculate_optimal_deposit(
        &self,
        reserve0: u64,
        reserve1: u64,
        amount0: u64,
    ) -> (u64, u64) {
        if reserve0 == 0 {
            return (amount0, amount0); // Initial deposit
        }

        let amount1 = (amount0 as u128 * reserve1 as u128 / reserve0 as u128) as u64;
        (amount0, amount1)
    }

    /// Calculate share of pool
    pub fn calculate_pool_share(
        &self,
        liquidity: u128,
        total_supply: u128,
    ) -> f64 {
        if total_supply == 0 {
            return 100.0;
        }
        (liquidity as f64 / total_supply as f64) * 100.0
    }
}

/// Pool reserves information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolReserves {
    pub reserve0: u64,
    pub reserve1: u64,
    pub block_timestamp_last: u64,
}

impl PoolReserves {
    /// Get the current price of token0 in terms of token1
    pub fn price_0_in_1(&self) -> f64 {
        if self.reserve0 == 0 {
            return 0.0;
        }
        self.reserve1 as f64 / self.reserve0 as f64
    }

    /// Get the current price of token1 in terms of token0
    pub fn price_1_in_0(&self) -> f64 {
        if self.reserve1 == 0 {
            return 0.0;
        }
        self.reserve0 as f64 / self.reserve1 as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liquidity_pool_creation() {
        let pool = LiquidityPool::new(
            "0xpool".to_string(),
            "0xtoken0".to_string(),
            "0xtoken1".to_string(),
            3000, // 0.3% fee tier
        );

        assert_eq!(pool.fee_tier, 3000);
    }

    #[test]
    fn test_add_liquidity_params() {
        let params = AddLiquidityParams::new(
            "0xtoken0".to_string(),
            "0xtoken1".to_string(),
            1000000,
            2000000,
            "0xrecipient".to_string(),
        ).with_slippage(50); // 0.5%

        assert!(params.amount0_min > 0);
        assert!(params.amount1_min > 0);
    }

    #[test]
    fn test_optimal_deposit() {
        let pool = LiquidityPool::new(
            "0xpool".to_string(),
            "0xtoken0".to_string(),
            "0xtoken1".to_string(),
            3000,
        );

        let (amount0, amount1) = pool.calculate_optimal_deposit(1000000, 2000000, 10000);
        assert_eq!(amount0, 10000);
        assert_eq!(amount1, 20000);
    }

    #[test]
    fn test_pool_reserves() {
        let reserves = PoolReserves {
            reserve0: 1000000,
            reserve1: 2000000,
            block_timestamp_last: 1234567890,
        };

        assert_eq!(reserves.price_0_in_1(), 2.0);
        assert!((reserves.price_1_in_0() - 0.5).abs() < 0.001);
    }
}
