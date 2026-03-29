//! Ethereum module for Xorion SDK

pub mod wallet;
pub mod rpc;
pub mod signer;

pub use wallet::EthereumWallet;
pub use rpc::EthereumRpc;
pub use signer::EthereumSigner;

use crate::config::Network;

/// Get the appropriate network for Ethereum operations
pub fn get_ethereum_network(rpc_url: Option<&str>) -> Network {
    if let Some(url) = rpc_url {
        if url.contains("sepolia") {
            Network::EthereumSepolia
        } else if url.contains("goerli") {
            Network::EthereumGoerli
        } else {
            Network::Custom(url.to_string())
        }
    } else {
        Network::EthereumMainnet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ethereum_network() {
        assert!(matches!(get_ethereum_network(None), Network::EthereumMainnet));
        
        let sepolia = get_ethereum_network(Some("https://sepolia.infura.io"));
        assert!(matches!(sepolia, Network::EthereumSepolia));
        
        let custom = get_ethereum_network(Some("https://custom.rpc.com"));
        assert!(matches!(custom, Network::Custom(_)));
    }
}
