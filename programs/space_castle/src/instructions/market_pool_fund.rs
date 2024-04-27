use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Token},
};

use crate::market_pool::*;

pub fn market_pool_fund(ctx: Context<MarketPoolFund>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.market_pool;

    // Deposit: (From, To, amount)
    let deposit = (
        &ctx.accounts.mint,
        &ctx.accounts.payer_token_account,
        &ctx.accounts.pool_token_account,
        amount,
    );

    pool.fund(
        deposit,
        &ctx.accounts.payer,
        &ctx.accounts.system_program,
        &ctx.accounts.token_program,
    )
}

#[derive(Accounts)]
pub struct MarketPoolFund<'info> {
    /// Liquidity Pool
    #[account(
        mut,
        seeds = [MarketPool::SEED_PREFIX.as_bytes()],
        bump = market_pool.bump,
    )]
    pub market_pool: Account<'info, MarketPool>,
    /// The mint account for the asset being deposited into the pool
    pub mint: Account<'info, token::Mint>,
    /// The Liquidity Pool's token account for the asset being deposited into
    /// the pool
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = market_pool,
    )]
    pub pool_token_account: Account<'info, token::TokenAccount>,
    /// The payer's - or Liquidity Provider's - token account for the asset
    /// being deposited into the pool
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub payer_token_account: Account<'info, token::TokenAccount>,
    // Payer / Liquidity Provider
    #[account(mut)]
    pub payer: Signer<'info>,
    /// System Program: Required for creating the Liquidity Pool's token account
    /// for the asset being deposited into the pool
    pub system_program: Program<'info, System>,
    /// Token Program: Required for transferring the assets from the Liquidity
    /// Provider's token account into the Liquidity Pool's token account
    pub token_program: Program<'info, token::Token>,
    /// Associated Token Program: Required for creating the Liquidity Pool's
    /// token account for the asset being deposited into the pool
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
