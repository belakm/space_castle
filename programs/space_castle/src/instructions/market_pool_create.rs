use anchor_lang::prelude::*;

use crate::market_pool::MarketPool;

pub fn market_pool_create(ctx: Context<MarketPoolCreate>) -> Result<()> {
    // Initialize the new `MarketPool` state
    ctx.accounts
        .market_pool
        .set_inner(MarketPool::new(ctx.bumps.market_pool));
    Ok(())
}

#[derive(Accounts)]
pub struct MarketPoolCreate<'info> {
    /// Liquidity Pool
    #[account(
        init,
        space = MarketPool::SPACE,
        payer = payer,
        seeds = [MarketPool::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub market_pool: Account<'info, MarketPool>,
    /// Rent payer
    #[account(mut)]
    pub payer: Signer<'info>,
    /// System Program: Required for creating the Liquidity Pool
    pub system_program: Program<'info, System>,
}
