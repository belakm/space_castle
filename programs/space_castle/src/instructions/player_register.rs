use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount}, 
    associated_token::AssociatedToken,
};
use crate::{player::{Player, PlayerErrorCode}, process_mint_igt, resource::PlayerCache, seeds};

pub fn player_register(ctx: Context<PlayerRegister>, player_name: String) -> Result<()> {
    if player_name.as_bytes().len() > 32 {
        return Err(PlayerErrorCode::NameTooLong.into());
    }
    
    // Set player information
    let player_info = &mut ctx.accounts.player;
    player_info.name = player_name;
    player_info.settled_planets = 0;
    
    // Credit some IGT to the player
    process_mint_igt(
        &ctx.accounts.token_program, 
        (
            &ctx.accounts.account_igt, 
            (&ctx.accounts.mint_igt, ctx.bumps.mint_igt), 
            &ctx.accounts.mint_igt
        ), 
        10
    )
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
    #[account(
        mut,
        seeds = [seeds::MINT_IGT],
        bump
    )]
    pub mint_igt: Account<'info, Mint>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint_igt,
        associated_token::authority = signer 
    )]
    pub account_igt: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = signer,
        space = 8 + PlayerCache::INIT_SPACE,
        seeds = [
            seeds::PLAYER_CACHE,
            signer.key().as_ref()
        ],
        bump
    )]
    pub player_cache: Account<'info, PlayerCache>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
