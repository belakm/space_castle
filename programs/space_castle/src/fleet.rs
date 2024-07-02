use crate::{
    battle::{BattlePresence, Defenses, FleetBattleRound, FleetStats, Morale, Weapons},
    building::{Building, BuildingType},
    mint_decimals,
    resource::Resources,
    utilities::{calculate_upgrade_cost, convert_from_float},
};
use anchor_lang::prelude::*;
use std::ops::{Div, Mul};

pub const SQUADRONS_IN_FLEET: usize = 9;
pub const MODULES_ON_SHIP: usize = 6;

#[account]
#[derive(InitSpace)]
/// Fleet occupying (x, y) position
pub struct Fleet {
    pub owner: Pubkey,
    is_present: bool,
    squadrons: [Option<Squadron>; SQUADRONS_IN_FLEET],
}

impl Fleet {
    pub fn convert_to_starting_fleet(&mut self, affinity: u8, owner: Pubkey) {
        let mut squadrons = [None; SQUADRONS_IN_FLEET];
        let template = starting_ship(affinity);
        squadrons[0] = Some(Squadron {
            template,
            amount: 3,
            morale: Morale::Normal,
            presence: BattlePresence::Active,
        });
        self.squadrons = squadrons;
        self.is_present = true;
        self.owner = owner
    }

    pub fn can_be_built(&self, holding_buildings: [Building; 6]) -> Result<()> {
        for squadron in self.squadrons.iter().filter_map(|s| s.as_ref()) {
            ship_can_be_built(squadron.template, holding_buildings)?;
        }
        Ok(())
    }

    pub fn get_quote(&self) -> Resources {
        let mut quote = Resources::default();
        for squadron in self.squadrons.iter().filter_map(|s| s.as_ref()) {
            quote = quote.sum(ship_quote(&squadron.template, squadron.amount))
        }
        quote
    }

    pub fn get_move_quote(&self, (x_from, y_from): (u16, u16), (x_to, y_to): (u16, u16)) -> u64 {
        let distance =
            (((x_to - x_from).saturating_pow(2) + (y_to - y_from).saturating_pow(2)) as f32).sqrt();
        let mut quote = 0u64;
        for squadron in self.squadrons.into_iter().flatten() {
            quote += (ship_move_quote(&squadron.template) as f32).mul(distance) as u64;
        }
        quote
    }

    /// Sets new owner
    pub fn set_presence(&mut self, new_owner: Pubkey) {
        self.owner = new_owner;
        self.is_present = true;
    }
    /// Set whether the fleet is there or not. This is because a fleet cannot be "deleted" in the
    /// traditional sense.
    pub fn reset(&mut self) {
        self.is_present = false;
        self.owner = Pubkey::default();
        self.squadrons = [None; SQUADRONS_IN_FLEET];
    }

    /// Checks if the fleet is there, used for determining whether PDA on x,y is
    /// active
    pub fn is_present(&self) -> bool {
        self.is_present
    }

    /// Checks if the fleet has a specific owner
    pub fn is_owned_by(&self, owner: &Pubkey) -> bool {
        self.owner.eq(owner)
    }

    /// Gets fleet's battle stats
    pub fn get_battle_strength(&self) -> FleetStats {
        let mut fleet_stats = FleetStats::default();
        for squadron in self.squadrons.into_iter().flatten() {
            let squadron_stats = FleetStats::from_modules(&squadron.template);
            let Defenses {
                armor,
                shield,
                hull,
            } = squadron_stats.defenses.multiply(squadron.amount);
            let Weapons {
                kinetic,
                laser,
                explosive,
            } = squadron_stats.weapons.multiply(squadron.amount);
            fleet_stats.defenses.armor += armor;
            fleet_stats.defenses.shield += shield;
            fleet_stats.defenses.hull += hull;
            fleet_stats.weapons.kinetic += kinetic;
            fleet_stats.weapons.laser += laser;
            fleet_stats.weapons.explosive += explosive;
        }
        fleet_stats
    }

    /// Takes losses and returns morale adjustment
    pub fn take_losses(&mut self, attack: &Weapons) -> FleetBattleRound {
        let mut losses = [0u16; SQUADRONS_IN_FLEET];
        let mut morale = [Morale::Broken; SQUADRONS_IN_FLEET];
        let mut presence = [BattlePresence::Gone; SQUADRONS_IN_FLEET];
        let mut active_squadrons: u8 = 0;
        for (index, squadron) in self.squadrons.iter().enumerate() {
            if let Some(squadron) = squadron {
                morale[index] = squadron.morale;
                presence[index] = squadron.presence;
                if !squadron.presence.eq(&BattlePresence::Gone) {
                    active_squadrons += 1;
                }
            }
        }
        // Calc how much dmg per squadron
        let dmg = attack.divide(active_squadrons); // <---- DIV BY ZERO?
        for (index, squadron) in self.squadrons.iter_mut().enumerate() {
            if let Some(squadron) = squadron {
                if squadron.presence.eq(&BattlePresence::Gone) {
                    continue;
                }
                let is_retreating = squadron.morale.eq(&Morale::Broken);
                let (loss, new_morale) = squadron.take_damage(&dmg); // <---- DIV BY ZERO?
                morale[index] = new_morale;
                losses[index] += loss;
                presence[index] = if is_retreating || squadron.amount == 0 {
                    BattlePresence::Gone
                } else if new_morale.eq(&Morale::Broken) {
                    BattlePresence::Retreating
                } else {
                    BattlePresence::Active
                };
                squadron.morale = new_morale;
                squadron.amount = squadron.amount.saturating_sub(loss);
                squadron.presence = presence[index];
            }
        }
        FleetBattleRound {
            losses,
            morale,
            presence,
        }
    }

    pub fn in_retreat(&self) -> bool {
        self.squadrons
            .iter()
            .flatten()
            .filter(|m| !m.presence.eq(&BattlePresence::Gone))
            .count()
            == 0
    }

    pub fn build_from_template(
        &mut self,
        fleet_template: [Option<SquadronBlueprint>; SQUADRONS_IN_FLEET],
    ) {
        let mut full_squadrons = [None; SQUADRONS_IN_FLEET];
        for (index, template_squadron) in fleet_template.iter().enumerate() {
            if let Some(squadron) = template_squadron {
                full_squadrons[index] = Some(Squadron {
                    amount: squadron.amount,
                    template: squadron.template,
                    presence: BattlePresence::Active,
                    morale: Morale::Normal,
                })
            }
        }
        self.squadrons = full_squadrons
    }

    pub fn replace_with_another_fleet(&mut self, new_fleet: &Fleet) {
        self.owner = new_fleet.owner;
        self.squadrons = new_fleet.squadrons;
        self.is_present = new_fleet.is_present;
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Default)]
pub struct SquadronBlueprint {
    template: [ShipModule; MODULES_ON_SHIP], // Aka ShipTemplate
    amount: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Default)]
pub struct Squadron {
    template: [ShipModule; MODULES_ON_SHIP], // Aka ShipTemplate
    amount: u16,
    morale: Morale,
    presence: BattlePresence,
}

impl Squadron {
    pub fn take_damage(&mut self, weapons: &Weapons) -> (u16, Morale) {
        let fleet_stats = FleetStats::from_modules(&self.template);
        let Defenses {
            armor,
            shield,
            hull,
        } = fleet_stats.defenses.multiply(self.amount);
        let total_health: i128 = armor as i128 + shield as i128 + hull as i128;
        let mut shield = shield as i128;
        let mut armor = armor as i128;
        let mut hull = hull as i128;
        let mut overflow: i128 = 0;

        // Calcualte damage with overflow
        for (index, base_damage) in [weapons.laser, weapons.kinetic, weapons.explosive]
            .iter()
            .enumerate()
        {
            if shield > 0 {
                shield = shield.saturating_sub(with_modifier(*base_damage, index as u8, 0) as i128);
                if shield >= 0 {
                    overflow = 0;
                    continue;
                } else {
                    overflow = without_modifier(shield, index as u8, 0);
                }
            }
            if armor > 0 {
                armor = armor.saturating_sub(overflow.saturating_add(with_modifier(
                    *base_damage,
                    index as u8,
                    1,
                ) as i128));
                if armor >= 0 {
                    overflow = 0;
                    continue;
                } else {
                    overflow = without_modifier(armor, index as u8, 1);
                }
            }
            if hull > 0 {
                hull = hull.saturating_sub(overflow.saturating_add(with_modifier(
                    *base_damage,
                    index as u8,
                    2,
                ) as i128));
                if hull >= 0 {
                    overflow = 0;
                    continue;
                } else {
                    overflow = without_modifier(hull, index as u8, 2);
                }
            }
        }
        if hull <= 0 {
            return (0, Morale::Broken);
        };
        let remainder_health = shield.saturating_add(armor).saturating_add(hull);
        let ratio_of_surviving_ships: f32 = (remainder_health as f32).div(total_health as f32); // <---- DIV BY ZERO?
        let new_amount = (self.amount as f32).mul(ratio_of_surviving_ships) as u16;
        let losses = self.amount.saturating_sub(new_amount);
        let morale = if ratio_of_surviving_ships >= 0.6 {
            Morale::Normal
        } else {
            Morale::Broken
        };
        self.morale = morale;
        self.amount = new_amount;
        (losses, morale)
    }
    pub fn from_blueprint(blueprint: SquadronBlueprint) -> Self {
        Self {
            template: blueprint.template,
            amount: blueprint.amount,
            morale: Morale::Normal,
            presence: BattlePresence::Active,
        }
    }
}

const WEAPON_SURFACE_BONUS: f32 = 1.25;

/// Adds modifier of a weapons on a surface to damage
///
/// weapons: 0 - laser, 1 - kinetic, 2 - explosive
/// surface: 0 - shield, 1 - armor, 2 - hull
fn with_modifier(damage: u64, weapon: u8, surface: u8) -> u64 {
    if weapon == surface {
        (damage as f32).mul(WEAPON_SURFACE_BONUS) as u64
    } else {
        damage
    }
}

/// Remove modifier of a weapons on a surface damage
///
/// weapons: 0 - laser, 1 - kinetic, 2 - explosive
/// surface: 0 - shield, 1 - armor, 2 - hull
fn without_modifier(damage: i128, weapon: u8, surface: u8) -> i128 {
    if weapon == surface {
        (damage as f32).mul(1.0 / WEAPON_SURFACE_BONUS) as i128
    } else {
        damage
    }
}

pub type ShipTemplate = [ShipModule; MODULES_ON_SHIP];

pub fn starting_ship(affinity: u8) -> ShipTemplate {
    let mut template = [ShipModule::default(); MODULES_ON_SHIP];
    template[0] = ShipModule {
        module_type: ShipModuleType::MiningDrill,
        level: 1,
    };
    template[1] = ShipModule {
        module_type: ShipModuleType::weapon_from_affinity(affinity),
        level: 1,
    };
    template
}
pub fn ship_can_be_built(template: ShipTemplate, holding_buildings: [Building; 6]) -> Result<()> {
    for module in template
        .iter()
        .filter(|m| !m.module_type.eq(&ShipModuleType::None))
    {
        // Check if the module can be built
        module.module_type.can_be_built(holding_buildings)?;
    }
    Ok(())
}
pub fn ship_quote(template: &ShipTemplate, amount: u16) -> Resources {
    let mut costs = Resources::default();
    for module in template
        .iter()
        .filter(|m| !m.module_type.eq(&ShipModuleType::None))
    {
        costs = costs.sum(
            module
                .module_type
                .get_quote(module.level)
                .mul(amount as u64),
        )
    }
    costs
}
pub fn ship_move_quote(template: &ShipTemplate) -> u64 {
    let mut fuel_cost = 0u64;
    for module in template
        .iter()
        .filter(|m| !m.module_type.eq(&ShipModuleType::None))
    {
        fuel_cost = fuel_cost.saturating_add(module.level as u64);
    }
    fuel_cost
}
pub fn default_morale(template: ShipTemplate) -> Morale {
    match template.iter().any(|m| {
        matches!(
            m.module_type,
            ShipModuleType::Lasers | ShipModuleType::Rockets | ShipModuleType::MachineGun
        )
    }) {
        true => Morale::Normal,
        false => Morale::Broken,
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Default)]
pub struct ShipModule {
    pub module_type: ShipModuleType,
    pub level: u8,
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
            ShipModuleType::ShieldBooster => Some(BuildingType::MetalIndustry),
            ShipModuleType::AdditionalArmor => Some(BuildingType::ChemicalRefinery),
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

    pub fn get_quote(&self, level: u8) -> Resources {
        let base_cost = self.base_cost();
        Resources {
            igt: convert_from_float(self.base_cost_igt(), mint_decimals::IGT),
            metal: convert_from_float(
                calculate_upgrade_cost(base_cost[0], 1.6, level),
                mint_decimals::METAL,
            ),
            crystal: convert_from_float(
                calculate_upgrade_cost(base_cost[1], 1.6, level),
                mint_decimals::CRYSTAL,
            ),
            chemical: convert_from_float(
                calculate_upgrade_cost(base_cost[2], 1.6, level),
                mint_decimals::CHEMICAL,
            ),
            fuel: convert_from_float(
                calculate_upgrade_cost(base_cost[3], 1.6, level),
                mint_decimals::FUEL,
            ),
        }
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

    pub fn base_weapons(&self) -> Weapons {
        match self {
            ShipModuleType::None => Weapons::from_numbers(0, 0, 0),
            ShipModuleType::Lasers => Weapons::from_numbers(3, 0, 0),
            ShipModuleType::MachineGun => Weapons::from_numbers(0, 3, 0),
            ShipModuleType::Rockets => Weapons::from_numbers(0, 0, 3),
            ShipModuleType::ShieldBooster => Weapons::from_numbers(0, 0, 0),
            ShipModuleType::AdditionalArmor => Weapons::from_numbers(0, 0, 0),
            ShipModuleType::HardenedHull => Weapons::from_numbers(0, 0, 0),
            ShipModuleType::HaulingBay => Weapons::from_numbers(0, 0, 0),
            ShipModuleType::TargetingSystems => Weapons::from_numbers(0, 0, 0),
            ShipModuleType::MiningDrill => Weapons::from_numbers(0, 0, 0),
            ShipModuleType::LandingPods => Weapons::from_numbers(0, 0, 0),
        }
    }
    pub fn base_defenses(&self) -> Defenses {
        match self {
            ShipModuleType::None => Defenses::from_numbers(0, 0, 0),
            ShipModuleType::Lasers => Defenses::from_numbers(1, 1, 1),
            ShipModuleType::MachineGun => Defenses::from_numbers(1, 1, 1),
            ShipModuleType::Rockets => Defenses::from_numbers(1, 1, 1),
            ShipModuleType::HardenedHull => Defenses::from_numbers(0, 0, 10),
            ShipModuleType::ShieldBooster => Defenses::from_numbers(10, 0, 0),
            ShipModuleType::AdditionalArmor => Defenses::from_numbers(0, 10, 0),
            ShipModuleType::HaulingBay => Defenses::from_numbers(0, 0, 2),
            ShipModuleType::TargetingSystems => Defenses::from_numbers(0, 0, 1),
            ShipModuleType::MiningDrill => Defenses::from_numbers(1, 1, 2),
            ShipModuleType::LandingPods => Defenses::from_numbers(1, 1, 1),
        }
    }
}

#[error_code]
pub enum FleetErrorCode {
    #[msg("Cannot build this ship template, missing a key building on planet")]
    CantBuildMissingBuilding,
    #[msg("Planet does not have a shipyard")]
    NoShipyardOnPlanet,
    #[msg("Missing authority over this fleet")]
    NoAuthority,
    #[msg("No active fleet at position")]
    FleetNotPresent,
    #[msg("Can't move fleet to position, its already occupied")]
    IllegalMoveAlreadyOccupied,
}
