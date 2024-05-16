use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    building::ResourceCost, process_burn_chemical, process_burn_crystal, process_burn_fuel,
    process_burn_metal,
};

#[derive(InitSpace)]
#[account]
pub struct ResourceAuthority {}

pub fn burn_resources<'info>(
    costs: ResourceCost,
    token_program: &Program<'info, Token>,
    resource_authority: &Account<'info, ResourceAuthority>,
    resource_authority_bump: u8,
    (mint_metal, mint_crystal, mint_chemical, mint_fuel): (
        &Account<'info, Mint>,
        &Account<'info, Mint>,
        &Account<'info, Mint>,
        &Account<'info, Mint>,
    ),
    (account_metal, account_crystal, account_chemical, account_fuel): (
        &Account<'info, TokenAccount>,
        &Account<'info, TokenAccount>,
        &Account<'info, TokenAccount>,
        &Account<'info, TokenAccount>,
    ),
) -> Result<()> {
    for mint_key in ["metal", "crystal", "chemical", "fuel"] {
        let amount = costs.by_key(mint_key);
        if amount == 0 {
            continue;
        }
        match mint_key {
            "metal" => {
                process_burn_metal(
                    token_program,
                    (
                        account_metal,
                        mint_metal,
                        (resource_authority, resource_authority_bump),
                    ),
                    amount,
                )?;
            }
            "crystal" => {
                process_burn_crystal(
                    token_program,
                    (
                        account_crystal,
                        mint_crystal,
                        (resource_authority, resource_authority_bump),
                    ),
                    amount,
                )?;
            }
            "chemical" => {
                process_burn_chemical(
                    token_program,
                    (
                        account_chemical,
                        mint_chemical,
                        (resource_authority, resource_authority_bump),
                    ),
                    amount,
                )?;
            }
            "fuel" => {
                process_burn_fuel(
                    token_program,
                    (
                        account_fuel,
                        mint_fuel,
                        (resource_authority, resource_authority_bump),
                    ),
                    amount,
                )?;
            }
            _ => {}
        }
    }
    Ok(())
}
