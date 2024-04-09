use anchor_lang::prelude::*;
use crate::seeds;

#[derive(Accounts)]
#[instruction(x: u16, y: u16)]
pub struct ClaimPlanet<'info> {
    #[account(
        init, 
        seeds = [
            seeds::PLANET_INFO, 
            x.to_le_bytes().as_ref(), 
            y.to_le_bytes().as_ref()
        ], 
        bump, 
        payer = player, 
        space = 8 + PlanetInfo::INIT_SPACE 
    )]
    pub planet_info: Account<'info, PlanetInfo>,
    #[account(
        init, 
        seeds = [
            seeds::PLANET_HOLDING, 
            x.to_le_bytes().as_ref(), 
            y.to_le_bytes().as_ref(), 
            player.key().as_ref()
        ], 
        bump, 
        payer = player, 
        space = 8 + PlanetHolding::INIT_SPACE 
    )]
    pub planet_holding: Account<'info, PlanetHolding>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct PlanetInfo {
    pub available_resources: u32,
}

#[derive(InitSpace)]
#[account]
pub struct PlanetHolding {
    pub last_harvest: u32,
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
    PlanetAlreadyClaimed
}
