use anchor_lang::prelude::*;

pub const MERKLE_TOKEN_DISTRIBUTOR_SEED: &str = "merkle_token_distributor";

#[account]
#[derive(Default)]
pub struct MerkleTokenDistributor {
    // 256-bit Merkle root (used for verifying claims)
    pub root: [u8; 32],
    
    // Bump seed for the PDA (Program Derived Address)
    pub bump: u8,

    // Total number of tokens that have been minted so far
    pub total_amount_minted: u64,

    // Maximum number of tokens that can be minted (cap on minting)
    pub max_mint_amount: u64,
}

impl MerkleTokenDistributor {
    pub const LEN: usize = 8 + // Anchor's discriminator (unique identifier for the account type)
        32 + // 32 bytes for the Merkle root (array of 32 u8s)
        1 +  // 1 byte for the bump seed (u8)
        8 +  // 8 bytes for total_amount_minted (u64)
        8;   // 8 bytes for max_mint_amount (u64)
}
