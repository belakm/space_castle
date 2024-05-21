use crate::{
    building::ResourceCost,
    fleet::{Fleet, FleetErrorCode},
    planet::*,
    process_burn_igt,
    resource::{burn_resources, ResourceAuthority},
    seeds,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn fleet_attack(ctx: Context<FleetAttack>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction(x: u16, y: u16, target_x: u16, target_y: u16)]
pub struct FleetAttack<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // Fleet
    #[account(
        mut,
        seeds = [
            x.to_le_bytes().as_ref(),
            y.to_le_bytes().as_ref()
        ],
        bump,
    )]
    pub fleet: Account<'info, Fleet>,
    // Resource authority
    #[account(mut, seeds = [seeds::RESOURCE_AUTHORITY], bump)]
    pub resource_authority: Account<'info, ResourceAuthority>,
    // Mints
    #[account(mut, seeds = [seeds::MINT_IGT], bump)]
    pub mint_igt: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::MINT_METAL], bump)]
    pub mint_metal: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::MINT_CHEMICAL], bump)]
    pub mint_chemical: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::MINT_CRYSTAL], bump)]
    pub mint_crystal: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::MINT_FUEL], bump)]
    pub mint_fuel: Account<'info, Mint>,
    // User resource token accounts
    #[account(
        mut,
        associated_token::mint = mint_igt,
        associated_token::authority = signer 
    )]
    pub account_igt: Account<'info, TokenAccount>,
    #[account(mut, seeds = [seeds::ACCOUNT_METAL, signer.key().as_ref()], bump)]
    pub account_metal: Account<'info, TokenAccount>,
    #[account(mut, seeds = [seeds::ACCOUNT_CRYSTAL, signer.key().as_ref()], bump)]
    pub account_crystal: Account<'info, TokenAccount>,
    #[account(mut, seeds = [seeds::ACCOUNT_CHEMICAL, signer.key().as_ref()], bump)]
    pub account_chemical: Account<'info, TokenAccount>,
    #[account(mut, seeds = [seeds::ACCOUNT_FUEL, signer.key().as_ref()], bump)]
    pub account_fuel: Account<'info, TokenAccount>,
    // Programs
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

