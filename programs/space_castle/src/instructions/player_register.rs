use anchor_lang::prelude::*;

use crate::{player::{PlayerErrorCode, Player}, seeds};

pub fn player_register(ctx: Context<PlayerRegister>, player_name: String) -> Result<()> {
    if player_name.as_bytes().len() > 32 {
        return Err(PlayerErrorCode::NameTooLong.into());
    }
    let player_info = &mut ctx.accounts.player;
    player_info.name = player_name;
    player_info.settled_planets = 0;

    //  TODO: Mint some gallactic bonds for the new player
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct PlayerRegister<'info> {
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
