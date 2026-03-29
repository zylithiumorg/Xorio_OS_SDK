//! Solana module for Xorion SDK

pub mod wallet;
pub mod rpc;

pub use wallet::SolanaWallet;
pub use rpc::SolanaRpc;

use crate::config::Network;

/// Get the appropriate network for Solana operations
pub fn get_solana_network(rpc_url: Option<&str>) -> Network {
    if let Some(url) = rpc_url {
        if url.contains("devnet") {
            Network::SolanaDevnet
        } else if url.contains("testnet") {
            Network::SolanaTestnet
        } else {
            Network::Custom(url.to_string())
        }
    } else {
        Network::SolanaMainnet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_solana_network() {
        assert!(matches!(get_solana_network(None), Network::SolanaMainnet));
        
        let devnet = get_solana_network(Some("https://api.devnet.solana.com"));
        assert!(matches!(devnet, Network::SolanaDevnet));
        
        let custom = get_solana_network(Some("https://custom.rpc.com"));
        assert!(matches!(custom, Network::Custom(_)));
    }
}
