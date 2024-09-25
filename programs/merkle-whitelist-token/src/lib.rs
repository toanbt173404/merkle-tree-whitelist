use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod states;

use instructions::*;
use states::*;

declare_id!("AUy9cTP7vQmcMvxq9fu2n9xQum3MdGUKTW2Ms3FpjFpE");

#[program]
pub mod merkle_whitelist_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)?;
        Ok(())
    }

    pub fn init_distributor(ctx: Context<InitDistributor>, root: [u8; 32]) -> Result<()> {
        instructions::init_distributor(ctx, root)?;
        Ok(())
    }

    
}
