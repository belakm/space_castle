use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Mint, MintTo, Token, TokenAccount}, 
    associated_token::AssociatedToken,
};
use crate::{player::{Player, PlayerErrorCode}, resource::PlayerCache, seeds};

pub fn player_register(ctx: Context<PlayerRegister>, player_name: String) -> Result<()> {
    if player_name.as_bytes().len() > 32 {
        return Err(PlayerErrorCode::NameTooLong.into());
    }
    
    // Set player information
    let player_info = &mut ctx.accounts.player;
    player_info.name = player_name;
    player_info.settled_planets = 0;
    
    // Credit some IGT to the player
    let signer_seeds: &[&[&[u8]]] = &[&[seeds::MINT_IGT, &[ctx.bumps.mint]]];
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        // mint 10 IGT to new players
        100 * 10u64.pow(ctx.accounts.mint.decimals as u32),
    )?;

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
    #[account(
        mut,
        seeds = [seeds::MINT_IGT],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer 
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = signer,
        space = PlayerCache::INIT_SPACE,
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
