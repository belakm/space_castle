use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Mint, MintTo, Token, TokenAccount}, 
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, Metadata,
    },
};

use crate::{mint::MintAuthority, seeds};

pub fn mint_init_chemical(ctx: Context<MintInitChemical>) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[seeds::MINT_CHEMICAL, &[ctx.bumps.mint]]];
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.mint.to_account_info(),
                update_authority: ctx.accounts.mint.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        DataV2 {
            name: "Chemicals".to_string(),
            symbol: "rCHEM".to_string(),
            uri: "https://not-really.com".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true, // is_mutable
        true, // update_authority_is_signer
        None, // collection_details
    )?;
    Ok(())
}

pub fn mint_chemicals(ctx: Context<MintChemicals>, amount: u64) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[seeds::MINT_CHEMICAL, &[ctx.bumps.mint]]];
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
        amount * 10u64.pow(ctx.accounts.mint.decimals as u32),
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct MintInitChemical<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init, 
        payer = payer, 
        seeds = [seeds::MINT_CHEMICAL], 
        bump, 
        mint::decimals = 6,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        seeds = [seeds::MINT_CHEMICAL_AUTH],
        bump,
        space = 8 + MintAuthority::INIT_SPACE
    )]
    pub mint_authority: Account<'info, MintAuthority>,
    /// CHECK: Validate with constraint, also checked by metadata program
    #[account(
        mut,
    )]
    pub metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintChemicals<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [seeds::MINT_CHEMICAL_AUTH],
        bump
    )]
    pub mint_authority: Account<'info, MintAuthority>,
    #[account(
        mut,
        seeds = [seeds::MINT_CHEMICAL],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [seeds::ACCOUNT_CHEMICAL, payer.key().as_ref()],
        bump,
        token::mint = mint, 
        token::authority = mint_authority 
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

