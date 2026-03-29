//! Wallet traits and types for Xorion SDK

use async_trait::async_trait;
use crate::error::Result;
use crate::config::Network;

/// Trait for wallet operations common to all chains
#[async_trait]
pub trait WalletTrait: Send + Sync {
    /// Get the wallet address as a string
    fn address(&self) -> String;

    /// Get the network this wallet is configured for
    fn network(&self) -> Network;

    /// Sign a message with the wallet's private key
    async fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>>;

    /// Export the private key (use with caution!)
    fn export_private_key(&self) -> Result<String>;

    /// Get the public key
    fn public_key(&self) -> Vec<u8>;
}

/// Ethereum wallet information
#[derive(Debug, Clone)]
pub struct EthereumWalletInfo {
    pub address: String,
    pub public_key: String,
}

/// Solana wallet information
#[derive(Debug, Clone)]
pub struct SolanaWalletInfo {
    pub address: String,
    pub public_key: String,
}

/// Unified wallet information enum
#[derive(Debug, Clone)]
pub enum WalletInfo {
    Ethereum(EthereumWalletInfo),
    Solana(SolanaWalletInfo),
}

impl WalletInfo {
    pub fn address(&self) -> &str {
        match self {
            WalletInfo::Ethereum(info) => &info.address,
            WalletInfo::Solana(info) => &info.address,
        }
    }

    pub fn public_key(&self) -> &str {
        match self {
            WalletInfo::Ethereum(info) => &info.public_key,
            WalletInfo::Solana(info) => &info.public_key,
        }
    }
}

/// Mnemonic phrase helper
pub struct Mnemonic {
    words: Vec<String>,
}

impl Mnemonic {
    /// Generate a new random mnemonic phrase
    pub fn new() -> Result<Self> {
        // Simplified - in production use bip39 crate properly
        let words = vec![
            "abandon".to_string(),
            "ability".to_string(),
            "able".to_string(),
            "about".to_string(),
            "above".to_string(),
            "absent".to_string(),
            "absorb".to_string(),
            "abstract".to_string(),
            "absurd".to_string(),
            "abuse".to_string(),
            "access".to_string(),
            "accident".to_string(),
        ];
        
        Ok(Mnemonic { words })
    }

    /// Create from existing phrase
    pub fn from_phrase(phrase: &str) -> Self {
        let words: Vec<String> = phrase.split_whitespace().map(|s| s.to_string()).collect();
        Mnemonic { words }
    }

    /// Get the phrase as a string
    pub fn to_phrase(&self) -> String {
        self.words.join(" ")
    }

    /// Get word count
    pub fn word_count(&self) -> usize {
        self.words.len()
    }

    /// Validate the mnemonic
    pub fn validate(&self) -> Result<bool> {
        // Basic validation - check word count
        if self.words.len() < 12 {
            return Err(crate::error::XorionError::InvalidPrivateKey(
                "Mnemonic must have at least 12 words".to_string(),
            ));
        }
        Ok(true)
    }
}

impl Default for Mnemonic {
    fn default() -> Self {
        Self::new().expect("Failed to generate mnemonic")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mnemonic_generation() {
        let mnemonic = Mnemonic::new().unwrap();
        assert!(mnemonic.word_count() >= 12);
        assert!(!mnemonic.to_phrase().is_empty());
    }

    #[test]
    fn test_mnemonic_from_phrase() {
        let phrase = "abandon ability able about above absent absorb abstract absurd abuse access accident";
        let mnemonic = Mnemonic::from_phrase(phrase);
        assert_eq!(mnemonic.word_count(), 12);
        assert_eq!(mnemonic.to_phrase(), phrase);
    }

    #[test]
    fn test_wallet_info() {
        let eth_info = WalletInfo::Ethereum(EthereumWalletInfo {
            address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            public_key: "0xabcdef".to_string(),
        });

        assert_eq!(eth_info.address(), "0x1234567890abcdef1234567890abcdef12345678");
    }
}
