use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use solana_program::clock::Clock;
use crate::{planet::*, player::*, process_mint_chemical, process_mint_crystal, process_mint_fuel, process_mint_metal, resource::{ResourceAuthority, Resources}, seeds};

pub fn planet_harvest(ctx: Context<PlanetHarvest>, _x: u16, _y: u16) -> Result<()> {
    let resources = Resources {
        igt: 1,
        metal: 1,
        crystal: 1,
        chemical: 1,
        fuel: 1
    }; 
    resources.mint(&ctx.accounts.token_program, (
        (&ctx.accounts.mint_igt, ctx.bumps.mint_igt),
        (&ctx.accounts.mint_metal, ctx.bumps.mint_metal),
        (&ctx.accounts.mint_crystal, ctx.bumps.mint_crystal),
        (&ctx.accounts.mint_chemical, ctx.bumps.mint_chemical),
        (&ctx.accounts.mint_fuel, ctx.bumps.mint_fuel),

    ), (
        &ctx.accounts.igt_token_account,
        &ctx.accounts.metal_token_account,
        &ctx.accounts.crystal_token_account,
        &ctx.accounts.chemical_token_account,
        &ctx.accounts.fuel_token_account,
    ))?; 

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
        constraint = planet_info.is_owned_by(&signer.key()) @ PlanetErrorCode::NoAuthority,
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
    pub metal_token_account:Account<'info, TokenAccount>,
    
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
    pub crystal_token_account: Account<'info, TokenAccount>,
    
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
    pub chemical_token_account:Account<'info, TokenAccount>,
    
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
    pub fuel_token_account: Account<'info, TokenAccount>,
    
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
    pub igt_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
 }

