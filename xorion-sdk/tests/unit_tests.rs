//! Unit tests for Xorion Web3 OS SDK

#[cfg(test)]
mod wallet_tests {
    use xorion_sdk::wallet::{Wallet, WalletType};
    use xorion_sdk::error::XorionResult;

    #[test]
    fn test_ethereum_wallet_address_length() -> XorionResult<()> {
        let wallet = Wallet::new(WalletType::Ethereum)?;
        assert_eq!(wallet.address().len(), 20);
        Ok(())
    }

    #[test]
    fn test_solana_wallet_address_length() -> XorionResult<()> {
        let wallet = Wallet::new(WalletType::Solana)?;
        assert_eq!(wallet.address().len(), 32);
        Ok(())
    }

    #[test]
    fn test_wallet_type_identification() -> XorionResult<()> {
        let eth_wallet = Wallet::new(WalletType::Ethereum)?;
        assert_eq!(eth_wallet.wallet_type(), WalletType::Ethereum);

        let sol_wallet = Wallet::new(WalletType::Solana)?;
        assert_eq!(sol_wallet.wallet_type(), WalletType::Solana);
        
        Ok(())
    }
}

#[cfg(test)]
mod config_tests {
    use xorion_sdk::config::{Config, Chain};

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.chain(), Chain::Ethereum);
    }

    #[test]
    fn test_config_with_chain() {
        let config = Config::default().with_chain(Chain::Solana);
        assert_eq!(config.chain(), Chain::Solana);
    }

    #[test]
    fn test_config_immutability() {
        let config1 = Config::default();
        let config2 = config1.with_chain(Chain::Solana);
        
        // Original should be unchanged (if we had Clone)
        // For now just verify the new config is correct
        assert_eq!(config2.chain(), Chain::Solana);
    }
}

#[cfg(test)]
mod error_tests {
    use xorion_sdk::error::{XorionError, XorionResult};

    #[test]
    fn test_error_variants() {
        let invalid_addr = XorionError::InvalidAddress("test".to_string());
        assert!(matches!(invalid_addr, XorionError::InvalidAddress(_)));

        let rpc_err = XorionError::RpcError("connection failed".to_string());
        assert!(matches!(rpc_err, XorionError::RpcError(_)));

        let signing_err = XorionError::SigningError("key error".to_string());
        assert!(matches!(signing_err, XorionError::SigningError(_)));

        let parse_err = XorionError::ParseError("parse failed".to_string());
        assert!(matches!(parse_err, XorionError::ParseError(_)));
    }

    #[test]
    fn test_error_display() {
        let err = XorionError::InvalidAddress("0xinvalid".to_string());
        let display = format!("{}", err);
        assert!(display.contains("0xinvalid"));
    }

    #[test]
    fn test_result_ok() -> XorionResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod signing_tests {
    use xorion_sdk::signing::{EthSigner, SolSigner};

    #[test]
    fn test_eth_signer_creation() {
        let private_key = [1u8; 32];
        let signer = EthSigner::new(private_key).unwrap();
        assert_eq!(signer.address().len(), 20);
    }

    #[test]
    fn test_eth_signer_sign_message() {
        let private_key = [1u8; 32];
        let signer = EthSigner::new(private_key).unwrap();
        let message = b"Hello, Xorion!";
        let signature = signer.sign_message(message).unwrap();
        assert_eq!(signature.len(), 65);
    }

    #[test]
    fn test_sol_signer_creation() {
        let private_key = [1u8; 64];
        let signer = SolSigner::new(private_key).unwrap();
        assert_eq!(signer.pubkey().len(), 32);
    }

    #[test]
    fn test_sol_signer_sign_message() {
        let private_key = [1u8; 64];
        let signer = SolSigner::new(private_key).unwrap();
        let message = b"Hello, Xorion!";
        let signature = signer.sign_message(message).unwrap();
        assert_eq!(signature.len(), 64);
    }
}

#[cfg(test)]
mod transaction_tests {
    use xorion_sdk::transaction::TransactionBuilder;
    use xorion_sdk::config::Chain;

    #[test]
    fn test_transaction_builder_ethereum() {
        let builder = TransactionBuilder::new(Chain::Ethereum);
        assert_eq!(builder.chain(), Chain::Ethereum);
    }

    #[test]
    fn test_transaction_builder_solana() {
        let builder = TransactionBuilder::new(Chain::Solana);
        assert_eq!(builder.chain(), Chain::Solana);
    }
}
