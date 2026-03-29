//! Tokens module for Xorion SDK

pub mod erc20;
pub mod erc721;
pub mod token_utils;

pub use erc20::ERC20Token;
pub use erc721::ERC721Token;
pub use token_utils::TokenInfo;

/// Token standard type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenStandard {
    ERC20,
    ERC721,
    ERC1155,
    SPL, // Solana Program Library token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_standard() {
        assert_eq!(TokenStandard::ERC20 as u8, 0);
        assert_eq!(TokenStandard::ERC721 as u8, 1);
    }
}
