//! Balance module for Xorion SDK

use serde::{Deserialize, Serialize};
use crate::error::Result;

/// Token balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// Address of the account
    pub address: String,
    /// Native token balance (ETH/SOL)
    pub native: String,
    /// Native token symbol
    pub symbol: String,
    /// Decimals for the native token
    pub decimals: u8,
    /// USD value (if available)
    pub usd_value: Option<f64>,
    /// Last updated block number
    pub block_number: Option<u64>,
}

impl Balance {
    /// Create a new balance object
    pub fn new(address: String, native: String, symbol: String, decimals: u8) -> Self {
        Balance {
            address,
            native,
            symbol,
            decimals,
            usd_value: None,
            block_number: None,
        }
    }

    /// Set USD value
    pub fn with_usd_value(mut self, value: f64) -> Self {
        self.usd_value = Some(value);
        self
    }

    /// Set block number
    pub fn with_block_number(mut self, block: u64) -> Self {
        self.block_number = Some(block);
        self
    }

    /// Get balance in human-readable format
    pub fn formatted(&self) -> String {
        let value: f64 = self.native.parse().unwrap_or(0.0);
        let divisor = 10u64.pow(self.decimals as u32) as f64;
        format!("{:.6} {}", value / divisor, self.symbol)
    }

    /// Get balance as f64 (in smallest unit)
    pub fn as_f64(&self) -> f64 {
        self.native.parse().unwrap_or(0.0)
    }

    /// Check if balance is zero
    pub fn is_zero(&self) -> bool {
        self.as_f64() == 0.0
    }
}

/// ERC20 token balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    /// Token contract address
    pub token_address: String,
    /// Account address
    pub account_address: String,
    /// Token balance
    pub balance: String,
    /// Token name
    pub name: String,
    /// Token symbol
    pub symbol: String,
    /// Token decimals
    pub decimals: u8,
    /// USD value (if available)
    pub usd_value: Option<f64>,
}

impl TokenBalance {
    /// Create a new token balance
    pub fn new(
        token_address: String,
        account_address: String,
        balance: String,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Self {
        TokenBalance {
            token_address,
            account_address,
            balance,
            name,
            symbol,
            decimals,
            usd_value: None,
        }
    }

    /// Get formatted balance
    pub fn formatted(&self) -> String {
        let value: f64 = self.balance.parse().unwrap_or(0.0);
        let divisor = 10u64.pow(self.decimals as u32) as f64;
        format!("{:.6} {}", value / divisor, self.symbol)
    }
}

/// Portfolio summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    /// Total USD value
    pub total_usd_value: f64,
    /// Native balance
    pub native_balance: Balance,
    /// Token balances
    pub token_balances: Vec<TokenBalance>,
}

impl Portfolio {
    /// Create a new portfolio
    pub fn new(native_balance: Balance) -> Self {
        Portfolio {
            total_usd_value: 0.0,
            native_balance,
            token_balances: Vec::new(),
        }
    }

    /// Add token balance
    pub fn add_token(mut self, token: TokenBalance) -> Self {
        self.token_balances.push(token);
        self
    }

    /// Calculate total USD value
    pub fn calculate_total_value(&mut self) -> f64 {
        let mut total = self.native_balance.usd_value.unwrap_or(0.0);
        for token in &self.token_balances {
            total += token.usd_value.unwrap_or(0.0);
        }
        self.total_usd_value = total;
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_creation() {
        let balance = Balance::new(
            "0x1234".to_string(),
            "1000000000000000000".to_string(),
            "ETH".to_string(),
            18,
        );

        assert_eq!(balance.address, "0x1234");
        assert_eq!(balance.symbol, "ETH");
        assert_eq!(balance.decimals, 18);
    }

    #[test]
    fn test_balance_formatted() {
        let balance = Balance::new(
            "0x1234".to_string(),
            "1500000000000000000".to_string(),
            "ETH".to_string(),
            18,
        );

        assert_eq!(balance.formatted(), "1.500000 ETH");
    }

    #[test]
    fn test_token_balance() {
        let token = TokenBalance::new(
            "0xtoken".to_string(),
            "0xaccount".to_string(),
            "1000000".to_string(),
            "USDC".to_string(),
            "USDC".to_string(),
            6,
        );

        assert_eq!(token.formatted(), "1.000000 USDC");
    }

    #[test]
    fn test_portfolio() {
        let native = Balance::new(
            "0x1234".to_string(),
            "1000000000000000000".to_string(),
            "ETH".to_string(),
            18,
        );

        let mut portfolio = Portfolio::new(native);
        
        let token = TokenBalance::new(
            "0xtoken".to_string(),
            "0x1234".to_string(),
            "1000000".to_string(),
            "USDC".to_string(),
            "USDC".to_string(),
            6,
        );

        portfolio = portfolio.add_token(token);
        assert_eq!(portfolio.token_balances.len(), 1);
    }
}
