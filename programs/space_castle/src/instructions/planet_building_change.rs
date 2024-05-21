use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };
use crate::{building::{Building, BuildingErrorCode, BuildingType}, planet::*,  resource::{burn_resources, ResourceAuthority}, seeds };

pub fn planet_building_change(
    ctx: Context<PlanetBuildingChange>, 
    building_type_from: BuildingType, 
    building_type_to: BuildingType
) -> Result<()> {
    let building: Option<&mut Building> = ctx.accounts.planet_holding.buildings.iter_mut().find(|b| b.building_type.eq(&building_type_from));
    match building {
        None => Err(BuildingErrorCode::BuildingNotPresent.into()),
        Some(building) => {
            building.building_type = building_type_to;
            let costs = building.calculate_upgrade_cost();
            building.level = building.level.saturating_div(2).min(1);
            burn_resources(
                costs, 
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
        } 
    }
}

#[derive(Accounts)]
#[instruction(x: u16, y: u16)]
pub struct PlanetBuildingChange<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut, 
        seeds = [
            seeds::PLANET_HOLDING,
            signer.key().as_ref(),
            x.to_le_bytes().as_ref(), 
            y.to_le_bytes().as_ref(), 
        ], 
        bump, 
    )]
    pub planet_holding: Account<'info, PlanetHolding>,

    // Resource authority
    #[account(mut, seeds = [seeds::RESOURCE_AUTHORITY], bump)]
    pub resource_authority: Account<'info, ResourceAuthority>,

    // Mints
    #[account(mut, seeds = [seeds::MINT_METAL], bump)]
    pub mint_metal: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::MINT_CHEMICAL], bump)]
    pub mint_chemical: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::MINT_CRYSTAL], bump)]
    pub mint_crystal: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::MINT_FUEL], bump)]
    pub mint_fuel: Account<'info, Mint>,

    // User resource token accounts
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
