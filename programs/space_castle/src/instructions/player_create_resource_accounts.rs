use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::{resource::ResourceAuthority, seeds};

pub fn player_create_resource_accounts_part1(ctx: Context<PlayerCreateResourceAccountsPart1>) -> Result<()> {
    Ok(())
}

pub fn player_create_resource_accounts_part2(ctx: Context<PlayerCreateResourceAccountsPart2>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct PlayerCreateResourceAccountsPart1<'info> {
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
        seeds = [seeds::MINT_METAL],
        bump,
    )]
    pub mint_metal: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = signer,
        seeds = [seeds::ACCOUNT_METAL, signer.key().as_ref()],
        bump,
        token::mint = mint_metal, 
        token::authority = resource_authority 
    )]
    pub metal_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [seeds::MINT_CRYSTAL],
        bump,
    )]
    pub mint_crystal: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = signer,
        seeds = [seeds::ACCOUNT_CRYSTAL, signer.key().as_ref()],
        bump,
        token::mint = mint_crystal, 
        token::authority = resource_authority 
    )]
    pub crystal_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct PlayerCreateResourceAccountsPart2<'info> {
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
        seeds = [seeds::MINT_CHEMICAL],
        bump,
    )]
    pub mint_chemical: Account<'info, Mint>,
    #[account(
        init,
        payer = signer,
        seeds = [seeds::ACCOUNT_CHEMICAL, signer.key().as_ref()],
        bump,
        token::mint = mint_chemical, 
        token::authority = resource_authority 
    )]
    pub chemical_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [seeds::MINT_FUEL],
        bump,
    )]
    pub mint_fuel: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = signer,
        seeds = [seeds::ACCOUNT_FUEL, signer.key().as_ref()],
        bump,
        token::mint = mint_fuel, 
        token::authority = resource_authority 
    )]
    pub fuel_token_account: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
