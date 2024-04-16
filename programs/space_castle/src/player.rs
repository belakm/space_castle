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
    NameTooLong,
}
