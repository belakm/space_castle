use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount, Mint, MintTo}};
use solana_program::clock::Clock;
use crate::{building::generate_initial_buildings_for_planet, planet::*, player::*, process_mint_chemical, process_mint_metal, resource::ResourceAuthority, seeds, ship::*};

pub fn planet_first_claim(ctx: Context<PlanetFirstClaim>, x: u16, y: u16) -> Result<()> {
    // CHECK IF PLANET ACTUALLY EXISTS
    if !are_planet_coordinates_valid(x, y) {
        return Err(PlanetErrorCode::NoPlanetAtCoordinates.into());
    }

    // CREATE PLANET METADATA
    let planet_info = &mut ctx.accounts.planet_info;
    if planet_info.owner.is_some() {
        return Err(PlanetErrorCode::PlanetAlreadySettled.into());
    }
    planet_info.owner = Some(*ctx.accounts.signer.key);
    planet_info.miner = None;

    // CREATE PLAYERS HOLDING
    let planet_holding = &mut ctx.accounts.planet_holding;
    planet_holding.last_harvest = Clock::get()?.slot;

    // UPDATE PLAYER INFO
    let player_info = &mut ctx.accounts.player_info;
    if player_info.settled_planets != 0 {
        return Err(PlanetErrorCode::MaxOneFirstPlanet.into());
    }
    player_info.settled_planets = 1;

    // Create initial buildings
    ctx.accounts.planet_holding.buildings = generate_initial_buildings_for_planet(x, y);

    // Give some initial resources
    process_mint_metal(
        &ctx.accounts.token_program,
        (
            &ctx.accounts.metal_token_account,
            (&ctx.accounts.mint_metal, ctx.bumps.mint_metal),
            &ctx.accounts.mint_metal
        ),
        10,
        ctx.accounts.mint_metal.decimals
    )?;
    process_mint_chemical(
        &ctx.accounts.token_program,
        (
            &ctx.accounts.chemical_token_account,
            (&ctx.accounts.mint_chemical, ctx.bumps.mint_chemical),
            &ctx.accounts.mint_chemical
        ),
        10,
        ctx.accounts.mint_chemical.decimals
    )?;
 

    // Create one initial ship for the player 
    let initial_ship = &mut ctx.accounts.initial_ship;
    initial_ship.convert_to_starting_ship();

    Ok(())
}

#[derive(Accounts)]
#[instruction(x: u16, y: u16)]
pub struct PlanetFirstClaim<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init, 
        seeds = [
            seeds::PLANET_INFO, 
            x.to_le_bytes().as_ref(), 
            y.to_le_bytes().as_ref()
        ], 
        bump, 
        payer = signer, 
        space = 8 + PlanetInfo::INIT_SPACE 
    )]
    pub planet_info: Account<'info, PlanetInfo>,
    #[account(
        init, 
        seeds = [
            seeds::PLANET_HOLDING,
            signer.key().as_ref(),
            x.to_le_bytes().as_ref(), 
            y.to_le_bytes().as_ref(), 
        ], 
        bump, 
        payer = signer, 
        space = 8 + PlanetHolding::INIT_SPACE 
    )]
    pub planet_holding: Account<'info, PlanetHolding>,
    #[account(
        init,
        payer = signer,
        seeds = [seeds::SHIP, signer.key().as_ref(), b"1"], 
        bump,
        space = 8 + Ship::INIT_SPACE 
    )]
    pub initial_ship: Account<'info, Ship>,
    #[account(mut)]
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
        init_if_needed,
        payer = signer,
        seeds = [seeds::ACCOUNT_METAL, signer.key().as_ref()],
        bump,
        token::mint = mint_metal, 
        token::authority = resource_authority 
    )]
    pub metal_token_account: Account<'info, TokenAccount>,
        #[account(
        mut,
        seeds = [seeds::MINT_CHEMICAL],
        bump,
    )]
    pub mint_chemical: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        seeds = [seeds::ACCOUNT_CHEMICAL, signer.key().as_ref()],
        bump,
        token::mint = mint_chemical, 
        token::authority = resource_authority 
    )]
    pub chemical_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

}
