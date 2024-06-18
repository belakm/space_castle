use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };
use crate::{building::BuildingType, fleet::{Fleet, FleetErrorCode}, planet::*, process_burn_igt, resource::{burn_resources, ResourceAuthority}, seeds };

pub fn fleet_new(ctx: Context<FleetNew>) -> Result<()> {
    let shipyard = ctx.accounts.planet_holding.buildings.iter_mut().find(|b| b.building_type == BuildingType::Shipyard);
    if shipyard.is_none() {
        return Err(FleetErrorCode::NoShipyardOnPlanet.into())
    }; 
    let fleet = &mut ctx.accounts.fleet;
    fleet.set_presence(ctx.accounts.signer.key());
    fleet.can_be_built(ctx.accounts.planet_holding.buildings)?;
    let quote = fleet.get_quote();
    burn_resources(
        quote.clone(), 
        &ctx.accounts.token_program, 
        &ctx.accounts.resource_authority, 
        ctx.bumps.resource_authority,
        (
            &ctx.accounts.mint_metal, 
            &ctx.accounts.mint_crystal, 
            &ctx.accounts.mint_chemical, 
            &ctx.accounts.mint_fuel
        ),
        (
            &ctx.accounts.account_metal, 
            &ctx.accounts.account_crystal, 
            &ctx.accounts.account_chemical, 
            &ctx.accounts.account_fuel
        )
    )
    // process_burn_igt(&ctx.accounts.token_program, (
    //     // from
    //     &ctx.accounts.account_igt,
    //     // mint
    //     &ctx.accounts.mint_igt,
    //     // authority
    //     &ctx.accounts.mint_igt
    // ), quote.igt)
}

#[derive(Accounts)]
#[instruction(x: u16, y: u16)]
pub struct FleetNew<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // Planet holdings
    #[account(
        seeds = [
            seeds::PLANET_HOLDING,
            signer.key().as_ref(),
            x.to_le_bytes().as_ref(), 
            y.to_le_bytes().as_ref(), 
        ], 
        bump, 
    )]
    pub planet_holding: Account<'info, PlanetHolding>,
    
    // Planet info
    #[account(
        mut, 
        seeds = [
            seeds::PLANET_INFO,
            signer.key().as_ref(),
            x.to_le_bytes().as_ref(), 
            y.to_le_bytes().as_ref(), 
        ], 
        bump, 
    )]
    pub planet_info: Account<'info, PlanetInfo>,

    // Fleet
    #[account(
        init_if_needed,
        seeds = [
            x.to_le_bytes().as_ref(),
            y.to_le_bytes().as_ref()
        ],
        bump,
        space = Fleet::INIT_SPACE,
        payer = signer
    )]
    pub fleet: Account<'info, Fleet>,

    // Resource authority
    #[account(mut, seeds = [seeds::RESOURCE_AUTHORITY], bump)]
    pub resource_authority: Account<'info, ResourceAuthority>,

    // Mints
    // #[account(mut, seeds = [seeds::MINT_IGT], bump)]
    // pub mint_igt: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::MINT_METAL], bump)]
    pub mint_metal: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::MINT_CHEMICAL], bump)]
    pub mint_chemical: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::MINT_CRYSTAL], bump)]
    pub mint_crystal: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::MINT_FUEL], bump)]
    pub mint_fuel: Account<'info, Mint>,

    // User resource token accounts
    // #[account(
    //     mut,
    //     associated_token::mint = mint_igt,
    //     associated_token::authority = signer 
    // )]
    // pub account_igt: Account<'info, TokenAccount>,
    #[account(mut, seeds = [seeds::ACCOUNT_METAL, signer.key().as_ref()], bump)]
    pub account_metal: Account<'info, TokenAccount>,
    #[account(mut, seeds = [seeds::ACCOUNT_CRYSTAL, signer.key().as_ref()], bump)]
    pub account_crystal: Account<'info, TokenAccount>,
    #[account(mut, seeds = [seeds::ACCOUNT_CHEMICAL, signer.key().as_ref()], bump)]
    pub account_chemical: Account<'info, TokenAccount>,
    #[account(mut, seeds = [seeds::ACCOUNT_FUEL, signer.key().as_ref()], bump)]
    pub account_fuel: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

