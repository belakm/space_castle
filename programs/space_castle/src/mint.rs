use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = signer,
        mint::freeze_authority = signer
    )]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
