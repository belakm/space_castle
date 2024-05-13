use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Player {
    #[max_len(32)]
    pub name: String,
    pub settled_planets: u8,
}

#[error_code]
pub enum PlayerErrorCode {
    #[msg("Player name is too long. Limit is 32 characters.")]
    NameTooLong,
    #[msg("Already picked up starting supplies")]
    StartingSupplies,
}
