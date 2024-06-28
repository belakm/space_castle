use anchor_lang::prelude::*;

use crate::{
    fleet::{Fleet, ShipModule, ShipModuleType, MODULES_ON_SHIP, SQUADRONS_IN_FLEET},
    resource::Resources,
};

pub const MAX_ROUNDS: usize = 16;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum BattleSide {
    Attacker,
    Defender,
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, PartialEq)]
pub enum BattlePresence {
    #[default]
    Active,
    Retreating,
    Gone,
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, PartialEq)]
pub enum Morale {
    #[default]
    Normal,
    Broken,
}

pub struct BattleResult {
    pub winner: BattleSide,
    pub rounds: [Option<(FleetBattleRound, FleetBattleRound)>; MAX_ROUNDS],
    pub att_losses: Resources,
    pub def_losses: Resources,
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Default, Copy, Clone)]
pub struct FleetStats {
    pub weapons: Weapons,
    pub defenses: Defenses,
}

impl FleetStats {
    pub fn from_modules(modules: &[ShipModule; MODULES_ON_SHIP]) -> Self {
        let mut weapons = Weapons {
            kinetic: 0,
            laser: 0,
            explosive: 0,
        };
        let mut defenses = Defenses {
            armor: 0,
            shield: 0,
            hull: 0,
        };
        for module in modules
            .iter()
            .filter(|m| !m.module_type.eq(&ShipModuleType::None))
        {
            let Weapons {
                kinetic,
                laser,
                explosive,
            } = module.module_type.base_weapons();
            let Defenses {
                armor,
                shield,
                hull,
            } = module.module_type.base_defenses();
            weapons.kinetic += kinetic;
            weapons.laser += laser;
            weapons.explosive += explosive;
            defenses.shield += shield;
            defenses.armor += armor;
            defenses.hull += hull;
        }
        FleetStats { weapons, defenses }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Default, Copy, Clone)]
pub struct Weapons {
    pub kinetic: u64,
    pub laser: u64,
    pub explosive: u64,
}

impl Weapons {
    pub fn divide(&self, parts: u8) -> Weapons {
        Weapons {
            kinetic: self.kinetic.saturating_div(parts as u64),
            laser: self.laser.saturating_div(parts as u64),
            explosive: self.explosive.saturating_div(parts as u64),
        }
    }
    pub fn multiply(&self, times: u16) -> Weapons {
        Weapons {
            kinetic: self.kinetic.saturating_mul(times as u64),
            laser: self.laser.saturating_mul(times as u64),
            explosive: self.explosive.saturating_mul(times as u64),
        }
    }
    pub fn from_numbers(laser: u64, kinetic: u64, explosive: u64) -> Self {
        Weapons {
            kinetic,
            laser,
            explosive,
        }
    }
}

#[derive(Default, InitSpace, Copy, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Defenses {
    pub armor: u64,
    pub shield: u64,
    pub hull: u64,
}

impl Defenses {
    pub fn multiply(&self, times: u16) -> Defenses {
        Defenses {
            armor: self.armor.saturating_mul(times as u64),
            shield: self.armor.saturating_mul(times as u64),
            hull: self.hull.saturating_mul(times as u64),
        }
    }
    pub fn from_numbers(shield: u64, armor: u64, hull: u64) -> Self {
        Defenses {
            shield,
            armor,
            hull,
        }
    }
}

pub struct FleetBattleRound {
    pub losses: [u16; SQUADRONS_IN_FLEET],
    pub morale: [Morale; SQUADRONS_IN_FLEET],
    pub presence: [BattlePresence; SQUADRONS_IN_FLEET],
}

pub fn fleet_battle(attacker_fleet: &mut Fleet, defender_fleet: &mut Fleet) -> BattleResult {
    let att_init_cost = attacker_fleet.get_quote();
    let def_init_cost = defender_fleet.get_quote();
    let mut round = 0;
    let mut rounds: [Option<(FleetBattleRound, FleetBattleRound)>; MAX_ROUNDS] = [
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None,
    ];
    while !attacker_fleet.in_retreat() && !defender_fleet.in_retreat() && round < MAX_ROUNDS {
        let att_weapons = &attacker_fleet.get_battle_strength().weapons;
        let def_weapons = &defender_fleet.get_battle_strength().weapons;
        let att_round = attacker_fleet.take_losses(def_weapons);
        let def_round = defender_fleet.take_losses(att_weapons);
        rounds[round] = Some((att_round, def_round));
        round += 1;
    }
    let winner = if attacker_fleet.in_retreat() {
        BattleSide::Defender
    } else {
        BattleSide::Attacker
    };
    let att_new_cost = attacker_fleet.get_quote();
    let def_new_cost = defender_fleet.get_quote();
    BattleResult {
        winner,
        rounds,
        att_losses: att_init_cost.sub(att_new_cost),
        def_losses: def_init_cost.sub(def_new_cost),
    }
}
