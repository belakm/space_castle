use anchor_lang::prelude::*;

use crate::planet::get_planet_resources;

#[account]
#[derive(InitSpace, Copy)]
pub struct Building {
    pub level: u8,
    pub building_type: BuildingType,
}

impl Building {
    pub fn default() -> Self {
        Building {
            level: 0,
            building_type: BuildingType::None,
        }
    }

    pub fn default_planetary_capital() -> Self {
        Building {
            level: 1,
            building_type: BuildingType::PlanetaryCapital,
        }
    }

    pub fn default_shipyard() -> Self {
        Building {
            level: 1,
            building_type: BuildingType::Shipyard,
        }
    }

    pub fn default_industry([metal, crystal, chemical, _]: [u16; 4]) -> Self {
        let building_type: BuildingType;
        if metal > crystal && metal > chemical {
            building_type = BuildingType::MetalIndustry
        } else if crystal > chemical {
            building_type = BuildingType::CrystalLabs
        } else {
            building_type = BuildingType::ChemicalRefinery
        }
        Building {
            level: 1,
            building_type,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum BuildingType {
    None,
    PlanetaryCapital,
    Shipyard,
    MetalIndustry,
    CrystalLabs,
    ChemicalRefinery,
    FuelExtractors,
    TradeBeacon,
    AstralNavyHQ,
    Infrastructure,
}

pub fn generate_initial_buildings_for_planet(x: u16, y: u16) -> [Building; 6] {
    let mut starting_buildings = [Building::default(); 6];
    starting_buildings[0] = Building::default_planetary_capital();
    starting_buildings[1] = Building::default_shipyard();
    starting_buildings[2] = Building::default_industry(get_planet_resources(x, y));
    starting_buildings
}
