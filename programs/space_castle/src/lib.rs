mod game;
pub mod instructions;
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
}

#[program]
mod space_castle {
    use super::*;

    pub fn player_register(ctx: Context<PlayerRegister>, player_name: String) -> Result<()> {
        instructions::player_register(ctx, player_name)
    }

    pub fn planet_first_claim(ctx: Context<PlanetFirstClaim>, x: u16, y: u16) -> Result<()> {
        instructions::planet_first_claim(ctx, x, y)
    }
}
