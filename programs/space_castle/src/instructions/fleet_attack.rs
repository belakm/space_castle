use crate::{
    battle::{fleet_battle, BattleResult, BattleSide},
    fleet::{Fleet, FleetErrorCode},
    resource::{process_burn_resource, PlayerCache, ResourceAuthority},
    seeds,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn fleet_attack(
    ctx: Context<FleetAttack>,
    x: u16,
    y: u16,
    target_x: u16,
    target_y: u16,
) -> Result<()> {
    let fleet = &mut ctx.accounts.fleet;
    let fleet_target = &mut ctx.accounts.fleet_target;

    // Here is where the battle happens
    let BattleResult {
        winner,
        rounds,
        att_losses,
        def_losses,
    } = fleet_battle(fleet, fleet_target);

    // Calculate resources the winner gets
    let resource_gain = match winner {
        BattleSide::Attacker => att_losses.div(3).sum(def_losses.div(5)),
        BattleSide::Defender => def_losses.div(3).sum(att_losses.div(5)),
    };

    // Determine winner account
    let winner_player_cache = match winner {
        BattleSide::Attacker => &mut ctx.accounts.player_cache,
        BattleSide::Defender => &mut ctx.accounts.player_cache_target,
    };
    // Add resources to account
    winner_player_cache.resources = winner_player_cache.resources.sum(resource_gain);

    // Burn fuel of the attacker
    process_burn_resource(
        &ctx.accounts.token_program,
        (
            &ctx.accounts.account_fuel,
            &ctx.accounts.mint_fuel,
            (
                &ctx.accounts.resource_authority,
                ctx.bumps.resource_authority,
            ),
        ),
        fleet.get_move_quote((x, y), (target_x, target_y)),
    )
}

#[derive(Accounts)]
#[instruction(x: u16, y: u16, target_x: u16, target_y: u16)]
pub struct FleetAttack<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // From
    #[account(
        mut,
        seeds = [
            seeds::FLEET,
            x.to_le_bytes().as_ref(),
            y.to_le_bytes().as_ref()
        ],
        bump,
        constraint = fleet.is_present() @ FleetErrorCode::FleetNotPresent,
        constraint = fleet.is_owned_by(&signer.key()) @ FleetErrorCode::NoAuthority,
    )]
    pub fleet: Account<'info, Fleet>,
    // To
    #[account(
        mut,
        seeds = [
            seeds::FLEET,
            target_x.to_le_bytes().as_ref(),
            target_y.to_le_bytes().as_ref()
        ],
        bump,
        constraint = fleet_target.is_present() @ FleetErrorCode::FleetNotPresent,
    )]
    pub fleet_target: Account<'info, Fleet>,
    // Resource authority
    #[account(mut, seeds = [seeds::RESOURCE_AUTHORITY], bump)]
    pub resource_authority: Account<'info, ResourceAuthority>,

    // Player Resource caches - we use this to minimize transaction size
    #[account(
        mut,
        seeds = [
           seeds::PLAYER_CACHE,
           signer.key().as_ref()
        ],
        bump,
    )]
    pub player_cache: Account<'info, PlayerCache>,
    #[account(
        mut,
        seeds = [
           seeds::PLAYER_CACHE,
           signer.key().as_ref()
        ],
        bump,
    )]
    pub player_cache_target: Account<'info, PlayerCache>,

    // Fuel
    #[account(mut, seeds = [seeds::MINT_FUEL], bump)]
    pub mint_fuel: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::ACCOUNT_FUEL, signer.key().as_ref()], bump)]
    pub account_fuel: Account<'info, TokenAccount>,

    // Programs
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
