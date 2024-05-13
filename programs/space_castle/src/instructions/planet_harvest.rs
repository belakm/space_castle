use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use solana_program::clock::Clock;
use crate::{planet::*, player::*, process_mint_chemical, process_mint_crystal, process_mint_fuel, process_mint_metal, resource::ResourceAuthority, seeds};

pub fn planet_harvest(ctx: Context<PlanetHarvest>, x: u16, y: u16) -> Result<()> {
    let amount = if ctx.accounts.planet_holding.last_harvest == 0 { 100u64 } else { 1u64 }; 

    // Give some initial supplies
    process_mint_metal(
        &ctx.accounts.token_program,
        (
            &ctx.accounts.metal_token_account,
            (&ctx.accounts.mint_metal, ctx.bumps.mint_metal),
            &ctx.accounts.mint_metal
        ),
        amount,
        ctx.accounts.mint_metal.decimals
    )?;
    process_mint_crystal(
        &ctx.accounts.token_program,
        (
            &ctx.accounts.crystal_token_account,
            (&ctx.accounts.mint_crystal, ctx.bumps.mint_crystal),
            &ctx.accounts.mint_crystal
        ),
        amount,
        ctx.accounts.mint_crystal.decimals
    )?;
    process_mint_chemical(
        &ctx.accounts.token_program,
        (
            &ctx.accounts.chemical_token_account,
            (&ctx.accounts.mint_chemical, ctx.bumps.mint_chemical),
            &ctx.accounts.mint_chemical
        ),
        amount,
        ctx.accounts.mint_chemical.decimals
    )?;
    process_mint_fuel(
        &ctx.accounts.token_program,
        (
            &ctx.accounts.fuel_token_account,
            (&ctx.accounts.mint_fuel, ctx.bumps.mint_fuel),
            &ctx.accounts.mint_fuel
        ),
        amount * 2,
        ctx.accounts.mint_fuel.decimals
    )?;

    let planet_holding = &mut ctx.accounts.planet_holding;
    planet_holding.last_harvest = Clock::get()?.slot;

    Ok(())
}

#[derive(Accounts)]
#[instruction(x: u16, y: u16)]
pub struct PlanetHarvest<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            seeds::PLANET_INFO, 
            x.to_le_bytes().as_ref(), 
            y.to_le_bytes().as_ref()
        ], 
        bump, 
    )]
    pub planet_info: Account<'info, PlanetInfo>,
    #[account(
        seeds = [
            seeds::PLANET_HOLDING,
            signer.key().as_ref(),
            x.to_le_bytes().as_ref(), 
            y.to_le_bytes().as_ref(), 
        ], 
        bump, 
    )]
    pub planet_holding: Account<'info, PlanetHolding>,
    #[account(
        mut,
        seeds = [seeds::PLAYER, signer.key().as_ref()],
        bump,
    )]
    pub player_info: Account<'info, Player>,
    #[account(
        mut,
        seeds = [seeds::RESOURCE_AUTHORITY],
        bump
    )]
    pub resource_authority: Account<'info, ResourceAuthority>,
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
    pub metal_token_account:Account<'info, TokenAccount>,
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
        token::mint = mint_chemical, 
        token::authority = resource_authority 
    )]
    pub chemical_token_account:Account<'info, TokenAccount>,
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
    pub fuel_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
 }

