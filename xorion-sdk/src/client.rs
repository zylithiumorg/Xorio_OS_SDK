//! RPC Client module for Xorion SDK

use crate::error::{XorionError, Result};
use crate::config::Config;
use serde_json::Value;
use std::time::Duration;

/// Generic RPC Client for blockchain interactions
pub struct RpcClient {
    config: Config,
    client: reqwest::Client,
}

impl RpcClient {
    /// Create a new RPC client with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| XorionError::NetworkError(e.to_string()))?;

        Ok(RpcClient { config, client })
    }

    /// Create a client with default Ethereum mainnet configuration
    pub fn ethereum_mainnet() -> Result<Self> {
        Self::new(Config::ethereum_mainnet())
    }

    /// Create a client with default Solana mainnet configuration
    pub fn solana_mainnet() -> Result<Self> {
        Self::new(Config::solana_mainnet())
    }

    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Send a JSON-RPC request
    pub async fn request(&self, method: &str, params: Value) -> Result<Value> {
        let rpc_url = self.config.get_rpc_url();
        
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let mut retry_count = 0;
        loop {
            match self.client
                .post(&rpc_url)
                .json(&request_body)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        let result: Value = response
                            .json()
                            .await
                            .map_err(|e| XorionError::SerializationError(e.to_string()))?;
                        
                        // Check for RPC error in response
                        if let Some(error) = result.get("error") {
                            return Err(XorionError::RpcError(error.to_string()));
                        }
                        
                        return Ok(result["result"].clone());
                    } else {
                        return Err(XorionError::RpcError(format!(
                            "HTTP error: {}",
                            response.status()
                        )));
                    }
                }
                Err(e) => {
                    retry_count += 1;
                    if retry_count >= self.config.max_retries {
                        return Err(XorionError::NetworkError(e.to_string()));
                    }
                    // Retry on network error
                    tokio::time::sleep(Duration::from_millis(100 * retry_count)).await;
                }
            }
        }
    }

    /// Get block number (Ethereum specific)
    pub async fn get_block_number(&self) -> Result<u64> {
        let result = self.request("eth_blockNumber", serde_json::json!([])).await?;
        let hex_str = result.as_str().ok_or_else(|| {
            XorionError::RpcError("Invalid block number response".to_string())
        })?;
        
        u64::from_str_radix(hex_str.trim_start_matches("0x"), 16)
            .map_err(|e| XorionError::RpcError(e.to_string()))
    }

    /// Get balance (generic)
    pub async fn get_balance(&self, address: &str) -> Result<String> {
        let params = serde_json::json!([address, "latest"]);
        let result = self.request("eth_getBalance", params).await?;
        
        Ok(result.as_str().unwrap_or("0").to_string())
    }

    /// Get chain ID
    pub async fn get_chain_id(&self) -> Result<u64> {
        let result = self.request("eth_chainId", serde_json::json!([])).await?;
        let hex_str = result.as_str().ok_or_else(|| {
            XorionError::RpcError("Invalid chain ID response".to_string())
        })?;
        
        u64::from_str_radix(hex_str.trim_start_matches("0x"), 16)
            .map_err(|e| XorionError::RpcError(e.to_string()))
    }
}

/// Builder for RpcClient
pub struct RpcClientBuilder {
    config: Config,
}

impl RpcClientBuilder {
    pub fn new() -> Self {
        RpcClientBuilder {
            config: Config::default(),
        }
    }

    pub fn network(mut self, network: crate::config::Network) -> Self {
        self.config.network = network;
        self
    }

    pub fn rpc_url(mut self, url: String) -> Self {
        self.config.rpc_url = Some(url);
        self
    }

    pub fn timeout(mut self, secs: u64) -> Self {
        self.config.timeout_secs = secs;
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }

    pub fn api_key(mut self, key: String) -> Self {
        self.config.api_key = Some(key);
        self
    }

    pub fn build(self) -> Result<RpcClient> {
        RpcClient::new(self.config)
    }
}

impl Default for RpcClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let client = RpcClientBuilder::new()
            .network(crate::config::Network::EthereumSepolia)
            .timeout(60)
            .max_retries(5)
            .build();
        
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_creation() {
        let config = Config::ethereum_mainnet();
        let client = RpcClient::new(config);
        assert!(client.is_ok());
    }
}
