use anchor_lang::prelude::*;

use crate::fleet::{Fleet, ShipModule, ShipModuleType};

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

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BattleRound {
    pub losses: [u16; 16],
    pub retreats: [bool; 16],
}

#[account]
pub struct BattleResult {
    pub winner: BattleSide,
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Default, Copy, Clone)]
pub struct FleetStats {
    pub weapons: Weapons,
    pub defenses: Defenses,
}

impl FleetStats {
    pub fn from_modules(modules: &[ShipModule; 6]) -> Self {
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
    pub losses: [u16; 16],
}

pub fn simulate_battle(attacker: &Fleet, defender: &Fleet, r_factor: u8) -> BattleResult {
    let attacker_fleet = &mut attacker.clone();
    let defender_fleet = &mut attacker.clone();
    let att_losses = while !attacker_fleet.in_retreat() || !defender_fleet.in_retreat() {
        attacker_fleet.take_loses(&defender_fleet.get_battle_strength().weapons);
        defender_fleet.take_loses(&attacker_fleet.get_battle_strength().weapons);
    };
    BattleResult {
        winner: BattleSide::Attacker,
    }
}
