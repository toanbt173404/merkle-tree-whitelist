use anchor_lang::prelude::*;

use crate::{ConfigAccount, MerkleTokenDistributor, MERKLE_TOKEN_DISTRIBUTOR_SEED};

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        seeds = [
            &MERKLE_TOKEN_DISTRIBUTOR_SEED.as_bytes(),
            config_account.merkle_index.to_le_bytes().as_ref(),
        ],
        bump,
        space = MerkleTokenDistributor::LEN,
        payer = payer,
    )]
    pub merkle_distributor: Account<'info, MerkleTokenDistributor>,
    #[account(
        mut,
    )]
    pub config_account: Account<'info, ConfigAccount>,

    pub system_program: Program<'info, System>,
}

pub fn mint_token(
    ctx: Context<MintToken>,
    proof: Vec<[u8; 32]>,
    index: u64,
    amount: u64,
) -> Result<()> {
    msg!("Start of token mint operation...");

    //init ctx variables
    let payer = &ctx.accounts.payer;
    let token_distributor = &mut ctx.accounts.merkle_distributor;
    //check that the owner is a Signer
    require!(ctx.accounts.payer.is_signer, MerkleError::Unauthorized);
    //a node/leaf in a merkletree
    let leaf = keccak::hashv(&[
        &payer.key().to_bytes(),
        &index.to_le_bytes(),
        &amount.to_le_bytes(),
    ]);
    require!(
        merkle_verify(proof, token_distributor.root, leaf.0),
        MerkleError::InvalidProof,
    );

    msg!("Mint: {}", ctx.accounts.mint.to_account_info().key());
    msg!(
        "Token receiver address: {}",
        ctx.accounts.recipient.to_account_info().key()
    );
    //accounts needed for the mint
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.recipient.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    msg!("CPI Accounts Assigned");
    //the token program
    let cpi_program = ctx.accounts.token_program.to_account_info();

    //PDA seeds
    let seeds = [
        b"MerkleTokenDistributor".as_ref(),
        &ctx.accounts.merkle_distributor.key().to_bytes(),
        &[merkle_distributor_pda_bump],
    ];
    let seeds_binding = [&seeds[..]];

    //create the CPI context
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &seeds_binding);

    require!(
        ctx.accounts.recipient.owner == ctx.accounts.payer.key(),
        MerkleError::OwnerMismatch
    );
    // anchor's helper function to mint tokens to address
    anchor_spl::token::mint_to(cpi_ctx, amount)?;

    let token_distributor = &mut ctx.accounts.merkle_distributor;
    token_distributor.total_amount_minted += amount;

    require!(
        token_distributor.total_amount_minted <= token_distributor.max_mint_amount,
        MerkleError::ExceededMaxMint
    );
    msg!("Token Minted !!!");

    Ok(())
}
