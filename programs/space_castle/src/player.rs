use anchor_lang::prelude::*;
use crate::{seeds, Ship};

#[account]
#[derive(InitSpace)]
pub struct Player {
    #[max_len(24)]
    pub name: String,
    pub num_fleets: u16
}

#[derive(Accounts)]
pub struct CreatePlayer<'info> {
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
    #[account(
        init,
        payer = signer,
        seeds = [seeds::SHIP, signer.key().as_ref(), b"1"], 
        bump,
        space = 8 + Player::INIT_SPACE 
    )]
    pub initial_ship: Account<'info, Ship>,
    pub system_program: Program<'info, System>,
}
