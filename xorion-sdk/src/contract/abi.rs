//! Contract ABI parsing and handling

use serde::{Deserialize, Serialize};
use crate::error::Result;

/// Contract ABI definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAbi {
    pub functions: Vec<Function>,
    pub events: Vec<Event>,
    pub constructor: Option<Constructor>,
}

/// Function definition in ABI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub inputs: Vec<Parameter>,
    pub outputs: Vec<Parameter>,
    pub state_mutability: StateMutability,
    pub constant: bool,
}

/// Event definition in ABI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub inputs: Vec<EventParameter>,
    pub anonymous: bool,
}

/// Constructor definition in ABI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constructor {
    pub inputs: Vec<Parameter>,
    pub payable: bool,
}

/// Function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub components: Option<Vec<Parameter>>,
    pub indexed: bool,
}

/// Event parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub indexed: bool,
    pub components: Option<Vec<Parameter>>,
}

/// State mutability of a function
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    Pure,
    View,
    Nonpayable,
    Payable,
}

impl Default for StateMutability {
    fn default() -> Self {
        StateMutability::Nonpayable
    }
}

impl ContractAbi {
    /// Parse ABI from JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        let abi_items: Vec<AbiItem> = serde_json::from_str(json)
            .map_err(|e| crate::error::XorionError::SerializationError(e.to_string()))?;
        
        let mut functions = Vec::new();
        let mut events = Vec::new();
        let mut constructor = None;

        for item in abi_items {
            match item.abi_type.as_str() {
                "function" => {
                    if let Some(func) = item.function {
                        functions.push(func);
                    }
                }
                "event" => {
                    if let Some(event) = item.event {
                        events.push(event);
                    }
                }
                "constructor" => {
                    constructor = item.constructor;
                }
                _ => {}
            }
        }

        Ok(ContractAbi {
            functions,
            events,
            constructor,
        })
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name)
    }

    /// Get an event by name
    pub fn get_event(&self, name: &str) -> Option<&Event> {
        self.events.iter().find(|e| e.name == name)
    }

    /// Check if contract has a specific function
    pub fn has_function(&self, name: &str) -> bool {
        self.get_function(name).is_some()
    }

    /// Check if contract has a specific event
    pub fn has_event(&self, name: &str) -> bool {
        self.get_event(name).is_some()
    }
}

/// Generic ABI item for deserialization
#[derive(Debug, Clone, Deserialize)]
struct AbiItem {
    #[serde(rename = "type")]
    abi_type: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    inputs: Option<Vec<Parameter>>,
    #[serde(default)]
    outputs: Option<Vec<Parameter>>,
    #[serde(default)]
    state_mutability: Option<StateMutability>,
    #[serde(default)]
    constant: Option<bool>,
    #[serde(default)]
    anonymous: Option<bool>,
    #[serde(default)]
    payable: Option<bool>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

impl AbiItem {
    fn function(&self) -> Option<Function> {
        if self.abi_type != "function" {
            return None;
        }
        
        Some(Function {
            name: self.name.clone().unwrap_or_default(),
            inputs: self.inputs.clone().unwrap_or_default(),
            outputs: self.outputs.clone().unwrap_or_default(),
            state_mutability: self.state_mutability.unwrap_or_default(),
            constant: self.constant.unwrap_or(false),
        })
    }

    fn event(&self) -> Option<Event> {
        if self.abi_type != "event" {
            return None;
        }

        let inputs = self.inputs.clone().unwrap_or_default();
        let event_inputs = inputs.into_iter().map(|p| EventParameter {
            name: p.name,
            param_type: p.param_type,
            indexed: p.indexed,
            components: p.components,
        }).collect();

        Some(Event {
            name: self.name.clone().unwrap_or_default(),
            inputs: event_inputs,
            anonymous: self.anonymous.unwrap_or(false),
        })
    }

    fn constructor(&self) -> Option<Constructor> {
        if self.abi_type != "constructor" {
            return None;
        }

        Some(Constructor {
            inputs: self.inputs.clone().unwrap_or_default(),
            payable: self.payable.unwrap_or(false),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_abi() {
        let json = r#"[
            {
                "type": "function",
                "name": "transfer",
                "inputs": [
                    {"name": "to", "type": "address"},
                    {"name": "amount", "type": "uint256"}
                ],
                "outputs": [{"name": "", "type": "bool"}],
                "stateMutability": "nonpayable"
            },
            {
                "type": "event",
                "name": "Transfer",
                "inputs": [
                    {"name": "from", "type": "address", "indexed": true},
                    {"name": "to", "type": "address", "indexed": true},
                    {"name": "value", "type": "uint256", "indexed": false}
                ],
                "anonymous": false
            }
        ]"#;

        let abi = ContractAbi::from_json(json).unwrap();
        
        assert_eq!(abi.functions.len(), 1);
        assert_eq!(abi.events.len(), 1);
        assert!(abi.has_function("transfer"));
        assert!(abi.has_event("Transfer"));
    }

    #[test]
    fn test_get_function() {
        let json = r#"[{
            "type": "function",
            "name": "balanceOf",
            "inputs": [{"name": "account", "type": "address"}],
            "outputs": [{"name": "", "type": "uint256"}],
            "stateMutability": "view"
        }]"#;

        let abi = ContractAbi::from_json(json).unwrap();
        let func = abi.get_function("balanceOf").unwrap();
        
        assert_eq!(func.name, "balanceOf");
        assert_eq!(func.inputs.len(), 1);
        assert!(matches!(func.state_mutability, StateMutability::View));
    }
}
