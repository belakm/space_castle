use crate::{error::MarketPoolError, market_pool::*, seeds};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

pub fn market_pool_mint_to(
    ctx: Context<MarketPoolMintTo>,
    amount: u64,
    resource: String,
) -> Result<()> {
    let pool = &mut ctx.accounts.market_pool;
    let mint_seed = seeds::mintkey_to_seed(&resource);
    match mint_seed {
        Some(mint_seed) => {
            let (_, mint_bump) = Pubkey::find_program_address(&[mint_seed], ctx.program_id);
            pool.mint_to_pool(
                &ctx.accounts.mint,
                (mint_seed, mint_bump),
                &ctx.accounts.pool_token_account,
                amount,
                &ctx.accounts.payer,
                &ctx.accounts.system_program,
                &ctx.accounts.token_program,
            )
        }
        None => Err(MarketPoolError::AssetMint.into()),
    }
}

#[derive(Accounts)]
pub struct MarketPoolMintTo<'info> {
    /// Market Pool
    #[account(
        mut,
        seeds = [MarketPool::SEED_PREFIX.as_bytes()],
        bump = market_pool.bump,
    )]
    pub market_pool: Account<'info, MarketPool>,
    /// The mint account for the asset being deposited into the pool
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    /// The Market Pool's token account for the asset being deposited into
    /// the pool
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = market_pool,
    )]
    pub pool_token_account: Account<'info, TokenAccount>,
    // Payer / Liquidity Provider
    #[account(mut)]
    pub payer: Signer<'info>,
    /// System Program: Required for creating the Market Pool's token account
    /// for the asset being deposited into the pool
    pub system_program: Program<'info, System>,
    /// Token Program: Required for transferring the assets from the Liquidity
    /// Provider's token account into the Market Pool's token account
    pub token_program: Program<'info, Token>,
    /// Associated Token Program: Required for creating the Market Pool's
    /// token account for the asset being deposited into the pool
    pub associated_token_program: Program<'info, AssociatedToken>,
}
