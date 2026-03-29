//! DeFi module for Xorion SDK

pub mod swap;
pub mod liquidity;
pub mod dex;

pub use swap::SwapRouter;
pub use liquidity::LiquidityPool;
pub use dex::DexProtocol;

/// DeFi operation type
#[derive(Debug, Clone)]
pub enum DefiOperation {
    Swap,
    AddLiquidity,
    RemoveLiquidity,
    Stake,
    Unstake,
    Claim,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defi_operation() {
        let op = DefiOperation::Swap;
        assert!(matches!(op, DefiOperation::Swap));
    }
}
