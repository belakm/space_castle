use anchor_lang::prelude::*;
use solana_program::clock::Clock;
use crate::{ship::*, planet::*, player::*, seeds};

pub fn planet_first_claim(ctx: Context<PlanetFirstClaim>, x: u16, y: u16) -> Result<()> {
    // CHECK IF PLANET ACTUALLY EXISTS
    if !are_planet_coordinates_valid(x, y) {
        return Err(PlanetErrorCode::NoPlanetAtCoordinates.into());
    }

    // CREATE PLANET METADATA
    let planet_info = &mut ctx.accounts.planet_info;
    if planet_info.is_settled {
        return Err(PlanetErrorCode::PlanetAlreadySettled.into());
    }
    planet_info.is_settled = true;

    // CREATE PLAYERS HOLDING
    let planet_holding = &mut ctx.accounts.planet_holding;
    planet_holding.last_harvest = Clock::get()?.slot;

    // UPDATE PLAYER INFO
    let player_info = &mut ctx.accounts.player_info;
    if player_info.settled_planets != 0 {
        return Err(PlanetErrorCode::MaxOneFirstPlanet.into());
    }
    player_info.settled_planets = 1;

    // CREATE INITIAL SHIP
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
    pub system_program: Program<'info, System>,
        #[account(
        init,
        payer = signer,
        seeds = [seeds::SHIP, signer.key().as_ref(), b"1"], 
        bump,
        space = 8 + Ship::INIT_SPACE 
    )]
    pub initial_ship: Account<'info, Ship>,
    #[account(mut)]
    pub player_info: Account<'info, Player>
}
