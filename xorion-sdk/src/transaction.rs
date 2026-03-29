//! Transaction module for Xorion SDK

use serde::{Deserialize, Serialize};
use crate::error::{XorionError, Result};

/// Transaction status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed(u64), // Block number
    Failed(String),
}

/// Generic transaction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction hash
    pub hash: String,
    /// Sender address
    pub from: String,
    /// Recipient address (optional for contract creation)
    pub to: Option<String>,
    /// Transaction value in smallest unit (wei/lamports)
    pub value: String,
    /// Gas price/fee
    pub gas_price: Option<String>,
    /// Gas limit
    pub gas_limit: Option<u64>,
    /// Nonce
    pub nonce: Option<u64>,
    /// Transaction data/payload
    pub data: Option<String>,
    /// Chain ID
    pub chain_id: Option<u64>,
    /// Signature (r, s, v for Ethereum)
    pub signature: Option<TransactionSignature>,
    /// Status
    pub status: TransactionStatus,
    /// Block number (if confirmed)
    pub block_number: Option<u64>,
    /// Timestamp
    pub timestamp: Option<u64>,
}

/// Transaction signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSignature {
    pub r: String,
    pub s: String,
    pub v: Option<u64>,
}

impl Transaction {
    /// Create a new pending transaction
    pub fn new(from: String, to: Option<String>, value: String) -> Self {
        Transaction {
            hash: String::new(),
            from,
            to,
            value,
            gas_price: None,
            gas_limit: None,
            nonce: None,
            data: None,
            chain_id: None,
            signature: None,
            status: TransactionStatus::Pending,
            block_number: None,
            timestamp: None,
        }
    }

    /// Set the transaction hash
    pub fn with_hash(mut self, hash: String) -> Self {
        self.hash = hash;
        self
    }

    /// Set gas price
    pub fn with_gas_price(mut self, gas_price: String) -> Self {
        self.gas_price = Some(gas_price);
        self
    }

    /// Set gas limit
    pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = Some(gas_limit);
        self
    }

    /// Set nonce
    pub fn with_nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }

    /// Set transaction data
    pub fn with_data(mut self, data: String) -> Self {
        self.data = Some(data);
        self
    }

    /// Set chain ID
    pub fn with_chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = Some(chain_id);
        self
    }

    /// Check if transaction is confirmed
    pub fn is_confirmed(&self) -> bool {
        matches!(self.status, TransactionStatus::Confirmed(_))
    }

    /// Check if transaction failed
    pub fn is_failed(&self) -> bool {
        matches!(self.status, TransactionStatus::Failed(_))
    }

    /// Get the confirmation block number
    pub fn block_number(&self) -> Option<u64> {
        match self.status {
            TransactionStatus::Confirmed(block) => Some(block),
            _ => self.block_number,
        }
    }

    /// Validate the transaction
    pub fn validate(&self) -> Result<()> {
        if self.from.is_empty() {
            return Err(XorionError::TransactionError(
                "Sender address is required".to_string(),
            ));
        }

        if self.value.is_empty() {
            return Err(XorionError::TransactionError(
                "Value is required".to_string(),
            ));
        }

        Ok(())
    }
}

/// Transaction builder
pub struct TransactionBuilder {
    from: String,
    to: Option<String>,
    value: String,
    gas_price: Option<String>,
    gas_limit: Option<u64>,
    nonce: Option<u64>,
    data: Option<String>,
    chain_id: Option<u64>,
}

impl TransactionBuilder {
    pub fn new(from: String) -> Self {
        TransactionBuilder {
            from,
            to: None,
            value: "0".to_string(),
            gas_price: None,
            gas_limit: None,
            nonce: None,
            data: None,
            chain_id: None,
        }
    }

    pub fn to(mut self, to: String) -> Self {
        self.to = Some(to);
        self
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    pub fn gas_price(mut self, gas_price: String) -> Self {
        self.gas_price = Some(gas_price);
        self
    }

    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = Some(gas_limit);
        self
    }

    pub fn nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }

    pub fn data(mut self, data: String) -> Self {
        self.data = Some(data);
        self
    }

    pub fn chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = Some(chain_id);
        self
    }

    pub fn build(self) -> Transaction {
        Transaction {
            hash: String::new(),
            from: self.from,
            to: self.to,
            value: self.value,
            gas_price: self.gas_price,
            gas_limit: self.gas_limit,
            nonce: self.nonce,
            data: self.data,
            chain_id: self.chain_id,
            signature: None,
            status: TransactionStatus::Pending,
            block_number: None,
            timestamp: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new(
            "0x1234".to_string(),
            Some("0x5678".to_string()),
            "1000".to_string(),
        );

        assert_eq!(tx.from, "0x1234");
        assert_eq!(tx.to, Some("0x5678".to_string()));
        assert_eq!(tx.value, "1000");
        assert!(tx.hash.is_empty());
    }

    #[test]
    fn test_transaction_builder() {
        let tx = TransactionBuilder::new("0x1234".to_string())
            .to("0x5678".to_string())
            .value("1000".to_string())
            .gas_limit(21000)
            .nonce(1)
            .chain_id(1)
            .build();

        assert_eq!(tx.from, "0x1234");
        assert_eq!(tx.gas_limit, Some(21000));
        assert_eq!(tx.nonce, Some(1));
        assert_eq!(tx.chain_id, Some(1));
    }

    #[test]
    fn test_transaction_status() {
        let mut tx = Transaction::new(
            "0x1234".to_string(),
            None,
            "0".to_string(),
        );

        assert!(matches!(tx.status, TransactionStatus::Pending));

        tx.status = TransactionStatus::Confirmed(12345);
        assert!(tx.is_confirmed());
        assert_eq!(tx.block_number(), Some(12345));

        tx.status = TransactionStatus::Failed("Out of gas".to_string());
        assert!(tx.is_failed());
    }

    #[test]
    fn test_transaction_validation() {
        let tx = Transaction::new(
            "".to_string(),
            None,
            "0".to_string(),
        );
        assert!(tx.validate().is_err());

        let tx = Transaction::new(
            "0x1234".to_string(),
            None,
            "".to_string(),
        );
        assert!(tx.validate().is_err());

        let tx = Transaction::new(
            "0x1234".to_string(),
            None,
            "0".to_string(),
        );
        assert!(tx.validate().is_ok());
    }
}
