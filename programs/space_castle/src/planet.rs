use anchor_lang::prelude::*;
use solana_program::blake3::hashv;

use crate::building::Building;

#[account]
#[derive(InitSpace)]
pub struct PlanetInfo {
    pub metal: u16,
    pub chemical: u16,
    pub crystal: u16,
    pub fuel: u16,
    pub miner: Option<Pubkey>,
    pub owner: Option<Pubkey>,
}

impl PlanetInfo {
    pub fn planet_affinity(&self) -> u8 {
        return get_planet_affinity([self.metal, self.crystal, self.chemical, self.fuel]);
    }
}

#[derive(InitSpace)]
#[account]
pub struct PlanetHolding {
    pub last_harvest: u64,
    #[max_len(6)]
    pub buildings: [Building; 6],
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

/// Gets planet resources
///
/// # Arguments
///
/// * `x` - x coordinate
/// * `y` - y coordinate
///
/// # Returns
///
/// A tuple where its params are quantities of resources each u16:
/// * [`metal`, `crystal`, `chemical`, `fuel`]
///
pub fn get_planet_resources(x: u16, y: u16) -> [u16; 4] {
    let bytes = [x.to_le_bytes(), y.to_le_bytes()].concat();
    let hash_result = hashv(&[&bytes]);
    let mut new_values = [0u16; 4];
    for (i, chunk) in hash_result.to_bytes().chunks(2).enumerate() {
        if let Ok(slice) = chunk.try_into() {
            if i == 4 {
                break;
            }
            new_values[i] = u16::from_le_bytes(slice);
        }
    }
    new_values
}

/// Gets planet resources
///
/// # Arguments
///
/// * `resources` - [metal, crystal, chemical, fuel]
///
/// # Returns
///
/// Index from 0 to 2 of which resource is the most bountiful
///
pub fn get_planet_affinity([metal, crystal, chemical, _]: [u16; 4]) -> u8 {
    if metal > crystal && metal > chemical {
        0
    } else if crystal > chemical {
        1
    } else {
        2
    }
}

#[error_code]
pub enum PlanetErrorCode {
    #[msg("Planet is already settled and cannot be claimed")]
    PlanetAlreadySettled,
    #[msg("Player already claimed their first planet")]
    MaxOneFirstPlanet,
    #[msg("Invalid planet coordinates")]
    NoPlanetAtCoordinates,
}
