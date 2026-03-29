//! Ethereum wallet implementation

use crate::error::{XorionError, Result};
use crate::config::Network;
use crate::wallet::WalletTrait;
use async_trait::async_trait;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::{Digest, Keccak256};
use hex;
use rand::rngs::OsRng;

/// Ethereum wallet structure
pub struct EthereumWallet {
    private_key: SecretKey,
    public_key: PublicKey,
    address: String,
    network: Network,
}

impl EthereumWallet {
    /// Create a new random wallet
    pub fn new_random() -> Result<Self> {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        
        let address = Self::compute_address(&public_key)?;
        
        Ok(EthereumWallet {
            private_key: secret_key,
            public_key,
            address,
            network: Network::EthereumMainnet,
        })
    }

    /// Create wallet from private key
    pub fn from_private_key(private_key: &str, network: Network) -> Result<Self> {
        let secp = Secp256k1::new();
        
        // Remove 0x prefix if present
        let key_str = private_key.strip_prefix("0x").unwrap_or(private_key);
        
        let secret_bytes = hex::decode(key_str)
            .map_err(|e| XorionError::InvalidPrivateKey(e.to_string()))?;
        
        if secret_bytes.len() != 32 {
            return Err(XorionError::InvalidPrivateKey(
                "Private key must be 32 bytes".to_string(),
            ));
        }
        
        let mut secret_array = [0u8; 32];
        secret_array.copy_from_slice(&secret_bytes);
        
        let secret_key = SecretKey::from_slice(&secret_array)
            .map_err(|e| XorionError::InvalidPrivateKey(e.to_string()))?;
        
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let address = Self::compute_address(&public_key)?;
        
        Ok(EthereumWallet {
            private_key: secret_key,
            public_key,
            address,
            network,
        })
    }

    /// Compute Ethereum address from public key
    fn compute_address(public_key: &PublicKey) -> Result<String> {
        let pubkey_bytes = public_key.serialize_uncompressed();
        
        // Skip the first byte (0x04 prefix) and hash the rest
        let mut hasher = Keccak256::new();
        hasher.update(&pubkey_bytes[1..]);
        let hash = hasher.finalize();
        
        // Take last 20 bytes and add 0x prefix
        let address = format!("0x{}", hex::encode(&hash[12..]));
        
        Ok(address)
    }

    /// Get the private key as hex string
    pub fn private_key_hex(&self) -> String {
        format!("0x{}", hex::encode(self.private_key.secret_bytes()))
    }

    /// Get the public key as hex string
    pub fn public_key_hex(&self) -> String {
        format!("0x{}", hex::encode(self.public_key.serialize_uncompressed()))
    }

    /// Set the network for this wallet
    pub fn with_network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    /// Sign a transaction hash
    pub fn sign_hash(&self, hash: &[u8]) -> Result<Vec<u8>> {
        let secp = Secp256k1::new();
        let message = secp256k1::Message::from_digest_slice(hash)
            .map_err(|e| XorionError::SigningError(e.to_string()))?;
        
        let signature = secp.sign_ecdsa(&message, &self.private_key);
        
        Ok(signature.serialize_compact().to_vec())
    }
}

#[async_trait]
impl WalletTrait for EthereumWallet {
    fn address(&self) -> String {
        self.address.clone()
    }

    fn network(&self) -> Network {
        self.network.clone()
    }

    async fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>> {
        // Hash the message with Keccak256 (Ethereum's personal_sign prefix handling simplified)
        let mut hasher = Keccak256::new();
        hasher.update(message);
        let hash = hasher.finalize();
        
        self.sign_hash(&hash)
    }

    fn export_private_key(&self) -> Result<String> {
        Ok(self.private_key_hex())
    }

    fn public_key(&self) -> Vec<u8> {
        self.public_key.serialize_uncompressed().to_vec()
    }
}

impl Clone for EthereumWallet {
    fn clone(&self) -> Self {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.private_key);
        
        EthereumWallet {
            private_key: self.private_key,
            public_key,
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
        let wallet = EthereumWallet::new_random().unwrap();
        assert!(wallet.address.starts_with("0x"));
        assert_eq!(wallet.address.len(), 42);
    }

    #[test]
    fn test_wallet_from_private_key() {
        // Known test private key
        let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let wallet = EthereumWallet::from_private_key(
            private_key,
            Network::EthereumSepolia,
        ).unwrap();
        
        assert!(wallet.address.starts_with("0x"));
        assert!(matches!(wallet.network(), Network::EthereumSepolia));
    }

    #[test]
    fn test_invalid_private_key() {
        let result = EthereumWallet::from_private_key(
            "invalid",
            Network::EthereumMainnet,
        );
        assert!(result.is_err());

        let result = EthereumWallet::from_private_key(
            "0x1234", // Too short
            Network::EthereumMainnet,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_public_key_hex() {
        let wallet = EthereumWallet::new_random().unwrap();
        let pub_key = wallet.public_key_hex();
        assert!(pub_key.starts_with("0x"));
        assert_eq!(pub_key.len(), 130); // 0x + 65 bytes * 2
    }
}
