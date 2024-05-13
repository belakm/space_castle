use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, Metadata,
    }, 
    token::{self, Burn, Mint, MintTo, Token, TokenAccount},
};

use crate::{resource::ResourceAuthority, seeds};

pub fn mint_init_metal(ctx: Context<MintInitMetal>) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[seeds::MINT_METAL, &[ctx.bumps.mint]]];
    let ra = &mut ctx.accounts.resource_authority;
    ra.metal_mint_bump = ctx.bumps.mint;

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
            name: "Metal".to_string(),
            symbol: "rMETL".to_string(),
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

pub fn mint_metal(ctx: Context<MintMetal>, amount: u64) -> Result<()> {
    process_mint_metal(
        &ctx.accounts.token_program,
        (
            &ctx.accounts.token_account,
            (&ctx.accounts.mint, ctx.bumps.mint),
            &ctx.accounts.mint
        ),
        amount,
        ctx.accounts.mint.decimals
    )
}

pub fn process_mint_metal<'info>(
    token_program: &Program<'info, Token>,
    (
        to,
        (mint, mint_bump),
        authority 
    ): (&Account<'info, TokenAccount>, (&Account<'info, Mint>, u8), &Account<'info, Mint>),
    amount: u64,
    decimals: u8 
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[seeds::MINT_METAL, &[mint_bump]]];
    token::mint_to(
        CpiContext::new(
            token_program.to_account_info(),
            MintTo {
                to: to.to_account_info(),
                mint: mint.to_account_info(),
                authority: authority.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        amount * 10u64.pow(decimals as u32),
    )
}

pub fn process_burn_metal<'info>(
    token_program: &Program<'info, Token>,
    (
        from,
        (mint, mint_bump),
        authority 
    ): (&Account<'info, TokenAccount>, (&Account<'info, Mint>, u8), &Account<'info, Mint>),
    amount: u64
) -> Result<()>{
    token::burn(CpiContext::new_with_signer(token_program.to_account_info(), Burn {
        mint: mint.to_account_info(),
        from: from.to_account_info(),
        authority: authority.to_account_info(),
    }, &[&[seeds::MINT_METAL, &[mint_bump]]]), amount)
}

#[derive(Accounts)]
pub struct MintInitMetal<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init, 
        payer = payer, 
        seeds = [seeds::MINT_METAL], 
        bump, 
        mint::decimals = 8,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        seeds = [seeds::RESOURCE_AUTHORITY],
        bump,
        space = 8 + ResourceAuthority::INIT_SPACE
    )]
    pub resource_authority: Account<'info, ResourceAuthority>,
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
pub struct MintMetal<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [seeds::RESOURCE_AUTHORITY],
        bump
    )]
    pub resource_authority: Account<'info, ResourceAuthority>,
    #[account(
        mut,
        seeds = [seeds::MINT_METAL],
        bump,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [seeds::ACCOUNT_METAL, payer.key().as_ref()],
        bump,
        token::mint = mint, 
        token::authority = resource_authority 
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
