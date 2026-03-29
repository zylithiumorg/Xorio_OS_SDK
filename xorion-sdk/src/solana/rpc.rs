//! Solana RPC client implementation

use crate::error::{XorionError, Result};
use crate::config::Config;
use serde_json::Value;

/// Solana-specific RPC client
pub struct SolanaRpc {
    config: Config,
    client: reqwest::Client,
}

impl SolanaRpc {
    /// Create a new Solana RPC client
    pub fn new(config: Config) -> Result<Self> {
        if !config.network.is_solana() {
            return Err(XorionError::UnsupportedChain(
                "Network is not a Solana network".to_string(),
            ));
        }

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| XorionError::NetworkError(e.to_string()))?;

        Ok(SolanaRpc { config, client })
    }

    /// Create with default mainnet configuration
    pub fn mainnet() -> Result<Self> {
        Self::new(Config::solana_mainnet())
    }

    /// Send JSON-RPC request
    async fn request(&self, method: &str, params: Value) -> Result<Value> {
        let rpc_url = self.config.get_rpc_url();
        
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
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

    /// Get the latest slot (block number equivalent in Solana)
    pub async fn get_slot(&self) -> Result<u64> {
        let params = serde_json::json!([]);
        let result = self.request("getSlot", params).await?;
        
        result.as_u64().ok_or_else(|| {
            XorionError::RpcError("Invalid slot response".to_string())
        })
    }

    /// Get balance of a public key
    pub async fn get_balance(&self, pubkey: &str) -> Result<u64> {
        let params = serde_json::json!([pubkey]);
        let result = self.request("getBalance", params).await?;
        
        result["value"].as_u64().ok_or_else(|| {
            XorionError::RpcError("Invalid balance response".to_string())
        })
    }

    /// Get account info
    pub async fn get_account_info(&self, pubkey: &str) -> Result<Value> {
        let params = serde_json::json!([pubkey, {
            "encoding": "base64",
            "commitment": "confirmed"
        }]);
        self.request("getAccountInfo", params).await
    }

    /// Get recent blockhash for transaction signing
    pub async fn get_recent_blockhash(&self) -> Result<String> {
        let params = serde_json::json!([{
            "commitment": "confirmed"
        }]);
        let result = self.request("getLatestBlockhash", params).await?;
        
        result["value"]["blockhash"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| {
                XorionError::RpcError("Invalid blockhash response".to_string())
            })
    }

    /// Get minimum rent exempt balance
    pub async fn get_minimum_balance_for_rent_exemption(&self, data_len: usize) -> Result<u64> {
        let params = serde_json::json!([data_len]);
        let result = self.request("getMinimumBalanceForRentExemption", params).await?;
        
        result.as_u64().ok_or_else(|| {
            XorionError::RpcError("Invalid rent exemption response".to_string())
        })
    }

    /// Send raw transaction
    pub async fn send_transaction(&self, encoded_tx: &str) -> Result<String> {
        let params = serde_json::json!([
            encoded_tx,
            {
                "encoding": "base64",
                "skipPreflight": false,
                "preflightCommitment": "confirmed"
            }
        ]);
        let result = self.request("sendTransaction", params).await?;
        
        result.as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| {
                XorionError::RpcError("Invalid transaction signature response".to_string())
            })
    }

    /// Get transaction status
    pub async fn get_signature_status(&self, signature: &str) -> Result<Value> {
        let params = serde_json::json!([[signature]]);
        self.request("getSignatureStatuses", params).await
    }

    /// Get transaction by signature
    pub async fn get_transaction(&self, signature: &str) -> Result<Value> {
        let params = serde_json::json!([signature, {
            "encoding": "json",
            "commitment": "confirmed"
        }]);
        self.request("getTransaction", params).await
    }

    /// Get token accounts by owner
    pub async fn get_token_accounts_by_owner(&self, owner: &str) -> Result<Value> {
        let params = serde_json::json!([
            owner,
            {
                "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            },
            {
                "encoding": "jsonParsed"
            }
        ]);
        self.request("getTokenAccountsByOwner", params).await
    }

    /// Get epoch info
    pub async fn get_epoch_info(&self) -> Result<Value> {
        let params = serde_json::json!([]);
        self.request("getEpochInfo", params).await
    }

    /// Get health status
    pub async fn get_health(&self) -> Result<String> {
        let params = serde_json::json!([]);
        let result = self.request("getHealth", params).await?;
        
        Ok(result.as_str().unwrap_or("unknown").to_string())
    }

    /// Get version info
    pub async fn get_version(&self) -> Result<Value> {
        let params = serde_json::json!([]);
        self.request("getVersion", params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solana_rpc_creation() {
        let rpc = SolanaRpc::mainnet();
        assert!(rpc.is_ok());
    }

    #[test]
    fn test_invalid_network() {
        let config = Config::ethereum_mainnet();
        let rpc = SolanaRpc::new(config);
        assert!(rpc.is_err());
    }
}
