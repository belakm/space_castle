mod planet;
mod player;
mod ship;
mod mint;

use anchor_lang::prelude::*;
use crate::{planet::*, player::*, ship::*, mint::*};

declare_id!("9M2kfet4NAaJyz7Uavx4GAjUexqZrZ6ozoA3QGbkRZHK");

pub mod seeds {
    pub const PLAYER: &[u8] = b"player";
    pub const PLANET_INFO: &[u8] = b"planet_info";
    pub const PLANET_HOLDING: &[u8] = b"planet_holding";
    pub const SHIP: &[u8] = b"ship";
}

#[program]
mod space_castle{
    use super::*;
    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        if game_state.is_initialized {
            return Err(GameErrorCode::AlreadyInitialized.into());
        }
        game_state.is_initialized = true;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeGame<'info> {
    #[account(
        init,
        seeds = [b"gamestate".as_ref()],
        bump,
        payer = admin,
        space = 8 + 32 
    )]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GameState {
    is_initialized: bool,
}

#[error_code]
pub enum GameErrorCode {
    AlreadyInitialized,
}
