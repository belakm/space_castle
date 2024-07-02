use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::{resource::{PlayerCache, ResourceAuthority}, seeds};

pub fn player_claim_resource_cache(ctx: Context<PlayerClaimResourceCache>) -> Result<()> {
    let player_cache = &mut ctx.accounts.player_cache.resources;
    player_cache.mint(
        &ctx.accounts.token_program, (
        (&ctx.accounts.mint_igt, ctx.bumps.mint_igt),
        (&ctx.accounts.mint_metal, ctx.bumps.mint_metal),
        (&ctx.accounts.mint_crystal, ctx.bumps.mint_crystal),
        (&ctx.accounts.mint_chemical, ctx.bumps.mint_chemical),
        (&ctx.accounts.mint_fuel, ctx.bumps.mint_fuel),

    ), (
        &ctx.accounts.account_igt,
        &ctx.accounts.account_metal,
        &ctx.accounts.account_crystal,
        &ctx.accounts.account_chemical,
        &ctx.accounts.account_fuel,
    ))?;
    player_cache.reset();
    Ok(())
}

#[derive(Accounts)]
pub struct PlayerClaimResourceCache<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [seeds::PLAYER_CACHE, signer.key().as_ref()],
        bump,
    )]
    pub player_cache: Account<'info, PlayerCache>,
    #[account(
        mut,
        seeds = [seeds::RESOURCE_AUTHORITY],
        bump
    )]
    pub resource_authority: Account<'info, ResourceAuthority>,
    // Metal
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
        token::mint = mint_metal, 
        token::authority = resource_authority 
    )]
    pub account_metal:Account<'info, TokenAccount>,
    
    // Crystal
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
        token::mint = mint_crystal, 
        token::authority = resource_authority 
    )]
    pub account_crystal: Account<'info, TokenAccount>,
    
    // Chemical
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
        token::mint = mint_chemical, 
        token::authority = resource_authority 
    )]
    pub account_chemical: Account<'info, TokenAccount>,
    
    // Fuel
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
        token::mint = mint_fuel, 
        token::authority = resource_authority 
    )]
    pub account_fuel: Account<'info, TokenAccount>,
   
    // IGT
    #[account(
        mut,
        seeds = [seeds::MINT_IGT],
        bump
    )]
    pub mint_igt: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_igt,
        associated_token::authority = signer 
    )]
    pub account_igt: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
