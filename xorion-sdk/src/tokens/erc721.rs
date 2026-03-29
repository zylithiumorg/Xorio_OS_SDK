//! ERC721 (NFT) token implementation

use crate::error::Result;
use crate::contract::Contract;
use crate::contract::abi::ContractAbi;
use serde::{Deserialize, Serialize};

/// ERC721 NFT Token handler
pub struct ERC721Token {
    contract: Contract,
    pub address: String,
    pub name: String,
    pub symbol: String,
}

impl ERC721Token {
    /// Create a new ERC721 token instance
    pub fn new(
        address: String,
        name: String,
        symbol: String,
        rpc_url: String,
    ) -> Self {
        // Minimal ERC721 ABI
        let abi_json = r#"[
            {"type": "function", "name": "balanceOf", "inputs": [{"name": "owner", "type": "address"}], "outputs": [{"name": "", "type": "uint256"}], "stateMutability": "view"},
            {"type": "function", "name": "ownerOf", "inputs": [{"name": "tokenId", "type": "uint256"}], "outputs": [{"name": "", "type": "address"}], "stateMutability": "view"},
            {"type": "function", "name": "safeTransferFrom", "inputs": [{"name": "from", "type": "address"}, {"name": "to", "type": "address"}, {"name": "tokenId", "type": "uint256"}], "outputs": [], "stateMutability": "nonpayable"},
            {"type": "function", "name": "transferFrom", "inputs": [{"name": "from", "type": "address"}, {"name": "to", "type": "address"}, {"name": "tokenId", "type": "uint256"}], "outputs": [], "stateMutability": "nonpayable"},
            {"type": "function", "name": "approve", "inputs": [{"name": "to", "type": "address"}, {"name": "tokenId", "type": "uint256"}], "outputs": [], "stateMutability": "nonpayable"},
            {"type": "function", "name": "getApproved", "inputs": [{"name": "tokenId", "type": "uint256"}], "outputs": [{"name": "", "type": "address"}], "stateMutability": "view"},
            {"type": "function", "name": "setApprovalForAll", "inputs": [{"name": "operator", "type": "address"}, {"name": "approved", "type": "bool"}], "outputs": [], "stateMutability": "nonpayable"},
            {"type": "function", "name": "isApprovedForAll", "inputs": [{"name": "owner", "type": "address"}, {"name": "operator", "type": "address"}], "outputs": [{"name": "", "type": "bool"}], "stateMutability": "view"},
            {"type": "function", "name": "tokenURI", "inputs": [{"name": "tokenId", "type": "uint256"}], "outputs": [{"name": "", "type": "string"}], "stateMutability": "view"},
            {"type": "event", "name": "Transfer", "inputs": [{"name": "from", "type": "address", "indexed": true}, {"name": "to", "type": "address", "indexed": true}, {"name": "tokenId", "type": "uint256", "indexed": true}], "anonymous": false},
            {"type": "event", "name": "Approval", "inputs": [{"name": "owner", "type": "address", "indexed": true}, {"name": "approved", "type": "address", "indexed": true}, {"name": "tokenId", "type": "uint256", "indexed": true}], "anonymous": false},
            {"type": "event", "name": "ApprovalForAll", "inputs": [{"name": "owner", "type": "address", "indexed": true}, {"name": "operator", "type": "address", "indexed": true}, {"name": "approved", "type": "bool", "indexed": false}], "anonymous": false}
        ]"#;

        let abi = ContractAbi::from_json(abi_json).unwrap();
        let contract = Contract::new(address.clone(), abi, rpc_url);

        ERC721Token {
            contract,
            address,
            name,
            symbol,
        }
    }

    /// Encode transferFrom transaction data
    pub fn encode_transfer_from(&self, from: &str, to: &str, token_id: u64) -> Result<String> {
        self.contract.encode_call(
            "transferFrom",
            &[
                serde_json::json!(from),
                serde_json::json!(to),
                serde_json::json!(token_id),
            ],
        )
    }

    /// Encode safeTransferFrom transaction data
    pub fn encode_safe_transfer_from(&self, from: &str, to: &str, token_id: u64) -> Result<String> {
        self.contract.encode_call(
            "safeTransferFrom",
            &[
                serde_json::json!(from),
                serde_json::json!(to),
                serde_json::json!(token_id),
            ],
        )
    }

    /// Encode approve transaction data
    pub fn encode_approve(&self, to: &str, token_id: u64) -> Result<String> {
        self.contract.encode_call(
            "approve",
            &[
                serde_json::json!(to),
                serde_json::json!(token_id),
            ],
        )
    }

    /// Encode setApprovalForAll transaction data
    pub fn encode_set_approval_for_all(&self, operator: &str, approved: bool) -> Result<String> {
        self.contract.encode_call(
            "setApprovalForAll",
            &[
                serde_json::json!(operator),
                serde_json::json!(approved),
            ],
        )
    }
}

/// NFT metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMetadata {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub external_url: Option<String>,
    pub attributes: Vec<NftAttribute>,
}

/// NFT attribute/trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftAttribute {
    pub trait_type: String,
    pub value: serde_json::Value,
}

impl NftMetadata {
    /// Parse metadata from JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| crate::error::XorionError::SerializationError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erc721_creation() {
        let nft = ERC721Token::new(
            "0x1234567890123456789012345678901234567890".to_string(),
            "Test NFT".to_string(),
            "TNFT".to_string(),
            "https://rpc.example.com".to_string(),
        );

        assert_eq!(nft.name, "Test NFT");
        assert_eq!(nft.symbol, "TNFT");
    }

    #[test]
    fn test_nft_metadata() {
        let json = r#"{
            "name": "Cool NFT #1",
            "description": "A cool NFT",
            "image": "ipfs://QmTest",
            "attributes": [
                {"trait_type": "Background", "value": "Blue"},
                {"trait_type": "Rarity", "value": "Legendary"}
            ]
        }"#;

        let metadata = NftMetadata::from_json(json).unwrap();
        assert_eq!(metadata.name, "Cool NFT #1");
        assert_eq!(metadata.attributes.len(), 2);
    }
}
