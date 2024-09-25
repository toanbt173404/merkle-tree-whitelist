use anchor_lang::prelude::*;

use crate::{ConfigAccount, MerkleTokenDistributor, MERKLE_TOKEN_DISTRIBUTOR_SEED};

#[event]
pub struct DistributorInitialized {
    pub admin: Pubkey,
    pub root: [u8; 32],
    pub merkle_index: u64,
    pub distributor: Pubkey,
}

#[derive(Accounts)]
pub struct InitDistributor<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        seeds = [
            &MERKLE_TOKEN_DISTRIBUTOR_SEED.as_bytes(),
            config_account.merkle_index.to_le_bytes().as_ref(),
        ],
        bump,
        space = MerkleTokenDistributor::LEN,
        payer = admin,
    )]
    pub merkle_distributor: Account<'info, MerkleTokenDistributor>,
    #[account(
        mut,
        constraint = config_account.admin == admin.key()
    )]
    pub config_account: Account<'info, ConfigAccount>,

    pub system_program: Program<'info, System>,
}

pub fn init_distributor(ctx: Context<InitDistributor>, root: [u8; 32]) -> Result<()> {
    let merkle_distributor = &mut ctx.accounts.merkle_distributor;
    let config_account = &mut ctx.accounts.config_account;

    merkle_distributor.bump = ctx.bumps.merkle_distributor;
    merkle_distributor.root = root;

    config_account.merkle_index += 1;

    emit!(DistributorInitialized {
        admin: ctx.accounts.admin.key(),
        root,
        merkle_index: config_account.merkle_index,
        distributor: merkle_distributor.key(),
    });

    Ok(())
}
