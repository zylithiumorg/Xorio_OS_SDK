//! Signing module for Xorion SDK

pub mod eth_signer;
pub mod sol_signer;

pub use eth_signer::EthereumMessageSigner;
pub use sol_signer::SolanaMessageSigner;

/// Generic message signer trait
pub trait MessageSigner: Send + Sync {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, crate::error::XorionError>;
    fn verify(&self, message: &[u8], signature: &[u8], address: &str) -> Result<bool, crate::error::XorionError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_object() {
        // Just verify the trait can be used as trait object
        let _signers: Vec<Box<dyn MessageSigner>> = Vec::new();
    }
}
