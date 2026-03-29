//! Contract interaction module

use crate::error::{XorionError, Result};
use crate::contract::abi::{ContractAbi, Function, StateMutability};
use serde_json::Value;

/// Smart contract instance
pub struct Contract {
    pub address: String,
    pub abi: ContractAbi,
    pub rpc_url: String,
}

impl Contract {
    /// Create a new contract instance
    pub fn new(address: String, abi: ContractAbi, rpc_url: String) -> Self {
        Contract { address, abi, rpc_url }
    }

    /// Create from ABI JSON string
    pub fn from_abi_json(address: String, abi_json: &str, rpc_url: String) -> Result<Self> {
        let abi = ContractAbi::from_json(abi_json)?;
        Ok(Self::new(address, abi, rpc_url))
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.abi.get_function(name)
    }

    /// Encode function call data
    pub fn encode_call(&self, function_name: &str, args: &[Value]) -> Result<String> {
        let function = self.get_function(function_name)
            .ok_or_else(|| XorionError::ContractError(format!("Function {} not found", function_name)))?;
        
        // Simplified encoding - in production use proper ABI encoder
        let mut encoded = format!("0x{}", function_selector(function));
        for arg in args {
            encoded.push_str(&encode_argument(arg)?);
        }
        
        Ok(encoded)
    }

    /// Check if function is view/pure (read-only)
    pub fn is_read_only(&self, function_name: &str) -> bool {
        self.get_function(function_name)
            .map(|f| matches!(f.state_mutability, StateMutability::View | StateMutability::Pure))
            .unwrap_or(false)
    }

    /// Get all function names
    pub fn function_names(&self) -> Vec<String> {
        self.abi.functions.iter().map(|f| f.name.clone()).collect()
    }

    /// Get all event names
    pub fn event_names(&self) -> Vec<String> {
        self.abi.events.iter().map(|e| e.name.clone()).collect()
    }
}

/// Compute function selector (first 4 bytes of keccak256 hash)
fn function_selector(function: &Function) -> String {
    let signature = format!(
        "{}({})",
        function.name,
        function.inputs.iter().map(|i| i.param_type.clone()).collect::<Vec<_>>().join(",")
    );
    
    // Simplified - in production use keccak256
    format!("{:08x}", signature.len() as u32)
}

/// Encode argument for transaction
fn encode_argument(arg: &Value) -> Result<String> {
    match arg {
        Value::String(s) => Ok(pad_hex(s)),
        Value::Number(n) => Ok(format!("{:064x}", n.as_u64().unwrap_or(0))),
        Value::Bool(b) => Ok(if *b { 
            format!("{:064x}", 1) 
        } else { 
            format!("{:064x}", 0) 
        }),
        _ => Err(XorionError::ContractError("Unsupported argument type".to_string())),
    }
}

/// Pad hex string to 32 bytes
fn pad_hex(s: &str) -> String {
    let stripped = s.strip_prefix("0x").unwrap_or(s);
    format!("{:064}", stripped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_creation() {
        let abi_json = r#"[{
            "type": "function",
            "name": "balanceOf",
            "inputs": [{"name": "account", "type": "address"}],
            "outputs": [{"name": "", "type": "uint256"}],
            "stateMutability": "view"
        }]"#;

        let contract = Contract::from_abi_json(
            "0x1234".to_string(),
            abi_json,
            "https://rpc.example.com".to_string(),
        ).unwrap();

        assert_eq!(contract.address, "0x1234");
        assert!(contract.is_read_only("balanceOf"));
    }

    #[test]
    fn test_function_names() {
        let abi_json = r#"[
            {"type": "function", "name": "transfer", "inputs": [], "outputs": [], "stateMutability": "nonpayable"},
            {"type": "function", "name": "approve", "inputs": [], "outputs": [], "stateMutability": "nonpayable"}
        ]"#;

        let contract = Contract::from_abi_json(
            "0x1234".to_string(),
            abi_json,
            "https://rpc.example.com".to_string(),
        ).unwrap();

        let names = contract.function_names();
        assert!(names.contains(&"transfer".to_string()));
        assert!(names.contains(&"approve".to_string()));
    }
}
