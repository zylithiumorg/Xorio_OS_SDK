//! Function encoder for contract calls

use crate::error::{XorionError, Result};
use sha3::{Digest, Keccak256};
use hex;

/// Function encoder for Ethereum contract calls
pub struct FunctionEncoder;

impl FunctionEncoder {
    /// Encode function call with arguments
    pub fn encode(function_name: &str, param_types: &[&str], args: &[Value]) -> Result<String> {
        let selector = Self::function_selector(function_name, param_types);
        let encoded_args = Self::encode_arguments(param_types, args)?;
        
        Ok(format!("0x{}{}", selector, encoded_args))
    }

    /// Compute 4-byte function selector
    pub fn function_selector(function_name: &str, param_types: &[&str]) -> String {
        let signature = format!(
            "{}({})",
            function_name,
            param_types.join(",")
        );
        
        let mut hasher = Keccak256::new();
        hasher.update(signature.as_bytes());
        let hash = hasher.finalize();
        
        hex::encode(&hash[..4])
    }

    /// Encode arguments according to types
    fn encode_arguments(param_types: &[&str], args: &[Value]) -> Result<String> {
        if param_types.len() != args.len() {
            return Err(XorionError::ContractError(
                "Number of types and arguments don't match".to_string(),
            ));
        }

        let mut encoded = String::new();
        
        for (param_type, arg) in param_types.iter().zip(args.iter()) {
            encoded.push_str(&Self::encode_single(param_type, arg)?);
        }
        
        Ok(encoded)
    }

    /// Encode a single argument
    fn encode_single(param_type: &str, arg: &Value) -> Result<String> {
        match *param_type {
            "address" => Self::encode_address(arg),
            t if t.starts_with("uint") => Self::encode_uint(t, arg),
            t if t.starts_with("int") => Self::encode_int(t, arg),
            "bool" => Self::encode_bool(arg),
            "string" => Self::encode_string(arg),
            "bytes" => Self::encode_bytes(arg),
            t if t.starts_with("bytes") => Self::encode_fixed_bytes(t, arg),
            _ => Err(XorionError::ContractError(format!(
                "Unsupported type: {}",
                param_type
            ))),
        }
    }

    /// Encode address type
    fn encode_address(arg: &Value) -> Result<String> {
        let s = arg.as_str().ok_or_else(|| {
            XorionError::ContractError("Address must be a string".to_string())
        })?;
        
        let stripped = s.strip_prefix("0x").unwrap_or(s);
        if stripped.len() != 40 {
            return Err(XorionError::ContractError("Invalid address length".to_string()));
        }
        
        Ok(format!("{:0>64}", stripped))
    }

    /// Encode uint type
    fn encode_uint(param_type: &str, arg: &Value) -> Result<String> {
        let bits: u32 = param_type[4..].parse().unwrap_or(256);
        if bits % 8 != 0 || bits > 256 {
            return Err(XorionError::ContractError(format!(
                "Invalid uint bits: {}",
                bits
            )));
        }

        let num = arg.as_u64().ok_or_else(|| {
            XorionError::ContractError("Uint must be a number".to_string())
        })?;
        
        Ok(format!("{:0>64x}", num))
    }

    /// Encode int type
    fn encode_int(_param_type: &str, arg: &Value) -> Result<String> {
        // Simplified - doesn't handle negative numbers properly
        let num = arg.as_i64().ok_or_else(|| {
            XorionError::ContractError("Int must be a number".to_string())
        })?;
        
        if num < 0 {
            // Two's complement for negative numbers
            Ok("f".repeat(64))
        } else {
            Ok(format!("{:0>64x}", num as u64))
        }
    }

    /// Encode bool type
    fn encode_bool(arg: &Value) -> Result<String> {
        let b = arg.as_bool().ok_or_else(|| {
            XorionError::ContractError("Bool must be a boolean".to_string())
        })?;
        
        Ok(if b {
            format!("{:0>64x}", 1)
        } else {
            format!("{:0>64x}", 0)
        })
    }

    /// Encode string type
    fn encode_string(arg: &Value) -> Result<String> {
        let s = arg.as_str().ok_or_else(|| {
            XorionError::ContractError("String must be a string".to_string())
        })?;
        
        let bytes = s.as_bytes();
        let len = format!("{:0>64x}", bytes.len());
        let padded_len = ((bytes.len() + 31) / 32) * 32;
        let hex_data = hex::encode(bytes);
        let padded_data = format!("{:0<width$}", hex_data, width = padded_len * 2);
        
        Ok(format!("{}{}", len, padded_data))
    }

    /// Encode bytes type
    fn encode_bytes(arg: &Value) -> Result<String> {
        let s = arg.as_str().ok_or_else(|| {
            XorionError::ContractError("Bytes must be a string".to_string())
        })?;
        
        let stripped = s.strip_prefix("0x").unwrap_or(s);
        let bytes = hex::decode(stripped).map_err(|e| {
            XorionError::ContractError(format!("Invalid hex: {}", e))
        })?;
        
        let len = format!("{:0>64x}", bytes.len());
        let padded_len = ((bytes.len() + 31) / 32) * 32;
        let padded_data = format!("{:0<width$}", stripped, width = padded_len * 2);
        
        Ok(format!("{}{}", len, padded_data))
    }

    /// Encode fixed-size bytes
    fn encode_fixed_bytes(param_type: &str, arg: &Value) -> Result<String> {
        let size: usize = param_type[5..].parse().map_err(|_| {
            XorionError::ContractError(format!("Invalid bytes size: {}", param_type))
        })?;
        
        let s = arg.as_str().ok_or_else(|| {
            XorionError::ContractError("Bytes must be a string".to_string())
        })?;
        
        let stripped = s.strip_prefix("0x").unwrap_or(s);
        if stripped.len() != size * 2 {
            return Err(XorionError::ContractError(format!(
                "Expected {} bytes, got {}",
                size,
                stripped.len() / 2
            )));
        }
        
        Ok(format!("{:0<width$}", stripped, width = 64))
    }
}

use serde_json::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_selector() {
        let selector = FunctionEncoder::function_selector(
            "transfer",
            &["address", "uint256"]
        );
        
        assert_eq!(selector.len(), 8);
    }

    #[test]
    fn test_encode_address() {
        let addr = Value::String("0x1234567890123456789012345678901234567890".to_string());
        let encoded = FunctionEncoder::encode_single("address", &addr).unwrap();
        
        assert_eq!(encoded.len(), 64);
        assert!(encoded.ends_with("1234567890123456789012345678901234567890"));
    }

    #[test]
    fn test_encode_uint() {
        let num = Value::Number(serde_json::Number::from(1000u64));
        let encoded = FunctionEncoder::encode_single("uint256", &num).unwrap();
        
        assert_eq!(encoded, "00000000000000000000000000000000000000000000000000000000000003e8");
    }

    #[test]
    fn test_encode_bool() {
        let true_val = Value::Bool(true);
        let false_val = Value::Bool(false);
        
        assert_eq!(
            FunctionEncoder::encode_single("bool", &true_val).unwrap(),
            "0000000000000000000000000000000000000000000000000000000000000001"
        );
        assert_eq!(
            FunctionEncoder::encode_single("bool", &false_val).unwrap(),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
    }
}
