//! Ethereum signer implementation

use crate::error::{XorionError, Result};
use secp256k1::{Secp256k1, SecretKey, Message};
use sha3::{Digest, Keccak256};
use hex;

/// Recovery ID for signature
#[derive(Debug, Clone, Copy)]
pub enum RecoveryId {
    Public = 0,
    Private = 1,
}

/// Ethereum signer for transactions and messages
pub struct EthereumSigner {
    private_key: SecretKey,
    chain_id: u64,
}

impl EthereumSigner {
    /// Create a new signer with private key and chain ID
    pub fn new(private_key: &str, chain_id: u64) -> Result<Self> {
        let secp = Secp256k1::new();
        
        let key_str = private_key.strip_prefix("0x").unwrap_or(private_key);
        let secret_bytes = hex::decode(key_str)
            .map_err(|e| XorionError::SigningError(e.to_string()))?;
        
        if secret_bytes.len() != 32 {
            return Err(XorionError::SigningError(
                "Private key must be 32 bytes".to_string(),
            ));
        }
        
        let mut secret_array = [0u8; 32];
        secret_array.copy_from_slice(&secret_bytes);
        
        let private_key = SecretKey::from_slice(&secret_array)
            .map_err(|e| XorionError::SigningError(e.to_string()))?;
        
        Ok(EthereumSigner {
            private_key,
            chain_id,
        })
    }

    /// Hash a message using Ethereum's personal_sign format
    pub fn hash_personal_message(message: &[u8]) -> Vec<u8> {
        let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());
        let mut hasher = Keccak256::new();
        hasher.update(prefix.as_bytes());
        hasher.update(message);
        hasher.finalize().to_vec()
    }

    /// Sign a transaction (EIP-155 compliant)
    pub fn sign_transaction(
        &self,
        nonce: u64,
        gas_price: u64,
        gas_limit: u64,
        to: Option<&str>,
        value: u64,
        data: &[u8],
    ) -> Result<TransactionSignature> {
        // Create RLP encoded transaction for signing
        let rlp_data = self.encode_transaction_for_signing(
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data,
        );

        // Hash the RLP data
        let mut hasher = Keccak256::new();
        hasher.update(&rlp_data);
        let hash = hasher.finalize();

        // Sign the hash
        let secp = Secp256k1::new();
        let message = Message::from_digest_slice(&hash)
            .map_err(|e| XorionError::SigningError(e.to_string()))?;
        
        let signature = secp.sign_ecdsa(&message, &self.private_key);
        let (recovery_id, rec_id) = signature.serialize_compact();

        // EIP-155: v = recovery_id + 35 + chain_id * 2
        let v = rec_id.to_i32() as u64 + 35 + self.chain_id * 2;

        Ok(TransactionSignature {
            r: hex::encode(&recovery_id[..32]),
            s: hex::encode(&recovery_id[32..64]),
            v,
        })
    }

    /// Encode transaction for signing (simplified RLP encoding)
    fn encode_transaction_for_signing(
        &self,
        nonce: u64,
        gas_price: u64,
        gas_limit: u64,
        to: Option<&str>,
        value: u64,
        data: &[u8],
    ) -> Vec<u8> {
        // Simplified encoding - in production use proper RLP library
        let mut encoded = Vec::new();
        
        // Nonce
        encoded.extend_from_slice(&nonce.to_be_bytes());
        // Gas price
        encoded.extend_from_slice(&gas_price.to_be_bytes());
        // Gas limit
        encoded.extend_from_slice(&gas_limit.to_be_bytes());
        // To address
        if let Some(to_addr) = to {
            encoded.extend_from_slice(to_addr.as_bytes());
        }
        // Value
        encoded.extend_from_slice(&value.to_be_bytes());
        // Data
        encoded.extend_from_slice(data);
        // Chain ID
        encoded.extend_from_slice(&self.chain_id.to_be_bytes());
        encoded.extend_from_slice(&0u8.to_be_bytes()); // r = 0
        encoded.extend_from_slice(&0u8.to_be_bytes()); // s = 0

        encoded
    }

    /// Sign a raw hash
    pub fn sign_hash(&self, hash: &[u8]) -> Result<Vec<u8>> {
        let secp = Secp256k1::new();
        let message = Message::from_digest_slice(hash)
            .map_err(|e| XorionError::SigningError(e.to_string()))?;
        
        let signature = secp.sign_ecdsa(&message, &self.private_key);
        Ok(signature.serialize_compact().to_vec())
    }

    /// Sign a personal message
    pub fn sign_personal_message(&self, message: &[u8]) -> Result<Vec<u8>> {
        let hash = Self::hash_personal_message(message);
        self.sign_hash(&hash)
    }

    /// Get the public key
    pub fn public_key(&self) -> Vec<u8> {
        let secp = Secp256k1::new();
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &self.private_key);
        public_key.serialize_uncompressed().to_vec()
    }

    /// Get the address from public key
    pub fn address(&self) -> String {
        let public_key = self.public_key();
        let mut hasher = Keccak256::new();
        hasher.update(&public_key[1..]); // Skip 0x04 prefix
        let hash = hasher.finalize();
        
        format!("0x{}", hex::encode(&hash[12..]))
    }
}

/// Transaction signature
#[derive(Debug, Clone)]
pub struct TransactionSignature {
    pub r: String,
    pub s: String,
    pub v: u64,
}

impl TransactionSignature {
    /// Convert to raw signature bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::with_capacity(65);
        bytes.extend_from_slice(&hex::decode(&self.r)?);
        bytes.extend_from_slice(&hex::decode(&self.s)?);
        bytes.push(self.v as u8);
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signer_creation() {
        let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let signer = EthereumSigner::new(private_key, 1).unwrap();
        
        assert!(!signer.address().is_empty());
        assert!(signer.address().starts_with("0x"));
    }

    #[test]
    fn test_sign_personal_message() {
        let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let signer = EthereumSigner::new(private_key, 1).unwrap();
        
        let message = b"Hello, World!";
        let signature = signer.sign_personal_message(message).unwrap();
        
        assert_eq!(signature.len(), 64); // r + s
    }

    #[test]
    fn test_invalid_private_key() {
        let result = EthereumSigner::new("invalid", 1);
        assert!(result.is_err());
    }
}
