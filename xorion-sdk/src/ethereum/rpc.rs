//! Ethereum RPC client implementation

use crate::error::{XorionError, Result};
use crate::config::Config;
use serde_json::Value;

/// Ethereum-specific RPC client
pub struct EthereumRpc {
    config: Config,
    client: reqwest::Client,
}

impl EthereumRpc {
    /// Create a new Ethereum RPC client
    pub fn new(config: Config) -> Result<Self> {
        if !config.network.is_ethereum() {
            return Err(XorionError::UnsupportedChain(
                "Network is not an Ethereum network".to_string(),
            ));
        }

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| XorionError::NetworkError(e.to_string()))?;

        Ok(EthereumRpc { config, client })
    }

    /// Create with default mainnet configuration
    pub fn mainnet() -> Result<Self> {
        Self::new(Config::ethereum_mainnet())
    }

    /// Send JSON-RPC request
    async fn request(&self, method: &str, params: Value) -> Result<Value> {
        let rpc_url = self.config.get_rpc_url();
        
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let response = self.client
            .post(&rpc_url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| XorionError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(XorionError::RpcError(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        let result: Value = response
            .json()
            .await
            .map_err(|e| XorionError::SerializationError(e.to_string()))?;

        if let Some(error) = result.get("error") {
            return Err(XorionError::RpcError(error.to_string()));
        }

        Ok(result["result"].clone())
    }

    /// Get the latest block number
    pub async fn get_block_number(&self) -> Result<u64> {
        let result = self.request("eth_blockNumber", serde_json::json!([])).await?;
        let hex_str = result.as_str().ok_or_else(|| {
            XorionError::RpcError("Invalid block number response".to_string())
        })?;
        
        u64::from_str_radix(hex_str.trim_start_matches("0x"), 16)
            .map_err(|e| XorionError::RpcError(e.to_string()))
    }

    /// Get balance of an address
    pub async fn get_balance(&self, address: &str) -> Result<String> {
        let params = serde_json::json!([address, "latest"]);
        let result = self.request("eth_getBalance", params).await?;
        Ok(result.as_str().unwrap_or("0x0").to_string())
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

    /// Get gas price
    pub async fn get_gas_price(&self) -> Result<String> {
        let result = self.request("eth_gasPrice", serde_json::json!([])).await?;
        Ok(result.as_str().unwrap_or("0x0").to_string())
    }

    /// Get nonce for an address
    pub async fn get_nonce(&self, address: &str) -> Result<u64> {
        let params = serde_json::json!([address, "pending"]);
        let result = self.request("eth_getTransactionCount", params).await?;
        let hex_str = result.as_str().ok_or_else(|| {
            XorionError::RpcError("Invalid nonce response".to_string())
        })?;
        
        u64::from_str_radix(hex_str.trim_start_matches("0x"), 16)
            .map_err(|e| XorionError::RpcError(e.to_string()))
    }

    /// Get transaction by hash
    pub async fn get_transaction(&self, tx_hash: &str) -> Result<Value> {
        let params = serde_json::json!([tx_hash]);
        self.request("eth_getTransactionByHash", params).await
    }

    /// Get transaction receipt
    pub async fn get_transaction_receipt(&self, tx_hash: &str) -> Result<Value> {
        let params = serde_json::json!([tx_hash]);
        self.request("eth_getTransactionReceipt", params).await
    }

    /// Send raw transaction
    pub async fn send_raw_transaction(&self, raw_tx: &str) -> Result<String> {
        let params = serde_json::json!([raw_tx]);
        let result = self.request("eth_sendRawTransaction", params).await?;
        Ok(result.as_str().unwrap_or("").to_string())
    }

    /// Call a contract (read-only)
    pub async fn call(&self, tx_object: Value) -> Result<String> {
        let params = serde_json::json!([tx_object, "latest"]);
        let result = self.request("eth_call", params).await?;
        Ok(result.as_str().unwrap_or("0x").to_string())
    }

    /// Estimate gas
    pub async fn estimate_gas(&self, tx_object: Value) -> Result<u64> {
        let params = serde_json::json!([tx_object]);
        let result = self.request("eth_estimateGas", params).await?;
        let hex_str = result.as_str().ok_or_else(|| {
            XorionError::RpcError("Invalid gas estimate response".to_string())
        })?;
        
        u64::from_str_radix(hex_str.trim_start_matches("0x"), 16)
            .map_err(|e| XorionError::RpcError(e.to_string()))
    }

    /// Get block by number
    pub async fn get_block_by_number(&self, block_number: u64) -> Result<Value> {
        let hex_number = format!("0x{:x}", block_number);
        let params = serde_json::json!([hex_number, true]);
        self.request("eth_getBlockByNumber", params).await
    }

    /// Get logs (events)
    pub async fn get_logs(&self, filter: Value) -> Result<Value> {
        self.request("eth_getLogs", filter).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethereum_rpc_creation() {
        let rpc = EthereumRpc::mainnet();
        assert!(rpc.is_ok());
    }

    #[test]
    fn test_invalid_network() {
        let config = Config::solana_mainnet();
        let rpc = EthereumRpc::new(config);
        assert!(rpc.is_err());
    }
}
