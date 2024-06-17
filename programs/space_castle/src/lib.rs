mod battle;
mod building;
mod error;
mod fleet;
pub mod instructions;
mod market_pool;
mod planet;
mod player;
mod resource;
mod utilities;

use anchor_lang::prelude::*;
use building::BuildingType;
use instructions::*;

declare_id!("9M2kfet4NAaJyz7Uavx4GAjUexqZrZ6ozoA3QGbkRZHK");

pub mod mint_decimals {
    pub const IGT: u8 = 8;
    pub const METAL: u8 = 8;
    pub const CRYSTAL: u8 = 8;
    pub const CHEMICAL: u8 = 8;
    pub const FUEL: u8 = 8;
}

pub mod seeds {
    pub const PLAYER: &[u8] = b"player";
    pub const PLAYER_CACHE: &[u8] = b"player_cache";
    pub const PLANET_INFO: &[u8] = b"planet_info";
    pub const PLANET_HOLDING: &[u8] = b"planet_holding";
    pub const FLEET: &[u8] = b"fleet";
    pub const GAME: &[u8] = b"game";
    pub const RESOURCE_AUTHORITY: &[u8] = b"resource_authority";
    pub const MINT_IGT: &[u8] = b"mint_igt";
    pub const MINT_METAL: &[u8] = b"mint_metal";
    pub const MINT_CRYSTAL: &[u8] = b"mint_crystal";
    pub const MINT_FUEL: &[u8] = b"mint_fuel";
    pub const MINT_CHEMICAL: &[u8] = b"mint_chemical";
    pub const ACCOUNT_METAL: &[u8] = b"account_metal";
    pub const ACCOUNT_CRYSTAL: &[u8] = b"account_crystal";
    pub const ACCOUNT_CHEMICAL: &[u8] = b"account_chemical";
    pub const ACCOUNT_FUEL: &[u8] = b"account_fuel";

    pub fn mintkey_to_seed(key: &str) -> Option<&[u8]> {
        match key {
            "igt" => Some(MINT_IGT),
            "metal" => Some(MINT_METAL),
            "crystal" => Some(MINT_CRYSTAL),
            "chemical" => Some(MINT_CHEMICAL),
            "fuel" => Some(MINT_FUEL),
            _ => None,
        }
    }
}

#[program]
mod space_castle {
    use super::*;

    /// Player
    ///
    /// Player - Registers a player
    pub fn player_register(ctx: Context<PlayerRegister>, player_name: String) -> Result<()> {
        instructions::player_register(ctx, player_name)
    }
    pub fn player_create_resource_accounts_part1(
        ctx: Context<PlayerCreateResourceAccountsPart1>,
    ) -> Result<()> {
        instructions::player_create_resource_accounts_part1(ctx)
    }
    pub fn player_create_resource_accounts_part2(
        ctx: Context<PlayerCreateResourceAccountsPart2>,
    ) -> Result<()> {
        instructions::player_create_resource_accounts_part2(ctx)
    }
    pub fn player_take_stash(ctx: Context<PlayerCreateResourceAccountsPart2>) -> Result<()> {
        instructions::player_create_resource_accounts_part2(ctx)
    }
    ///
    /// Planet
    ///
    /// Planet - First planet claim for new users
    pub fn planet_first_claim(ctx: Context<PlanetFirstClaim>, x: u16, y: u16) -> Result<()> {
        instructions::planet_first_claim(ctx, x, y)
    }
    pub fn planet_harvest(ctx: Context<PlanetHarvest>, x: u16, y: u16) -> Result<()> {
        instructions::planet_harvest(ctx, x, y)
    }
    /// Planet - build a new building
    pub fn planet_building_new(
        ctx: Context<PlanetBuildingNew>,
        _x: u16,
        _y: u16,
        building_type: BuildingType,
    ) -> Result<()> {
        instructions::planet_building_new(ctx, building_type)
    }
    /// Planet - upgrade a building
    pub fn planet_building_upgrade(
        ctx: Context<PlanetBuildingUpgrade>,
        _x: u16,
        _y: u16,
        building_type: BuildingType,
    ) -> Result<()> {
        instructions::planet_building_upgrade(ctx, building_type)
    }
    /// Planet - change a building
    pub fn planet_building_change(
        ctx: Context<PlanetBuildingChange>,
        _x: u16,
        _y: u16,
        building_type_from: BuildingType,
        building_type_to: BuildingType,
    ) -> Result<()> {
        instructions::planet_building_change(ctx, building_type_from, building_type_to)
    }

    ///
    /// Fleet
    ///
    /// Create a new fleet at (x, y)
    pub fn fleet_new(ctx: Context<FleetNew>, _x: u16, _y: u16) -> Result<()> {
        instructions::fleet_new(ctx)
    }
    /// Move a fleet to (x, y)
    pub fn fleet_move(
        ctx: Context<FleetMove>,
        x: u16,
        y: u16,
        move_x: u16,
        move_y: u16,
    ) -> Result<()> {
        instructions::fleet_move(ctx, x, y, move_x, move_y)
    }
    /// Attack a fleet or planet at (x, y)
    pub fn fleet_attack(
        ctx: Context<FleetAttack>,
        x: u16,
        y: u16,
        target_x: u16,
        target_y: u16,
    ) -> Result<()> {
        instructions::fleet_attack(ctx, x, y, target_x, target_y)
    }

    ///
    /// Mints & Tokens
    ///
    /// Create IGT Mint
    pub fn mint_init_igt(ctx: Context<MintInitIGT>) -> Result<()> {
        instructions::mint_init_igt(ctx)
    }
    /// Mint IGT to X Account
    pub fn mint_igt(ctx: Context<MintIGT>, amount: u64) -> Result<()> {
        instructions::mint_igt(ctx, amount)
    }
    /// Create Metal Mint
    pub fn mint_init_metal(ctx: Context<MintInitMetal>) -> Result<()> {
        instructions::mint_init_metal(ctx)
    }
    /// Mint Metal to X Account
    pub fn mint_metal(ctx: Context<MintMetal>, amount: u64) -> Result<()> {
        instructions::mint_metal(ctx, amount)
    }
    /// Create Chemical Mint
    pub fn mint_init_chemical(ctx: Context<MintInitChemical>) -> Result<()> {
        instructions::mint_init_chemical(ctx)
    }
    /// Mint Chemicals to X Account
    pub fn mint_chemical(ctx: Context<MintChemical>, amount: u64) -> Result<()> {
        instructions::mint_chemical(ctx, amount)
    }
    /// Create Crystal Mint
    pub fn mint_init_crystal(ctx: Context<MintInitCrystal>) -> Result<()> {
        instructions::mint_init_crystal(ctx)
    }
    /// Mint Crystal to X Account
    pub fn mint_crystal(ctx: Context<MintCrystal>, amount: u64) -> Result<()> {
        instructions::mint_crystal(ctx, amount)
    }
    /// Create Fuel Mint
    pub fn mint_init_fuel(ctx: Context<MintInitFuel>) -> Result<()> {
        instructions::mint_init_fuel(ctx)
    }
    /// Mint Fuel to X Account
    pub fn mint_fuel(ctx: Context<MintFuel>, amount: u64) -> Result<()> {
        instructions::mint_fuel(ctx, amount)
    }

    /// Market pool
    ///
    /// Market pool - Create the market liquidity pool
    pub fn market_pool_create(ctx: Context<MarketPoolCreate>) -> Result<()> {
        instructions::market_pool_create(ctx)
    }
    /// Market pool - Provide liquidity to the pool by funding it with some asset
    pub fn market_pool_mint_to(
        ctx: Context<MarketPoolMintTo>,
        amount: u64,
        resource: String,
    ) -> Result<()> {
        instructions::market_pool_mint_to(ctx, amount, resource)
    }
    /// Market pool - Provide liquidity to the pool by funding it with some asset
    pub fn market_pool_fund(
        ctx: Context<MarketPoolFund>,
        amount: u64,
        pay_in_resource: bool,
    ) -> Result<()> {
        instructions::market_pool_fund(ctx, amount, pay_in_resource)
    }
    /// Market pool - swap assets in the Market pool
    pub fn market_pool_swap(
        ctx: Context<MarketPoolSwap>,
        amount_to_swap: u64,
        pay_in_resource: bool,
    ) -> Result<()> {
        instructions::market_pool_swap(ctx, amount_to_swap, pay_in_resource)
    }
}
