use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PlanetInfo {
    pub available_resources: u32,
    pub is_settled: bool,
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
    #[msg("Planet is already settled and cannot be claimed.")]
    PlanetAlreadySettled,
    #[msg("Player already claimed their first planet.")]
    MaxOneFirstPlanet,
    #[msg("Invalid planet coordinates")]
    NoPlanetAtCoordinates,
}
