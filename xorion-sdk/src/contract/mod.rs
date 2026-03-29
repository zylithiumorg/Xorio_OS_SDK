//! Smart contract module for Xorion SDK

pub mod abi;
pub mod contract;
pub mod encoder;
pub mod events;

pub use abi::ContractAbi;
pub use contract::Contract;
pub use encoder::FunctionEncoder;
pub use events::EventLog;

/// Contract call type
#[derive(Debug, Clone)]
pub enum CallType {
    /// Read-only call (no gas required)
    View,
    /// State-changing call (requires gas)
    Transaction,
}

/// Contract deployment information
#[derive(Debug, Clone)]
pub struct DeploymentInfo {
    pub transaction_hash: String,
    pub contract_address: String,
    pub block_number: Option<u64>,
    pub gas_used: Option<u64>,
}

impl DeploymentInfo {
    pub fn new(transaction_hash: String, contract_address: String) -> Self {
        DeploymentInfo {
            transaction_hash,
            contract_address,
            block_number: None,
            gas_used: None,
        }
    }

    pub fn with_block_number(mut self, block: u64) -> Self {
        self.block_number = Some(block);
        self
    }

    pub fn with_gas_used(mut self, gas: u64) -> Self {
        self.gas_used = Some(gas);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_info() {
        let info = DeploymentInfo::new(
            "0xtxhash".to_string(),
            "0xcontract".to_string(),
        );

        assert_eq!(info.transaction_hash, "0xtxhash");
        assert_eq!(info.contract_address, "0xcontract");
        assert!(info.block_number.is_none());
    }
}
