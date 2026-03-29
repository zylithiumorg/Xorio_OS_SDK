//! Token utilities and helpers

use crate::error::{XorionError, Result};
use serde::{Deserialize, Serialize};

/// Token information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Option<String>,
    pub logo_url: Option<String>,
}

impl TokenInfo {
    /// Create new token info
    pub fn new(
        address: String,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Self {
        TokenInfo {
            address,
            name,
            symbol,
            decimals,
            total_supply: None,
            logo_url: None,
        }
    }

    pub fn with_total_supply(mut self, supply: String) -> Self {
        self.total_supply = Some(supply);
        self
    }

    pub fn with_logo_url(mut self, url: String) -> Self {
        self.logo_url = Some(url);
        self
    }
}

/// Format a token amount with proper decimals
pub fn format_token_amount(amount: u64, decimals: u8) -> String {
    let divisor = 10u64.pow(decimals as u32) as f64;
    let value = amount as f64 / divisor;
    
    // Format with appropriate precision based on decimals
    let precision = if decimals <= 6 { decimals as usize } else { 6 };
    format!("{:.precision$}", value, precision = precision)
}

/// Parse a human-readable amount to raw units
pub fn parse_token_amount(amount_str: &str, decimals: u8) -> Result<u64> {
    let value: f64 = amount_str.parse()
        .map_err(|e| XorionError::SerializationError(format!("Invalid amount: {}", e)))?;
    
    let multiplier = 10u64.pow(decimals as u32) as f64;
    Ok((value * multiplier) as u64)
}

/// Validate an Ethereum address
pub fn is_valid_eth_address(address: &str) -> bool {
    if !address.starts_with("0x") {
        return false;
    }
    
    let hex_part = &address[2..];
    if hex_part.len() != 40 {
        return false;
    }
    
    hex_part.chars().all(|c| c.is_ascii_hexdigit())
}

/// Validate a Solana address (base58)
pub fn is_valid_solana_address(address: &str) -> bool {
    // Basic validation - length between 32-44 chars for base58
    if address.len() < 32 || address.len() > 44 {
        return false;
    }
    
    // Base58 alphabet (no 0, O, I, l)
    const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    address.chars().all(|c| BASE58_ALPHABET.contains(c))
}

/// Convert between different token units
pub struct UnitConverter;

impl UnitConverter {
    /// Convert wei to ether
    pub fn wei_to_ether(wei: u64) -> f64 {
        wei as f64 / 1e18
    }

    /// Convert ether to wei
    pub fn ether_to_wei(ether: f64) -> u64 {
        (ether * 1e18) as u64
    }

    /// Convert gwei to wei
    pub fn gwei_to_wei(gwei: u64) -> u64 {
        gwei * 1_000_000_000
    }

    /// Convert wei to gwei
    pub fn wei_to_gwei(wei: u64) -> f64 {
        wei as f64 / 1e9
    }

    /// Convert lamports to SOL
    pub fn lamports_to_sol(lamports: u64) -> f64 {
        lamports as f64 / 1e9
    }

    /// Convert SOL to lamports
    pub fn sol_to_lamports(sol: f64) -> u64 {
        (sol * 1e9) as u64
    }
}

/// Well-known token addresses
pub mod well_known_tokens {
    /// Ethereum mainnet tokens
    pub mod ethereum_mainnet {
        pub const USDT: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";
        pub const USDC: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
        pub const DAI: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";
        pub const WETH: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
        pub const UNI: &str = "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984";
        pub const LINK: &str = "0x514910771AF9Ca656af840dff83E8264EcF986CA";
    }

    /// Solana mainnet tokens
    pub mod solana_mainnet {
        pub const USDC: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
        pub const USDT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
        pub const RAY: &str = "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R";
        pub const SRM: &str = "SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_token_amount() {
        assert_eq!(format_token_amount(1_000_000, 6), "1.000000");
        assert_eq!(format_token_amount(1_000_000_000_000_000_000, 18), "1.000000");
        assert_eq!(format_token_amount(100, 8), "0.000001");
    }

    #[test]
    fn test_parse_token_amount() {
        assert_eq!(parse_token_amount("1.0", 6).unwrap(), 1_000_000);
        assert_eq!(parse_token_amount("1.5", 18).unwrap(), 1_500_000_000_000_000_000);
    }

    #[test]
    fn test_validate_eth_address() {
        assert!(is_valid_eth_address("0x1234567890123456789012345678901234567890"));
        assert!(!is_valid_eth_address("0x123"));
        assert!(!is_valid_eth_address("1234567890123456789012345678901234567890"));
        assert!(!is_valid_eth_address("0xGGGG567890123456789012345678901234567890"));
    }

    #[test]
    fn test_validate_solana_address() {
        assert!(is_valid_solana_address("So11111111111111111111111111111111111111112"));
        assert!(!is_valid_solana_address("0x1234"));
        assert!(!is_valid_solana_address("too_short"));
    }

    #[test]
    fn test_unit_converter() {
        assert_eq!(UnitConverter::wei_to_ether(1_000_000_000_000_000_000), 1.0);
        assert_eq!(UnitConverter::ether_to_wei(1.0), 1_000_000_000_000_000_000);
        
        assert_eq!(UnitConverter::gwei_to_wei(1), 1_000_000_000);
        assert_eq!(UnitConverter::lamports_to_sol(1_000_000_000), 1.0);
        assert_eq!(UnitConverter::sol_to_lamports(1.0), 1_000_000_000);
    }

    #[test]
    fn test_well_known_tokens() {
        assert!(well_known_tokens::ethereum_mainnet::USDT.starts_with("0x"));
        assert!(!well_known_tokens::solana_mainnet::USDC.starts_with("0x"));
    }
}
