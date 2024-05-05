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

use crate::seeds;

pub fn mint_init_igt(ctx: Context<MintInitIGT>) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[seeds::MINT_IGT, &[ctx.bumps.mint]]];
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
            name: "Intergalactic Tender".to_string(),
            symbol: "IGT".to_string(),
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

pub fn mint_igt(ctx: Context<MintIGT>, amount: u64) -> Result<()> {
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
        amount * 10u64.pow(ctx.accounts.mint.decimals as u32),
    )
}

#[derive(Accounts)]
pub struct MintInitIGT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init, 
        payer = payer, 
        seeds = [seeds::MINT_IGT], 
        bump, 
        mint::decimals = 8,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
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
pub struct MintIGT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [seeds::MINT_IGT],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer 
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
