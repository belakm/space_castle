mod error;
pub mod instructions;
mod market_pool;
mod mint;
mod planet;
mod player;
mod ship;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("9M2kfet4NAaJyz7Uavx4GAjUexqZrZ6ozoA3QGbkRZHK");

pub mod seeds {
    pub const PLAYER: &[u8] = b"player";
    pub const PLANET_INFO: &[u8] = b"planet_info";
    pub const PLANET_HOLDING: &[u8] = b"planet_holding";
    pub const SHIP: &[u8] = b"ship";
    pub const GAME: &[u8] = b"game";
    pub const MINT_IGT: &[u8] = b"mint_igt";
    pub const MINT_METAL: &[u8] = b"mint_metal";
    pub const MINT_METAL_AUTH: &[u8] = b"mint_metal_auth";
    pub const MINT_CRYSTAL: &[u8] = b"mint_crystal";
    pub const MINT_CRYSTAL_AUTH: &[u8] = b"mint_crystal_auth";
    pub const MINT_FUEL: &[u8] = b"mint_fuel";
    pub const MINT_FUEL_AUTH: &[u8] = b"mint_fuel_auth";
    pub const MINT_CHEMICAL: &[u8] = b"mint_chemical";
    pub const MINT_CHEMICAL_AUTH: &[u8] = b"mint_chemical_auth";
    pub const ACCOUNT_METAL: &[u8] = b"account_metal";
    pub const ACCOUNT_CRYSTAL: &[u8] = b"account_crystal";
    pub const ACCOUNT_CHEMICAL: &[u8] = b"account_chemical";
    pub const ACCOUNT_FUEL: &[u8] = b"account_fuel";
}

#[program]
mod space_castle {
    use super::*;

    ///
    /// Player
    ///
    /// Player - Registers a player
    pub fn player_register(ctx: Context<PlayerRegister>, player_name: String) -> Result<()> {
        instructions::player_register(ctx, player_name)
    }

    ///
    /// Planet
    ///
    /// Planet - First planet claim for new users
    pub fn planet_first_claim(ctx: Context<PlanetFirstClaim>, x: u16, y: u16) -> Result<()> {
        instructions::planet_first_claim(ctx, x, y)
    }

    ///
    /// Mints & Tokens
    ///
    /// Create IGT Mint
    pub fn mint_init_igt(ctx: Context<MintInitIGT>) -> Result<()> {
        instructions::mint_init_igt(ctx)
    }
    /// Mint to X Account
    pub fn mint_igt(ctx: Context<MintIGT>, amount: u64) -> Result<()> {
        instructions::mint_igt(ctx, amount)
    }
    /// Create IGT Mint
    pub fn mint_init_metal(ctx: Context<MintInitMetal>) -> Result<()> {
        instructions::mint_init_metal(ctx)
    }
    /// Mint to X Account
    pub fn mint_metal(ctx: Context<MintMetal>, amount: u64) -> Result<()> {
        instructions::mint_metal(ctx, amount)
    }

    /// Metal
    /// Chemicals
    /// Crystals
    ///
    /// Market pool
    ///
    /// Market pool - Create the market liquidity pool
    pub fn create_pool(ctx: Context<MarketPoolCreate>) -> Result<()> {
        instructions::market_pool_create(ctx)
    }
    /// Market pool - Provide liquidity to the pool by funding it with some asset
    pub fn fund_pool(ctx: Context<MarketPoolFund>, amount: u64) -> Result<()> {
        instructions::market_pool_fund(ctx, amount)
    }
    /// Market pool - swap assets in the Market pool
    pub fn swap(ctx: Context<MarketPoolSwap>, amount_to_swap: u64) -> Result<()> {
        instructions::market_pool_swap(ctx, amount_to_swap)
    }
}
