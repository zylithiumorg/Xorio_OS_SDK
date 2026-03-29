//! Ethereum signer implementation for Xorion Web3 OS SDK

use crate::error::{XorionError, XorionResult};
use secp256k1::{Message, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

/// Ethereum signer for signing transactions and messages
pub struct EthSigner {
    private_key: SecretKey,
    address: [u8; 20],
}

impl EthSigner {
    /// Create a new Ethereum signer from a private key
    pub fn new(private_key_bytes: [u8; 32]) -> XorionResult<Self> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&private_key_bytes)
            .map_err(|e| XorionError::SigningError(format!("Invalid private key: {}", e)))?;

        // Derive address from public key
        let public_key = secret_key.public_key(&secp);
        let public_key_bytes = public_key.serialize_uncompressed();
        
        // Hash the public key (skip first byte which is 0x04 for uncompressed)
        let mut hasher = Keccak256::new();
        hasher.update(&public_key_bytes[1..]);
        let hash = hasher.finalize();
        
        // Take last 20 bytes as address
        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[12..32]);

        Ok(Self {
            private_key: secret_key,
            address,
        })
    }

    /// Get the Ethereum address
    pub fn address(&self) -> [u8; 20] {
        self.address
    }

    /// Sign a message hash
    pub fn sign_hash(&self, hash: &[u8; 32]) -> XorionResult<[u8; 65]> {
        let secp = Secp256k1::new();
        let message = Message::from_digest(*hash);
        
        let signature = secp.sign_ecdsa_recoverable(&message, &self.private_key);
        let (recovery_id, sig) = signature.serialize_compact();
        
        let mut result = [0u8; 65];
        result[..64].copy_from_slice(&sig);
        result[64] = recovery_id.to_i32() as u8;
        
        Ok(result)
    }

    /// Sign an arbitrary message (prepends with Ethereum prefix)
    pub fn sign_message(&self, message: &[u8]) -> XorionResult<[u8; 65]> {
        // Prepend with Ethereum signed message prefix
        let prefix = b"\x19Ethereum Signed Message:\n";
        let mut msg = Vec::new();
        msg.extend_from_slice(prefix);
        msg.extend_from_slice(message.len().to_string().as_bytes());
        msg.extend_from_slice(message);
        
        let mut hasher = Keccak256::new();
        hasher.update(&msg);
        let hash: [u8; 32] = hasher.finalize().into();
        
        self.sign_hash(&hash)
    }

    /// Sign a transaction payload
    pub fn sign_transaction(&self, rlp_encoded: &[u8]) -> XorionResult<[u8; 65]> {
        let mut hasher = Keccak256::new();
        hasher.update(rlp_encoded);
        let hash: [u8; 32] = hasher.finalize().into();
        
        self.sign_hash(&hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signer_creation() {
        let private_key = [1u8; 32];
        let signer = EthSigner::new(private_key).unwrap();
        assert_eq!(signer.address().len(), 20);
    }

    #[test]
    fn test_sign_message() {
        let private_key = [1u8; 32];
        let signer = EthSigner::new(private_key).unwrap();
        let message = b"Hello, Xorion!";
        let signature = signer.sign_message(message).unwrap();
        assert_eq!(signature.len(), 65);
    }
}
