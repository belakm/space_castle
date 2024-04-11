use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Ship {
    armor: u8,
    shields: u8,
    engine_module: EngineModule,
    #[max_len(3)]
    weapon_modules: Vec<WeaponModule>,
    #[max_len(3)]
    utility_modules: Vec<UtilityModule>,
}

impl Ship {
    pub fn convert_to_starting_ship(&mut self) {
        self.armor = 32;
        self.shields = 8;
        self.engine_module = EngineModule::Normal;
        self.weapon_modules = vec![WeaponModule {
            weapon_type: WeaponType::Kinetic,
        }];
        self.utility_modules = vec![UtilityModule {
            utility_type: UtilityType::MiningDrill,
        }];
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub struct WeaponModule {
    weapon_type: WeaponType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub struct UtilityModule {
    utility_type: UtilityType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum EngineModule {
    Normal,
    Fast,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum WeaponType {
    Kinetic,
    Laser,
    Rocket,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum UtilityType {
    ShieldGenerator,
    EngineBooster,
    ExtraArmor,
    CommandCenter,
    SeedingBay,
    ScienceLab,
    MiningDrill,
}
