use crate::{
    resource::{PlayerCache, ResourceAuthority},
    seeds,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn player_claim_resource_cache(ctx: Context<PlayerClaimResourceCache>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct PlayerClaimResourceCache<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [seeds::RESOURCE_AUTHORITY],
        bump
    )]
    pub resource_authority: Box<Account<'info, ResourceAuthority>>,
    #[account(
        mut,
        seeds = [seeds::PLAYER_CACHE, signer.key().as_ref()],
        bump,
    )]
    pub player_cache: Account<'info, PlayerCache>,
    #[account(
        mut,
        seeds = [seeds::MINT_METAL],
        bump,
    )]
    pub mint_metal: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [seeds::ACCOUNT_METAL, signer.key().as_ref()],
        bump,
    )]
    pub metal_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [seeds::MINT_CRYSTAL],
        bump,
    )]
    pub mint_crystal: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [seeds::ACCOUNT_CRYSTAL, signer.key().as_ref()],
        bump,
    )]
    pub crystal_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [seeds::MINT_CHEMICAL],
        bump,
    )]
    pub mint_chemical: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [seeds::ACCOUNT_CHEMICAL, signer.key().as_ref()],
        bump,
    )]
    pub chemical_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [seeds::MINT_FUEL],
        bump,
    )]
    pub mint_fuel: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [seeds::ACCOUNT_FUEL, signer.key().as_ref()],
        bump,
    )]
    pub fuel_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
