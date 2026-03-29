//! Configuration module for Xorion SDK

use serde::{Deserialize, Serialize};
use crate::error::{XorionError, Result};

/// Supported blockchain networks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    /// Ethereum Mainnet
    EthereumMainnet,
    /// Ethereum Goerli Testnet
    EthereumGoerli,
    /// Ethereum Sepolia Testnet
    EthereumSepolia,
    /// Solana Mainnet
    SolanaMainnet,
    /// Solana Devnet
    SolanaDevnet,
    /// Solana Testnet
    SolanaTestnet,
    /// Custom network with RPC URL
    Custom(String),
}

impl Network {
    /// Get the default RPC URL for the network
    pub fn rpc_url(&self) -> String {
        match self {
            Network::EthereumMainnet => "https://eth.llamarpc.com".to_string(),
            Network::EthereumGoerli => "https://goerli.infura.io/v3/".to_string(),
            Network::EthereumSepolia => "https://sepolia.infura.io/v3/".to_string(),
            Network::SolanaMainnet => "https://api.mainnet-beta.solana.com".to_string(),
            Network::SolanaDevnet => "https://api.devnet.solana.com".to_string(),
            Network::SolanaTestnet => "https://api.testnet.solana.com".to_string(),
            Network::Custom(url) => url.clone(),
        }
    }

    /// Check if the network is Ethereum
    pub fn is_ethereum(&self) -> bool {
        matches!(
            self,
            Network::EthereumMainnet
                | Network::EthereumGoerli
                | Network::EthereumSepolia
                | Network::Custom(_)
        )
    }

    /// Check if the network is Solana
    pub fn is_solana(&self) -> bool {
        matches!(
            self,
            Network::SolanaMainnet | Network::SolanaDevnet | Network::SolanaTestnet
        )
    }

    /// Get chain ID for Ethereum networks
    pub fn chain_id(&self) -> Option<u64> {
        match self {
            Network::EthereumMainnet => Some(1),
            Network::EthereumGoerli => Some(5),
            Network::EthereumSepolia => Some(11155111),
            _ => None,
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Network::EthereumMainnet
    }
}

/// SDK Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Network to connect to
    pub network: Network,
    /// RPC endpoint URL (overrides network default if set)
    pub rpc_url: Option<String>,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// Maximum number of retries for failed requests
    pub max_retries: u32,
    /// API key for premium RPC providers (optional)
    pub api_key: Option<String>,
    /// WebSocket endpoint (optional)
    pub ws_url: Option<String>,
}

impl Config {
    /// Create a new configuration with default values
    pub fn new(network: Network) -> Self {
        Config {
            network,
            rpc_url: None,
            timeout_secs: 30,
            max_retries: 3,
            api_key: None,
            ws_url: None,
        }
    }

    /// Create configuration for Ethereum mainnet
    pub fn ethereum_mainnet() -> Self {
        Config::new(Network::EthereumMainnet)
    }

    /// Create configuration for Solana mainnet
    pub fn solana_mainnet() -> Self {
        Config::new(Network::SolanaMainnet)
    }

    /// Set custom RPC URL
    pub fn with_rpc_url(mut self, url: String) -> Self {
        self.rpc_url = Some(url);
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Set maximum retries
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// Set API key
    pub fn with_api_key(mut self, key: String) -> Self {
        self.api_key = Some(key);
        self
    }

    /// Set WebSocket URL
    pub fn with_ws_url(mut self, url: String) -> Self {
        self.ws_url = Some(url);
        self
    }

    /// Get the effective RPC URL
    pub fn get_rpc_url(&self) -> String {
        self.rpc_url.clone().unwrap_or_else(|| self.network.rpc_url())
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.timeout_secs == 0 {
            return Err(XorionError::ConfigError(
                "Timeout must be greater than 0".to_string(),
            ));
        }

        // Validate RPC URL if custom one is provided
        if let Some(ref url) = self.rpc_url {
            url::Url::parse(url).map_err(|e| {
                XorionError::ConfigError(format!("Invalid RPC URL: {}", e))
            })?;
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(Network::EthereumMainnet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_rpc_url() {
        assert_eq!(
            Network::EthereumMainnet.rpc_url(),
            "https://eth.llamarpc.com"
        );
        assert_eq!(
            Network::SolanaMainnet.rpc_url(),
            "https://api.mainnet-beta.solana.com"
        );
    }

    #[test]
    fn test_network_chain_id() {
        assert_eq!(Network::EthereumMainnet.chain_id(), Some(1));
        assert_eq!(Network::EthereumSepolia.chain_id(), Some(11155111));
        assert_eq!(Network::SolanaMainnet.chain_id(), None);
    }

    #[test]
    fn test_config_builder() {
        let config = Config::ethereum_mainnet()
            .with_timeout(60)
            .with_max_retries(5)
            .with_api_key("test-key".to_string());

        assert_eq!(config.timeout_secs, 60);
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.api_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_config_validation() {
        let config = Config::default();
        assert!(config.validate().is_ok());

        let invalid_config = Config {
            timeout_secs: 0,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }
}
