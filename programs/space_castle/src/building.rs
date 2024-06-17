use anchor_lang::prelude::*;

use crate::{
    mint_decimals,
    resource::Resources,
    utilities::{calculate_upgrade_cost, convert_from_float},
};

#[account]
#[derive(InitSpace, Copy)]
pub struct Building {
    pub level: u8,
    pub building_type: BuildingType,
}

impl Building {
    const UPGRADE_FACTOR: f32 = 1.1;

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

    /// Base upgrade cost for this building
    ///
    /// # Returns
    ///
    /// * [`metal`, `crystal`, `chemical`, `fuel`]
    ///
    pub fn base_upgrade_cost(&self) -> [f32; 4] {
        match self.building_type {
            // General buildings
            BuildingType::Shipyard => [100.0, 100.0, 100.0, 100.0],
            BuildingType::AstralNavyHQ => [50.0, 50.0, 50.0, 150.0],
            BuildingType::TradeBeacon => [250.0, 250.0, 250.0, 250.0],
            BuildingType::Infrastructure => [20.0, 20.0, 20.0, 50.0],
            BuildingType::PlanetaryCapital => [20.0, 20.0, 20.0, 100.0],
            // Resource buildings
            BuildingType::MetalIndustry => [10.0, 5.0, 5.0, 10.0],
            BuildingType::CrystalLabs => [5.0, 10.0, 5.0, 10.0],
            BuildingType::ChemicalRefinery => [5.0, 5.0, 10.0, 10.0],
            BuildingType::FuelExtractors => [50.0, 50.0, 50.0, 150.0],
            // Should never happen
            BuildingType::None => [0.0, 0.0, 0.0, 0.0],
        }
    }

    /// Calculates the upgrade cost for this building
    ///
    /// # Returns
    ///
    /// A tuple where its params are quantities of resources each u16:
    /// * [`metal`, `crystal`, `chemical`, `fuel`]
    ///
    pub fn calculate_upgrade_cost(&self) -> Resources {
        let [metal, crystal, chemical, fuel] = self.base_upgrade_cost();
        Resources {
            igt: 0,
            metal: convert_from_float(
                calculate_upgrade_cost(metal, Building::UPGRADE_FACTOR, self.level),
                mint_decimals::METAL,
            ),
            crystal: convert_from_float(
                calculate_upgrade_cost(crystal, Building::UPGRADE_FACTOR, self.level),
                mint_decimals::CRYSTAL,
            ),
            chemical: convert_from_float(
                calculate_upgrade_cost(chemical, Building::UPGRADE_FACTOR, self.level),
                mint_decimals::CHEMICAL,
            ),
            fuel: convert_from_float(
                calculate_upgrade_cost(fuel, Building::UPGRADE_FACTOR, self.level),
                mint_decimals::FUEL,
            ),
        }
    }
}

#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, PartialEq, Eq, PartialOrd, Ord,
)]
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

impl BuildingType {
    pub fn from_str(input: &str) -> Result<BuildingType> {
        match input {
            "Infrastructure" => Ok(BuildingType::Infrastructure),
            "PlanetaryCapital" => Ok(BuildingType::PlanetaryCapital),
            "ChemicalRefinery" => Ok(BuildingType::ChemicalRefinery),
            "MetalIndustry" => Ok(BuildingType::MetalIndustry),
            "CrystalLabs" => Ok(BuildingType::CrystalLabs),
            "FuelExtractors" => Ok(BuildingType::FuelExtractors),
            "AstralNavyHQ" => Ok(BuildingType::AstralNavyHQ),
            "Shipyard" => Ok(BuildingType::Shipyard),
            "TradeBeacon" => Ok(BuildingType::TradeBeacon),
            _ => Err(BuildingErrorCode::BuildingKey.into()),
        }
    }
}

pub fn generate_initial_buildings_for_planet(resources: [u16; 4]) -> [Building; 6] {
    let mut starting_buildings = [Building::default(); 6];
    starting_buildings[0] = Building::default_planetary_capital();
    starting_buildings[1] = Building::default_shipyard();
    starting_buildings[2] = Building::default_industry(resources);
    starting_buildings
}

#[error_code]
pub enum BuildingErrorCode {
    #[msg("Building key given doesn't match any buildings")]
    BuildingKey,
    #[msg("Building has already been built on this planet")]
    BuildingAlreadyBuilt,
    #[msg("Planet has no empty building slots left")]
    NoBuildingSpotLeft,
    #[msg("BuildingNotPresent")]
    BuildingNotPresent,
}
