//! Error types for Xorion SDK

use thiserror::Error;

/// Main error type for Xorion SDK
#[derive(Error, Debug)]
pub enum XorionError {
    /// Invalid private key
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    /// Invalid address
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    /// RPC error
    #[error("RPC error: {0}")]
    RpcError(String),

    /// Transaction error
    #[error("Transaction error: {0}")]
    TransactionError(String),

    /// Signing error
    #[error("Signing error: {0}")]
    SigningError(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Contract error
    #[error("Contract error: {0}")]
    ContractError(String),

    /// Insufficient balance
    #[error("Insufficient balance")]
    InsufficientBalance,

    /// Timeout error
    #[error("Operation timed out")]
    Timeout,

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Unsupported chain
    #[error("Unsupported chain: {0}")]
    UnsupportedChain(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Hex decode error
    #[error("Hex decode error: {0}")]
    HexError(#[from] hex::FromHexError),

    /// URL parse error
    #[error("URL parse error: {0}")]
    UrlError(#[from] url::ParseError),
}

/// Result type alias for Xorion SDK
pub type Result<T> = std::result::Result<T, XorionError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = XorionError::InvalidPrivateKey("test".to_string());
        assert!(err.to_string().contains("Invalid private key"));

        let err = XorionError::RpcError("connection failed".to_string());
        assert!(err.to_string().contains("RPC error"));

        let err = XorionError::InsufficientBalance;
        assert_eq!(err.to_string(), "Insufficient balance");
    }

    #[test]
    fn test_result_type() {
        let result: Result<()> = Ok(());
        assert!(result.is_ok());

        let result: Result<()> = Err(XorionError::Timeout);
        assert!(result.is_err());
    }
}
