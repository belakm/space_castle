use anchor_lang::prelude::*;
use crate::{building::generate_initial_buildings_for_planet, fleet::*, planet::*, player::*, resource::{PlayerCache, Resources}, seeds};

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
    planet_holding.last_harvest = 0;

    // UPDATE PLAYER INFO
    let player_info = &mut ctx.accounts.player_info;
    msg!("{}", player_info.settled_planets);
    if player_info.settled_planets != 0 {
         return Err(PlanetErrorCode::MaxOneFirstPlanet.into());
    }
    player_info.settled_planets = 1;

    // Get affinity (metal = 0, crystal = 1, chemical = 2)
    let planet_resources = get_planet_resources(x, y);
    let planet_affinity = get_planet_affinity(planet_resources);

    // Create initial buildings
    ctx.accounts.planet_holding.buildings = generate_initial_buildings_for_planet(planet_resources);

    // Create one initial fleet for the player 
    let initial_fleet = &mut ctx.accounts.initial_fleet;
    initial_fleet.convert_to_starting_fleet(planet_affinity, ctx.accounts.signer.key());

    // Give player some resources to cache
    let cache = &mut ctx.accounts.player_cache;
    cache.resources.add(Resources {
        igt: 10,
        metal: 10,
        crystal: 10,
        chemical: 10,
        fuel: 10
    });

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
        seeds = [
            seeds::FLEET, 
            x.to_le_bytes().as_ref(), 
            y.to_le_bytes().as_ref(), 
        ], 
        bump,
        space = 8 + Fleet::INIT_SPACE 
    )]
    pub initial_fleet: Account<'info, Fleet>,
    #[account(
        mut,
        seeds = [seeds::PLAYER, signer.key().as_ref()],
        bump,
    )]
    pub player_info: Account<'info, Player>,
    #[account(
        mut,
        seeds = [
            seeds::PLAYER_CACHE,
            signer.key().as_ref()
        ],
        bump
    )]
    pub player_cache: Account<'info, PlayerCache>,
    pub system_program: Program<'info, System>,
 }
