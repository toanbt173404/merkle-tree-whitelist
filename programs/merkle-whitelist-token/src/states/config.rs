use anchor_lang::prelude::*;

pub const CONFIG_SEED: &str = "config";
#[account]
pub struct ConfigAccount {
    pub bump: u8,
    pub is_initialized: bool,
    pub admin: Pubkey,
    pub merkle_index: u64,

}

impl ConfigAccount {
    pub const LEN: usize = 8 // Account discriminator added by Anchor for each account
            + 1 // bump
            + 1 //is_initialized
            + 32 //admin
            + 32 //reward mint
            + 8; //merkle_index

}
