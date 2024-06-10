use crate::{
    battle::{simulate_battle, BattleResult, BattleSide},
    fleet::{Fleet, FleetErrorCode},
    process_burn_fuel, process_mint_chemical, process_mint_crystal, process_mint_fuel,
    process_mint_metal,
    resource::ResourceAuthority,
    seeds,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn fleet_attack(
    ctx: Context<FleetAttack>,
    x: u16,
    y: u16,
    move_to_x: u16,
    move_to_y: u16,
) -> Result<()> {
    let fleet = &mut ctx.accounts.fleet;
    let fleet_target = &mut ctx.accounts.fleet_target;

    let BattleResult {
        winner,
        rounds,
        att_losses,
        def_losses,
    } = simulate_battle(fleet, fleet_target);

    let resource_gain = match winner {
        BattleSide::Attacker => att_losses.div(3).sum(def_losses.div(5)),
        BattleSide::Defender => def_losses.div(3).sum(att_losses.div(5)),
    };
    let (account_metal, account_crystal, account_chemical, account_fuel) = match winner {
        BattleSide::Attacker => (
            &ctx.accounts.account_metal,
            &ctx.accounts.account_crystal,
            &ctx.accounts.account_chemical,
            &ctx.accounts.account_fuel,
        ),
        BattleSide::Defender => (
            &ctx.accounts.account_metal_target,
            &ctx.accounts.account_crystal_target,
            &ctx.accounts.account_chemical_target,
            &ctx.accounts.account_fuel_target,
        ),
    };

    process_mint_metal(
        &ctx.accounts.token_program,
        (
            account_metal,
            (&ctx.accounts.mint_metal, ctx.bumps.mint_metal),
            &ctx.accounts.mint_metal,
        ),
        resource_gain.metal,
        ctx.accounts.mint_metal.decimals,
    )?;
    process_mint_crystal(
        &ctx.accounts.token_program,
        (
            account_crystal,
            (&ctx.accounts.mint_crystal, ctx.bumps.mint_crystal),
            &ctx.accounts.mint_crystal,
        ),
        resource_gain.crystal,
        ctx.accounts.mint_crystal.decimals,
    )?;
    process_mint_chemical(
        &ctx.accounts.token_program,
        (
            account_chemical,
            (&ctx.accounts.mint_chemical, ctx.bumps.mint_chemical),
            &ctx.accounts.mint_chemical,
        ),
        resource_gain.chemical,
        ctx.accounts.mint_chemical.decimals,
    )?;
    process_mint_fuel(
        &ctx.accounts.token_program,
        (
            account_fuel,
            (&ctx.accounts.mint_fuel, ctx.bumps.mint_fuel),
            &ctx.accounts.mint_fuel,
        ),
        resource_gain.fuel,
        ctx.accounts.mint_fuel.decimals,
    )?;

    // Burn fuel of the attacker
    process_burn_fuel(
        &ctx.accounts.token_program,
        (
            &ctx.accounts.account_fuel,
            &ctx.accounts.mint_fuel,
            (
                &ctx.accounts.resource_authority,
                ctx.bumps.resource_authority,
            ),
        ),
        fleet.get_move_quote((x, y), (move_to_x, move_to_y)),
    )
}

#[derive(Accounts)]
#[instruction(x: u16, y: u16, move_to_x: u16, move_to_y: u16)]
pub struct FleetAttack<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // From
    #[account(
        mut,
        seeds = [
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
        init_if_needed,
        seeds = [
            x.to_le_bytes().as_ref(),
            y.to_le_bytes().as_ref()
        ],
        bump,
        space = Fleet::INIT_SPACE,
        payer = signer,
        constraint = fleet_target.is_present() @ FleetErrorCode::FleetNotPresent,
    )]
    pub fleet_target: Account<'info, Fleet>,
    // Resource authority
    #[account(mut, seeds = [seeds::RESOURCE_AUTHORITY], bump)]
    pub resource_authority: Account<'info, ResourceAuthority>,
    // Metal
    #[account(mut, seeds = [seeds::MINT_METAL], bump)]
    pub mint_metal: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::ACCOUNT_METAL, signer.key().as_ref()], bump)]
    pub account_metal: Account<'info, TokenAccount>,
    #[account(mut, seeds = [seeds::ACCOUNT_METAL, fleet_target.owner.as_ref()], bump)]
    pub account_metal_target: Account<'info, TokenAccount>,

    // Crystal
    #[account(mut, seeds = [seeds::MINT_CRYSTAL], bump)]
    pub mint_crystal: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::ACCOUNT_CRYSTAL, signer.key().as_ref()], bump)]
    pub account_crystal: Account<'info, TokenAccount>,
    #[account(mut, seeds = [seeds::ACCOUNT_CRYSTAL, fleet_target.owner.as_ref()], bump)]
    pub account_crystal_target: Account<'info, TokenAccount>,

    // Chemical
    #[account(mut, seeds = [seeds::MINT_CHEMICAL], bump)]
    pub mint_chemical: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::ACCOUNT_CHEMICAL, signer.key().as_ref()], bump)]
    pub account_chemical: Account<'info, TokenAccount>,
    #[account(mut, seeds = [seeds::ACCOUNT_CHEMICAL, fleet_target.owner.as_ref()], bump)]
    pub account_chemical_target: Account<'info, TokenAccount>,

    // Fuel
    #[account(mut, seeds = [seeds::MINT_FUEL], bump)]
    pub mint_fuel: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::ACCOUNT_FUEL, signer.key().as_ref()], bump)]
    pub account_fuel: Account<'info, TokenAccount>,
    #[account(mut, seeds = [seeds::ACCOUNT_FUEL, fleet_target.owner.as_ref()], bump)]
    pub account_fuel_target: Account<'info, TokenAccount>,

    // Programs
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
