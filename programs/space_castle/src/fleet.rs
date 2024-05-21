use anchor_lang::prelude::*;

use crate::{
    building::{Building, BuildingType},
    mint_decimals,
    utilities::{calculate_upgrade_cost, convert_from_float, multiply_costs, sum_costs},
};

#[account]
#[derive(InitSpace)]
/// Fleet occupying (x, y) position. I
pub struct Fleet {
    owner: Pubkey,
    is_present: bool,
    squadrons: [Squadron; 16],
}

impl Fleet {
    pub fn into_starting_fleet(&mut self, affinity: u8) -> Self {
        let mut squadrons = [Squadron::default(); 16];
        let mut template = ShipTemplate::default();
        template.starting_ship(affinity);
        squadrons[0] = Squadron {
            template,
            amount: 3,
        };
        Self {
            squadrons,
            is_present: true,
            owner: Pubkey::default(),
        }
    }
    pub fn get_quote(&self, holding_buildings: [Building; 6]) -> Result<(u64, [u64; 4])> {
        let mut quote: (u64, [u64; 4]) = (0, [0, 0, 0, 0]);
        for squadron in self.squadrons {
            let squadron_cost = squadron
                .template
                .get_quote(squadron.amount, holding_buildings)?;
            quote = sum_costs(quote, multiply_costs(squadron_cost, squadron.amount as u64));
        }
        Ok(quote)
    }
    pub fn set_owner(&mut self, new_owner: Pubkey) {
        self.owner = new_owner;
    }
    /// Set whether the fleet is there or not. This is because a fleet cannot be "deleted" in the
    /// traditional sense.
    pub fn set_is_present(&mut self, is_present: bool) {
        self.is_present = is_present;
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Default)]
pub struct Squadron {
    template: ShipTemplate,
    amount: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Default)]
pub struct ShipTemplate {
    armor: u8,
    shields: u8,
    hull: u8,
    modules: [ShipModule; 6],
}

impl ShipTemplate {
    pub fn starting_ship(&mut self, affinity: u8) {
        self.armor = 1;
        self.shields = 1;
        self.modules = [ShipModule::default(); 6];
        self.modules[0] = ShipModule {
            module_type: ShipModuleType::MiningDrill,
            level: 1,
        };
        self.modules[1] = ShipModule {
            module_type: ShipModuleType::weapon_from_affinity(affinity),
            level: 1,
        }
    }
    pub fn get_quote(
        &self,
        amount: u16,
        holding_buildings: [Building; 6],
    ) -> Result<(u64, [u64; 4])> {
        let mut costs: (u64, [u64; 4]) = (0, [0, 0, 0, 0]);
        for module in self
            .modules
            .iter()
            .filter(|m| !m.module_type.eq(&ShipModuleType::None))
        {
            // Check if the module can be built
            module.module_type.can_be_built(holding_buildings)?;
            // Calculate costs
            let r_cost = module.module_type.get_quote(module.level)?;
            let igt_cost =
                convert_from_float(module.module_type.base_cost_igt(), mint_decimals::IGT);
            costs = sum_costs(costs, multiply_costs((igt_cost, r_cost), amount as u64));
        }
        Ok(costs)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Default)]
pub struct ShipModule {
    module_type: ShipModuleType,
    level: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Default, PartialEq)]
pub enum ShipModuleType {
    #[default]
    None,
    Rockets,
    MachineGun,
    Lasers,
    HardenedHull,
    ShieldBooster,
    AdditionalArmor,
    HaulingBay,
    TargetingSystems,
    MiningDrill,
    LandingPods,
}

impl ShipModuleType {
    /// # Params
    /// * affinity - 0 = metal, 1 = crystal, other = chemical
    pub fn weapon_from_affinity(affinity: u8) -> ShipModuleType {
        match affinity {
            0 => ShipModuleType::MachineGun,
            1 => ShipModuleType::Lasers,
            _ => ShipModuleType::Rockets,
        }
    }

    pub fn can_be_built(&self, buildings: [Building; 6]) -> Result<()> {
        let relevant_building: Option<BuildingType> = match self {
            ShipModuleType::None => None,
            ShipModuleType::Rockets => Some(BuildingType::ChemicalRefinery),
            ShipModuleType::MachineGun => Some(BuildingType::MetalIndustry),
            ShipModuleType::Lasers => Some(BuildingType::CrystalLabs),
            ShipModuleType::HardenedHull => Some(BuildingType::CrystalLabs),
            ShipModuleType::ShieldBooster => Some(BuildingType::CrystalLabs),
            ShipModuleType::AdditionalArmor => Some(BuildingType::CrystalLabs),
            ShipModuleType::HaulingBay => None,
            ShipModuleType::TargetingSystems => Some(BuildingType::AstralNavyHQ),
            ShipModuleType::MiningDrill => None,
            ShipModuleType::LandingPods => Some(BuildingType::AstralNavyHQ),
        };
        if let Some(relevant_building) = relevant_building {
            buildings
                .binary_search_by_key(&relevant_building, |b| b.building_type)
                .map_err(|_| FleetErrorCode::CantBuildMissingBuilding)?;
        }
        Ok(())
    }

    pub fn get_quote(&self, level: u8) -> Result<[u64; 4]> {
        let base_cost = self.base_cost();
        Ok([
            convert_from_float(
                calculate_upgrade_cost(base_cost[0], 1.6, level),
                mint_decimals::METAL,
            ),
            convert_from_float(
                calculate_upgrade_cost(base_cost[1], 1.6, level),
                mint_decimals::CRYSTAL,
            ),
            convert_from_float(
                calculate_upgrade_cost(base_cost[2], 1.6, level),
                mint_decimals::CHEMICAL,
            ),
            convert_from_float(
                calculate_upgrade_cost(base_cost[3], 1.6, level),
                mint_decimals::FUEL,
            ),
        ])
    }

    pub fn base_cost(&self) -> [f32; 4] {
        match self {
            ShipModuleType::None => [0.0, 0.0, 0.0, 0.0],
            ShipModuleType::Rockets => [0.5, 0.5, 2.5, 0.5],
            ShipModuleType::MachineGun => [2.5, 0.5, 0.5, 0.5],
            ShipModuleType::Lasers => [0.5, 2.5, 0.5, 0.5],
            ShipModuleType::HardenedHull => [3.0, 0.5, 0.5, 0.5],
            ShipModuleType::ShieldBooster => [0.5, 3.0, 0.5, 0.5],
            ShipModuleType::AdditionalArmor => [0.5, 0.5, 3.0, 0.5],
            ShipModuleType::HaulingBay => [20.0, 0.5, 0.5, 0.5],
            ShipModuleType::TargetingSystems => [10.0, 10.0, 10.0, 2.0],
            ShipModuleType::MiningDrill => [2.0, 2.0, 2.0, 5.0],
            ShipModuleType::LandingPods => [2.5, 0.5, 2.0, 5.0],
        }
    }

    pub fn base_cost_igt(&self) -> f32 {
        match self {
            ShipModuleType::None => 0.0,
            ShipModuleType::Rockets => 0.001,
            ShipModuleType::MachineGun => 0.001,
            ShipModuleType::Lasers => 0.001,
            ShipModuleType::HardenedHull => 0.0001,
            ShipModuleType::ShieldBooster => 0.0001,
            ShipModuleType::AdditionalArmor => 0.0001,
            ShipModuleType::HaulingBay => 0.001,
            ShipModuleType::TargetingSystems => 0.008,
            ShipModuleType::MiningDrill => 0.001,
            ShipModuleType::LandingPods => 0.001,
        }
    }
}

#[error_code]
pub enum FleetErrorCode {
    #[msg("Cannot build this ship template, missing a key building on planet")]
    CantBuildMissingBuilding,
    #[msg("Planet does not have a shipyard")]
    NoShipyardOnPlanet,
}
