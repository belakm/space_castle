use crate::seeds;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(
        init,
        seeds = [seeds::GAME],
        bump,
        payer = signer,
        space = 8 + GameState::INIT_SPACE
    )]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct GameState {
    pub is_initialized: bool,
}

#[error_code]
pub enum GameErrorCode {
    AlreadyInitialized,
}
