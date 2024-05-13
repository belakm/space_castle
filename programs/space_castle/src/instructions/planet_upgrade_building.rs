use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };
use crate::{building::BuildingType, error::GenericError, planet::*, process_burn_chemical, process_burn_crystal, process_burn_fuel, process_burn_metal, seeds };

// TODO: This could be optimized by using a separate call for each building (possibly) 

pub fn planet_upgrade_building(ctx: Context<PlanetUpgradeBuilding>, building_type: BuildingType) -> Result<()> {
    for b in ctx.accounts.planet_holding.buildings.iter_mut() {
        if b.building_type.eq(&building_type) {
            let costs = b.calculate_upgrade_cost();
            b.level += 1;
            for mint_key in ["metal", "crystal", "chemical", "fuel"] {
                let amount: u64;
                match mint_key {
                    "metal" => {
                        amount = costs.metal;
                        if amount == 0 { continue; }
                        let burn_context = (
                           &ctx.accounts.account_metal,
                           (&ctx.accounts.mint_metal, ctx.bumps.mint_metal),
                           &ctx.accounts.mint_metal 
                        );
                        process_burn_metal(&ctx.accounts.token_program, burn_context, amount)?;
                    }
                    "crystal" => {
                        amount = costs.crystal;
                        if amount == 0 { continue; }
                        let burn_context = (
                           &ctx.accounts.account_metal,
                           (&ctx.accounts.mint_metal, ctx.bumps.mint_metal),
                           &ctx.accounts.mint_metal 
                        );
                        process_burn_crystal(&ctx.accounts.token_program, burn_context, amount)?;
                    }
                    "chemical" => { 
                        amount = costs.chemical; 
                        if amount == 0 { continue; }
                        let burn_context = (
                           &ctx.accounts.account_metal,
                           (&ctx.accounts.mint_metal, ctx.bumps.mint_metal),
                           &ctx.accounts.mint_metal 
                        );
                        process_burn_chemical(&ctx.accounts.token_program, burn_context, amount)?;
                    }
                    "fuel" => {
                        amount = costs.metal; 
                        if amount == 0 { continue; }
                        let burn_context = (
                           &ctx.accounts.account_metal,
                           (&ctx.accounts.mint_metal, ctx.bumps.mint_metal),
                           &ctx.accounts.mint_metal 
                        );
                        process_burn_fuel(&ctx.accounts.token_program, burn_context, amount)?;
                    },
                    _ => { return Err(GenericError::WrongMintKey.into()); }
                }
            }
            break;
        }
    }
    Ok(())
}

#[derive(Accounts)]
#[instruction(x: u16, y: u16)]
pub struct PlanetUpgradeBuilding<'info> {
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

