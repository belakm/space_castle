mod game;
mod mint;
mod planet;
mod player;
mod ship;

use crate::{game::*, mint::*, planet::*, player::*, ship::*};
use anchor_lang::prelude::*;
use solana_program::clock::Clock;

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
    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        if game_state.is_initialized {
            return Err(GameErrorCode::AlreadyInitialized.into());
        }
        game_state.is_initialized = true;
        Ok(())
    }

    pub fn register_player(ctx: Context<RegisterPlayer>, player_name: String) -> Result<()> {
        if player_name.as_bytes().len() > 32 {
            return Err(PlayerErrorCode::NameTooLong.into());
        }
        let player_info = &mut ctx.accounts.player;
        player_info.name = player_name;
        player_info.settled_planets = 0;

        //  TODO: Mint some gallactic bonds for the new player
        Ok(())
    }

    pub fn settle_first_planet(ctx: Context<SettleFirstPlanet>, x: u16, y: u16) -> Result<()> {
        // CHECK IF PLANET ACTUALLY EXISTS
        if !planet::are_planet_coordinates_valid(x, y) {
            return Err(PlanetErrorCode::NoPlanetAtCoordinates.into());
        }

        // CREATE PLANET METADATA
        let planet_info = &mut ctx.accounts.planet_info;
        if planet_info.is_settled {
            return Err(PlanetErrorCode::PlanetAlreadySettled.into());
        }
        planet_info.is_settled = true;

        // CREATE PLAYERS HOLDING
        let planet_holding = &mut ctx.accounts.planet_holding;
        planet_holding.last_harvest = Clock::get()?.slot;

        // UPDATE PLAYER INFO
        let player_info = &mut ctx.accounts.player_info;
        if player_info.settled_planets != 0 {
            return Err(PlanetErrorCode::MaxOneFirstPlanet.into());
        }
        player_info.settled_planets = 1;

        // CREATE INITIAL SHIP
        let initial_ship = &mut ctx.accounts.initial_ship;
        initial_ship.convert_to_starting_ship();

        Ok(())
    }
}
