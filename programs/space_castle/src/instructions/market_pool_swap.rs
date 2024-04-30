use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::market_pool::*;
use crate::{error::*, seeds};

pub fn market_pool_swap(
    ctx: Context<MarketPoolSwap>,
    amount_to_swap: u64,
    pay_mint_key: String,
) -> Result<()> {
    if amount_to_swap == 0 {
        return Err(MarketPoolError::SwapZeroAmount.into());
    }

    let pool = &mut ctx.accounts.pool;

    // Receive: The assets the user is requesting to receive in exchange:
    // (Mint, From, To)
    let receive = (
        ctx.accounts.receive_mint.as_ref(),
        ctx.accounts.pool_receive_token_account.as_ref(),
        ctx.accounts.payer_receive_token_account.as_ref(),
    );

    // Pay: The assets the user is proposing to pay in the swap:
    // (Mint, From, To, Amount)
    let pay = (
        ctx.accounts.pay_mint.as_ref(),
        ctx.accounts.payer_pay_token_account.as_ref(),
        ctx.accounts.pool_pay_token_account.as_ref(),
        amount_to_swap,
    );

    pool.process_swap(
        receive,
        pay,
        &ctx.accounts.payer,
        pay_mint_key,
        &ctx.program_id,
        &ctx.accounts.token_program,
    )
}

#[derive(Accounts)]
pub struct MarketPoolSwap<'info> {
    /// Market Pool
    #[account(
        mut,
        seeds = [MarketPool::SEED_PREFIX.as_bytes()],
        bump = pool.bump,
    )]
    pub pool: Account<'info, MarketPool>,
    /// The mint account for the asset the user is requesting to receive in
    /// exchange
    #[account(
        constraint = !receive_mint.key().eq(&pay_mint.key()) @ MarketPoolError::SwapMatchingAssets
    )]
    pub receive_mint: Box<Account<'info, Mint>>,
    /// The Market Pool's token account for the mint of the asset the user is
    /// requesting to receive in exchange (which will be debited)
    #[account(
        mut,
        associated_token::mint = receive_mint,
        associated_token::authority = pool,
    )]
    pub pool_receive_token_account: Box<Account<'info, TokenAccount>>,
    /// The user's token account for the mint of the asset the user is
    /// requesting to receive in exchange (which will be credited)
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = receive_mint,
        associated_token::authority = payer,
    )]
    pub payer_receive_token_account: Box<Account<'info, TokenAccount>>,
    /// The mint account for the asset the user is proposing to pay in the swap
    pub pay_mint: Box<Account<'info, Mint>>,
    /// The Market Pool's token account for the mint of the asset the user is
    /// proposing to pay in the swap (which will be credited)
    #[account(
        mut,
        associated_token::mint = pay_mint,
        associated_token::authority = pool,
    )]
    pub pool_pay_token_account: Box<Account<'info, TokenAccount>>,
    /// The user's token account for the mint of the asset the user is
    /// proposing to pay in the swap (which will be debited)
    #[account(
        mut,
        associated_token::mint = pay_mint,
        associated_token::authority = payer,
    )]
    pub payer_pay_token_account: Box<Account<'info, TokenAccount>>,
    /// The authority requesting to swap (user)
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Token Program: Required for transferring the assets between all token
    /// accounts involved in the swap
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
