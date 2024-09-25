use std::ops::DerefMut;

use anchor_lang::prelude::*;

use crate::{error::ProgramErrorCode, ConfigAccount, CONFIG_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = ConfigAccount::LEN,
        seeds = [&CONFIG_SEED.as_bytes()],
        bump
    )]
    pub config_account: Account<'info, ConfigAccount>,

    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let config_account = ctx.accounts.config_account.deref_mut();

    if config_account.is_initialized {
        return Err(ProgramErrorCode::AlreadyInitialized.into());
    }

    config_account.bump = ctx.bumps.config_account;
    config_account.is_initialized = true;
    config_account.admin = *ctx.accounts.owner.key; 


    Ok(())
}
