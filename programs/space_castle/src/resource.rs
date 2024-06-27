use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

use crate::{
    process_mint_chemical, process_mint_crystal, process_mint_fuel, process_mint_igt,
    process_mint_metal, seeds,
};

#[account]
#[derive(InitSpace)]
pub struct PlayerCache {
    pub resources: Resources,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, InitSpace)]
pub struct Resources {
    pub igt: u64,
    pub metal: u64,
    pub crystal: u64,
    pub chemical: u64,
    pub fuel: u64,
}

impl Resources {
    pub fn reset(&mut self) -> Self {
        Self {
            igt: 0,
            metal: 0,
            crystal: 0,
            chemical: 0,
            fuel: 0,
        }
    }
    pub fn by_key(&self, key: &str) -> u64 {
        match key {
            "metal" => self.metal,
            "crystal" => self.crystal,
            "chemical" => self.chemical,
            "fuel" => self.fuel,
            "igt" => self.igt,
            _ => 0u64,
        }
    }
    pub fn sum(&self, other: Resources) -> Resources {
        Resources {
            igt: self.igt.saturating_add(other.igt),
            metal: self.metal.saturating_add(other.metal),
            crystal: self.crystal.saturating_add(other.crystal),
            chemical: self.chemical.saturating_add(other.chemical),
            fuel: self.fuel.saturating_add(other.fuel),
        }
    }
    pub fn add(&mut self, other: Resources) {
        self.igt += other.igt;
        self.metal += other.metal;
        self.crystal += other.crystal;
        self.chemical += other.chemical;
        self.fuel += other.fuel;
    }
    pub fn sub(&self, other: Resources) -> Resources {
        Resources {
            igt: self.igt.saturating_sub(other.igt),
            metal: self.metal.saturating_sub(other.metal),
            crystal: self.crystal.saturating_sub(other.crystal),
            chemical: self.chemical.saturating_sub(other.chemical),
            fuel: self.fuel.saturating_sub(other.fuel),
        }
    }
    pub fn div(&self, factor: u64) -> Resources {
        Resources {
            igt: self.igt.saturating_div(factor),
            metal: self.metal.saturating_div(factor),
            crystal: self.crystal.saturating_div(factor),
            chemical: self.chemical.saturating_div(factor),
            fuel: self.fuel.saturating_div(factor),
        }
    }
    pub fn mul(&self, factor: u64) -> Resources {
        Resources {
            igt: self.igt.saturating_mul(factor),
            metal: self.metal.saturating_mul(factor),
            crystal: self.crystal.saturating_mul(factor),
            chemical: self.chemical.saturating_mul(factor),
            fuel: self.fuel.saturating_mul(factor),
        }
    }
    pub fn mint<'info>(
        &self,
        token_program: &Program<'info, Token>,
        (
            (mint_igt, bump_igt),
            (mint_metal, bump_metal),
            (mint_crystal, bump_crystal),
            (mint_chemical, bump_chemical),
            (mint_fuel, bump_fuel),
        ): (
            (&Account<'info, Mint>, u8),
            (&Account<'info, Mint>, u8),
            (&Account<'info, Mint>, u8),
            (&Account<'info, Mint>, u8),
            (&Account<'info, Mint>, u8),
        ),
        (account_igt, account_metal, account_crystal, account_chemical, account_fuel): (
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
        ),
    ) -> Result<()> {
        // Give some initial supplies
        process_mint_metal(
            token_program,
            (account_metal, (mint_metal, bump_metal), mint_metal),
            self.metal,
        )?;
        process_mint_crystal(
            token_program,
            (account_crystal, (mint_crystal, bump_crystal), mint_crystal),
            self.crystal,
        )?;
        process_mint_chemical(
            token_program,
            (
                account_chemical,
                (mint_chemical, bump_chemical),
                mint_chemical,
            ),
            self.chemical,
        )?;
        process_mint_fuel(
            token_program,
            (account_fuel, (mint_fuel, bump_fuel), mint_fuel),
            self.fuel,
        )?;
        process_mint_igt(
            token_program,
            (account_igt, (mint_igt, bump_igt), mint_igt),
            self.igt,
        )
    }
}

#[derive(InitSpace)]
#[account]
pub struct ResourceAuthority {}

pub fn burn_resources<'info>(
    costs: Resources,
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
                process_burn_resource(
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
                process_burn_resource(
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
                process_burn_resource(
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
                process_burn_resource(
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

pub fn process_burn_resource<'info>(
    token_program: &Program<'info, Token>,
    (from, mint, (authority, authority_bump)): (
        &Account<'info, TokenAccount>,
        &Account<'info, Mint>,
        (&Account<'info, ResourceAuthority>, u8),
    ),
    amount: u64,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[seeds::RESOURCE_AUTHORITY, &[authority_bump]]];
    let cpi_accounts = Burn {
        mint: mint.to_account_info(),
        from: from.to_account_info(),
        authority: authority.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
    token::burn(cpi_ctx, amount)
}
