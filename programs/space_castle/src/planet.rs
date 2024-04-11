use anchor_lang::prelude::*;
use crate::{seeds, Player, Ship};

#[derive(Accounts)]
#[instruction(x: u16, y: u16)]
pub struct SettleFirstPlanet<'info> {
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

#[account]
#[derive(InitSpace)]
pub struct PlanetInfo {
    pub available_resources: u32,
    pub is_settled: bool
}

#[derive(InitSpace)]
#[account]
pub struct PlanetHolding {
    pub last_harvest: u64,
}

pub fn are_planet_coordinates_valid(x: u16, y: u16) -> bool {
    let total_layers = (x * 2) + 1;
    let spiral_length = total_layers * total_layers;
    let min_val_per_layer = (total_layers).saturating_sub(2) * total_layers.saturating_sub(2);
    let position = if x >= y {
        min_val_per_layer + x.saturating_sub(y)
    } else {
        spiral_length.saturating_sub(y.saturating_sub(x))
    };
    position % 2 != 0 && (position % 7 == 0 || position % 37 == 0 || position % 89 == 0)
}

#[error_code]
pub enum PlanetErrorCode {
    PlanetAlreadySettled,
    MaxOneFirstPlanet,
    NoPlanetAtCoordinates
}
