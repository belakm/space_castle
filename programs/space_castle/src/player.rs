use anchor_lang::prelude::*;
use crate::seeds;

#[account]
#[derive(InitSpace)]
pub struct Player {
    #[max_len(32)]
    pub name: String,
    pub settled_planets: u8
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct RegisterPlayer<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [seeds::PLAYER, signer.key().as_ref()], 
        bump,
        space = 8 + Player::INIT_SPACE 
    )]
    pub player: Account<'info, Player>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum PlayerErrorCode {
    NameTooLong
}
