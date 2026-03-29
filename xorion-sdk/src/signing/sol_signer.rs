//! Solana signer implementation for Xorion Web3 OS SDK

use crate::error::{XorionError, XorionResult};
use ed25519_dalek::{Signer, SigningKey};

/// Solana signer for signing transactions and messages
pub struct SolSigner {
    private_key: SigningKey,
    pubkey: [u8; 32],
}

impl SolSigner {
    /// Create a new Solana signer from a private key
    pub fn new(private_key_bytes: [u8; 64]) -> XorionResult<Self> {
        let secret_key = SigningKey::from_bytes(&private_key_bytes);
        let pubkey = secret_key.verifying_key().to_bytes();

        Ok(Self {
            private_key: secret_key,
            pubkey,
        })
    }

    /// Create a Solana signer from a seed phrase (simplified)
    pub fn from_seed(seed: &[u8]) -> XorionResult<Self> {
        // In production, you'd derive the key properly from the seed
        // This is a simplified version for demonstration
        use sha2::{Digest, Sha256};
        
        let mut hasher = Sha256::new();
        hasher.update(seed);
        let hash = hasher.finalize();
        
        let mut key_bytes = [0u8; 64];
        // For simplicity, we'll use the hash twice (in real impl, use proper derivation)
        key_bytes[..32].copy_from_slice(&hash);
        key_bytes[32..].copy_from_slice(&hash);
        
        Self::new(key_bytes)
    }

    /// Get the Solana public key
    pub fn pubkey(&self) -> [u8; 32] {
        self.pubkey
    }

    /// Sign a message
    pub fn sign_message(&self, message: &[u8]) -> XorionResult<[u8; 64]> {
        let signature = self.private_key.sign(message);
        Ok(signature.to_bytes())
    }

    /// Sign a transaction (serialized bytes)
    pub fn sign_transaction(&self, transaction_data: &[u8]) -> XorionResult<[u8; 64]> {
        self.sign_message(transaction_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signer_creation() {
        let private_key = [1u8; 64];
        let signer = SolSigner::new(private_key).unwrap();
        assert_eq!(signer.pubkey().len(), 32);
    }

    #[test]
    fn test_sign_message() {
        let private_key = [1u8; 64];
        let signer = SolSigner::new(private_key).unwrap();
        let message = b"Hello, Xorion!";
        let signature = signer.sign_message(message).unwrap();
        assert_eq!(signature.len(), 64);
    }
}
