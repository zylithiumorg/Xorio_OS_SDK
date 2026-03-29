//! DEX protocol abstractions

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// DEX Protocol trait for common operations
pub trait DexProtocol: Send + Sync {
    /// Get the protocol name
    fn name(&self) -> &str;
    
    /// Get router address
    fn router_address(&self) -> &str;
    
    /// Get factory address
    fn factory_address(&self) -> &str;
}

/// Uniswap V2 implementation
#[derive(Debug, Clone)]
pub struct UniswapV2 {
    pub router: String,
    pub factory: String,
}

impl UniswapV2 {
    pub fn new() -> Self {
        UniswapV2 {
            router: "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".to_string(),
            factory: "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".to_string(),
        }
    }
}

impl Default for UniswapV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DexProtocol for UniswapV2 {
    fn name(&self) -> &str {
        "Uniswap V2"
    }

    fn router_address(&self) -> &str {
        &self.router
    }

    fn factory_address(&self) -> &str {
        &self.factory
    }
}

/// Uniswap V3 implementation
#[derive(Debug, Clone)]
pub struct UniswapV3 {
    pub router: String,
    pub factory: String,
    pub quoter: String,
}

impl UniswapV3 {
    pub fn new() -> Self {
        UniswapV3 {
            router: "0xE592427A0AEce92De3Edee1F18E0157C05861564".to_string(),
            factory: "0x1F98431c8aD98523631AE4a59f267346ea31F984".to_string(),
            quoter: "0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6".to_string(),
        }
    }
}

impl Default for UniswapV3 {
    fn default() -> Self {
        Self::new()
    }
}

impl DexProtocol for UniswapV3 {
    fn name(&self) -> &str {
        "Uniswap V3"
    }

    fn router_address(&self) -> &str {
        &self.router
    }

    fn factory_address(&self) -> &str {
        &self.factory
    }
}

/// SushiSwap implementation
#[derive(Debug, Clone)]
pub struct SushiSwap {
    pub router: String,
    pub factory: String,
}

impl SushiSwap {
    pub fn new() -> Self {
        SushiSwap {
            router: "0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F".to_string(),
            factory: "0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac".to_string(),
        }
    }
}

impl Default for SushiSwap {
    fn default() -> Self {
        Self::new()
    }
}

impl DexProtocol for SushiSwap {
    fn name(&self) -> &str {
        "SushiSwap"
    }

    fn router_address(&self) -> &str {
        &self.router
    }

    fn factory_address(&self) -> &str {
        &self.factory
    }
}

/// Raydium (Solana) implementation
#[derive(Debug, Clone)]
pub struct Raydium {
    pub program_id: String,
}

impl Raydium {
    pub fn new() -> Self {
        Raydium {
            program_id: "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string(),
        }
    }
}

impl Default for Raydium {
    fn default() -> Self {
        Self::new()
    }
}

/// DEX aggregator for finding best routes
pub struct DexAggregator {
    pub protocols: Vec<Box<dyn DexProtocol>>,
}

impl DexAggregator {
    pub fn new() -> Self {
        DexAggregator {
            protocols: Vec::new(),
        }
    }

    pub fn add_protocol(mut self, protocol: Box<dyn DexProtocol>) -> Self {
        self.protocols.push(protocol);
        self
    }

    pub fn get_protocol(&self, name: &str) -> Option<&dyn DexProtocol> {
        self.protocols.iter()
            .find(|p| p.name() == name)
            .map(|p| p.as_ref())
    }

    pub fn list_protocols(&self) -> Vec<&str> {
        self.protocols.iter().map(|p| p.name()).collect()
    }
}

impl Default for DexAggregator {
    fn default() -> Self {
        Self::new()
    }
}

/// Route for multi-hop swaps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapRoute {
    pub path: Vec<String>,
    pub protocols: Vec<String>,
    pub expected_output: u64,
    pub price_impact: f64,
}

impl SwapRoute {
    pub fn new(path: Vec<String>) -> Self {
        SwapRoute {
            path,
            protocols: Vec::new(),
            expected_output: 0,
            price_impact: 0.0,
        }
    }

    pub fn with_protocols(mut self, protocols: Vec<String>) -> Self {
        self.protocols = protocols;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniswap_v2() {
        let uniswap = UniswapV2::new();
        assert_eq!(uniswap.name(), "Uniswap V2");
        assert!(uniswap.router_address().starts_with("0x"));
    }

    #[test]
    fn test_uniswap_v3() {
        let uniswap = UniswapV3::new();
        assert_eq!(uniswap.name(), "Uniswap V3");
        assert!(uniswap.quoter.starts_with("0x"));
    }

    #[test]
    fn test_sushiswap() {
        let sushi = SushiSwap::new();
        assert_eq!(sushi.name(), "SushiSwap");
    }

    #[test]
    fn test_dex_aggregator() {
        let mut aggregator = DexAggregator::new()
            .add_protocol(Box::new(UniswapV2::new()))
            .add_protocol(Box::new(SushiSwap::new()));

        assert_eq!(aggregator.list_protocols().len(), 2);
        
        let protocol = aggregator.get_protocol("Uniswap V2");
        assert!(protocol.is_some());
    }

    #[test]
    fn test_swap_route() {
        let route = SwapRoute::new(vec![
            "0xtoken1".to_string(),
            "0xweth".to_string(),
            "0xtoken2".to_string(),
        ]);

        assert_eq!(route.path.len(), 3);
    }
}
