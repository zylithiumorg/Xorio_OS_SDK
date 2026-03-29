//! Solana wallet implementation

use crate::error::{XorionError, Result};
use crate::config::Network;
use crate::wallet::WalletTrait;
use async_trait::async_trait;
use bs58;
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

/// Solana wallet structure
pub struct SolanaWallet {
    secret_key: [u8; 64],
    public_key: [u8; 32],
    address: String,
    network: Network,
}

impl SolanaWallet {
    /// Create a new random wallet
    pub fn new_random() -> Result<Self> {
        // Generate random bytes for seed
        let mut seed = [0u8; 32];
        OsRng.fill_bytes(&mut seed);
        
        Self::from_seed(&seed, Network::SolanaMainnet)
    }

    /// Create wallet from seed
    pub fn from_seed(seed: &[u8], network: Network) -> Result<Self> {
        if seed.len() != 32 {
            return Err(XorionError::InvalidPrivateKey(
                "Seed must be 32 bytes".to_string(),
            ));
        }

        // Derive keypair using simplified Ed25519 derivation
        // In production, use proper Ed25519 derivation with curve25519-dalek
        let mut hasher = Sha256::new();
        hasher.update(seed);
        let hash = hasher.finalize();
        
        let mut secret_key = [0u8; 64];
        let mut public_key = [0u8; 32];
        
        // Simplified key derivation (for demo purposes)
        secret_key[..32].copy_from_slice(seed);
        public_key.copy_from_slice(&hash[..32]);
        
        let address = bs58::encode(&public_key).into_string();

        Ok(SolanaWallet {
            secret_key,
            public_key,
            address,
            network,
        })
    }

    /// Create wallet from base58 encoded private key
    pub fn from_base58(encoded_key: &str, network: Network) -> Result<Self> {
        let decoded = bs58::decode(encoded_key)
            .into_vec()
            .map_err(|e| XorionError::InvalidPrivateKey(e.to_string()))?;

        if decoded.len() < 32 {
            return Err(XorionError::InvalidPrivateKey(
                "Invalid key length".to_string(),
            ));
        }

        // Extract seed from decoded bytes
        let seed = if decoded.len() == 64 {
            // Full keypair
            &decoded[..32]
        } else if decoded.len() == 32 {
            // Just the seed
            &decoded
        } else {
            return Err(XorionError::InvalidPrivateKey(
                "Unexpected key length".to_string(),
            ));
        };

        Self::from_seed(seed, network)
    }

    /// Get the secret key as base58 string
    pub fn to_base58(&self) -> String {
        bs58::encode(&self.secret_key[..32]).into_string()
    }

    /// Get the public key as base58 string
    pub fn public_key_base58(&self) -> String {
        bs58::encode(&self.public_key).into_string()
    }

    /// Get the public key bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public_key
    }

    /// Set the network for this wallet
    pub fn with_network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    /// Sign a message
    pub fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>> {
        // Simplified signing - in production use proper Ed25519 signing
        let mut hasher = Sha256::new();
        hasher.update(&self.secret_key[..32]);
        hasher.update(message);
        let signature = hasher.finalize();
        
        Ok(signature.to_vec())
    }
}

#[async_trait]
impl WalletTrait for SolanaWallet {
    fn address(&self) -> String {
        self.address.clone()
    }

    fn network(&self) -> Network {
        self.network.clone()
    }

    async fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>> {
        self.sign_message(message)
    }

    fn export_private_key(&self) -> Result<String> {
        Ok(self.to_base58())
    }

    fn public_key(&self) -> Vec<u8> {
        self.public_key.to_vec()
    }
}

impl Clone for SolanaWallet {
    fn clone(&self) -> Self {
        SolanaWallet {
            secret_key: self.secret_key,
            public_key: self.public_key,
            address: self.address.clone(),
            network: self.network.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let wallet = SolanaWallet::new_random().unwrap();
        assert!(!wallet.address.is_empty());
        assert!(wallet.address.len() > 30); // Base58 addresses are typically 32-44 chars
    }

    #[test]
    fn test_wallet_from_seed() {
        let seed = [1u8; 32];
        let wallet = SolanaWallet::from_seed(&seed, Network::SolanaDevnet).unwrap();
        
        assert!(!wallet.address.is_empty());
        assert!(matches!(wallet.network(), Network::SolanaDevnet));
    }

    #[test]
    fn test_wallet_from_base58() {
        // Create a wallet and export it
        let original = SolanaWallet::new_random().unwrap();
        let exported = original.to_base58();
        
        let restored = SolanaWallet::from_base58(&exported, Network::SolanaMainnet).unwrap();
        assert_eq!(original.address(), restored.address());
    }

    #[test]
    fn test_invalid_seed() {
        let result = SolanaWallet::from_seed(&[1u8; 16], Network::SolanaMainnet);
        assert!(result.is_err());
    }

    #[test]
    fn test_public_key_formats() {
        let wallet = SolanaWallet::new_random().unwrap();
        
        let pk_base58 = wallet.public_key_base58();
        let pk_bytes = wallet.public_key_bytes();
        
        assert!(!pk_base58.is_empty());
        assert_eq!(pk_bytes.len(), 32);
    }
}
