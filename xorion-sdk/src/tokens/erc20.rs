//! ERC20 token implementation

use crate::error::{XorionError, Result};
use crate::contract::Contract;
use crate::contract::abi::ContractAbi;

/// ERC20 Token handler
pub struct ERC20Token {
    contract: Contract,
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

impl ERC20Token {
    /// Create a new ERC20 token instance
    pub fn new(
        address: String,
        name: String,
        symbol: String,
        decimals: u8,
        rpc_url: String,
    ) -> Self {
        // Minimal ERC20 ABI
        let abi_json = r#"[
            {"type": "function", "name": "balanceOf", "inputs": [{"name": "account", "type": "address"}], "outputs": [{"name": "", "type": "uint256"}], "stateMutability": "view"},
            {"type": "function", "name": "transfer", "inputs": [{"name": "to", "type": "address"}, {"name": "amount", "type": "uint256"}], "outputs": [{"name": "", "type": "bool"}], "stateMutability": "nonpayable"},
            {"type": "function", "name": "approve", "inputs": [{"name": "spender", "type": "address"}, {"name": "amount", "type": "uint256"}], "outputs": [{"name": "", "type": "bool"}], "stateMutability": "nonpayable"},
            {"type": "function", "name": "allowance", "inputs": [{"name": "owner", "type": "address"}, {"name": "spender", "type": "address"}], "outputs": [{"name": "", "type": "uint256"}], "stateMutability": "view"},
            {"type": "function", "name": "totalSupply", "inputs": [], "outputs": [{"name": "", "type": "uint256"}], "stateMutability": "view"},
            {"type": "event", "name": "Transfer", "inputs": [{"name": "from", "type": "address", "indexed": true}, {"name": "to", "type": "address", "indexed": true}, {"name": "value", "type": "uint256", "indexed": false}], "anonymous": false},
            {"type": "event", "name": "Approval", "inputs": [{"name": "owner", "type": "address", "indexed": true}, {"name": "spender", "type": "address", "indexed": true}, {"name": "value", "type": "uint256", "indexed": false}], "anonymous": false}
        ]"#;

        let abi = ContractAbi::from_json(abi_json).unwrap();
        let contract = Contract::new(address.clone(), abi, rpc_url);

        ERC20Token {
            contract,
            address,
            name,
            symbol,
            decimals,
        }
    }

    /// Get balance of an account
    pub async fn balance_of(&self, account: &str) -> Result<String> {
        // In production, this would call the RPC to get the actual balance
        // For now, return a placeholder
        Ok("0".to_string())
    }

    /// Encode transfer transaction data
    pub fn encode_transfer(&self, to: &str, amount: u64) -> Result<String> {
        self.contract.encode_call(
            "transfer",
            &[
                serde_json::json!(to),
                serde_json::json!(amount),
            ],
        )
    }

    /// Encode approve transaction data
    pub fn encode_approve(&self, spender: &str, amount: u64) -> Result<String> {
        self.contract.encode_call(
            "approve",
            &[
                serde_json::json!(spender),
                serde_json::json!(amount),
            ],
        )
    }

    /// Convert raw amount to human-readable format
    pub fn from_wei(&self, amount: u64) -> f64 {
        amount as f64 / 10u64.pow(self.decimals as u32) as f64
    }

    /// Convert human-readable amount to raw (wei) format
    pub fn to_wei(&self, amount: f64) -> u64 {
        (amount * 10u64.pow(self.decimals as u32) as f64) as u64
    }

    /// Format amount with symbol
    pub fn format_amount(&self, amount: u64) -> String {
        format!("{:.6} {}", self.from_wei(amount), self.symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erc20_creation() {
        let token = ERC20Token::new(
            "0x1234567890123456789012345678901234567890".to_string(),
            "Test Token".to_string(),
            "TEST".to_string(),
            18,
            "https://rpc.example.com".to_string(),
        );

        assert_eq!(token.symbol, "TEST");
        assert_eq!(token.decimals, 18);
    }

    #[test]
    fn test_wei_conversion() {
        let token = ERC20Token::new(
            "0x1234".to_string(),
            "Test".to_string(),
            "TEST".to_string(),
            18,
            "https://rpc.example.com".to_string(),
        );

        let amount = 1_000_000_000_000_000_000u64; // 1 token with 18 decimals
        assert_eq!(token.from_wei(amount), 1.0);
        assert_eq!(token.to_wei(1.0), amount);
    }

    #[test]
    fn test_format_amount() {
        let token = ERC20Token::new(
            "0x1234".to_string(),
            "USDC".to_string(),
            "USDC".to_string(),
            6,
            "https://rpc.example.com".to_string(),
        );

        let amount = 1_000_000u64; // 1 USDC
        assert_eq!(token.format_amount(amount), "1.000000 USDC");
    }
}
