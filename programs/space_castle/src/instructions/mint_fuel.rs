use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Burn, Mint, MintTo, Token, TokenAccount}, 
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, Metadata,
    },
};
use crate::{mint_decimals, resource::ResourceAuthority, seeds};

pub fn mint_init_fuel(ctx: Context<MintInitFuel>) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[seeds::MINT_FUEL, &[ctx.bumps.mint]]];
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
            name: "Fuel".to_string(),
            symbol: "rFUEL".to_string(),
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

pub fn mint_fuel(ctx: Context<MintFuel>, amount: u64) -> Result<()> {
    process_mint_fuel(
    &ctx.accounts.token_program,
    (
        &ctx.accounts.token_account,
        (&ctx.accounts.mint, ctx.bumps.mint),
        &ctx.accounts.mint
    ),
    amount
    )
}

pub fn process_mint_fuel<'info>(
    token_program: &Program<'info, Token>,
    (
        to,
        (mint, mint_bump),
        authority 
    ): (&Account<'info, TokenAccount>, (&Account<'info, Mint>, u8), &Account<'info, Mint>),
    amount: u64,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[seeds::MINT_FUEL, &[mint_bump]]];
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
        amount * 10u64.pow(mint_decimals::FUEL as u32),
    )
}

#[derive(Accounts)]
pub struct MintInitFuel<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init, 
        payer = payer, 
        seeds = [seeds::MINT_FUEL], 
        bump, 
        mint::decimals = 8,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
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
pub struct MintFuel<'info> {
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
        seeds = [seeds::MINT_FUEL],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [seeds::ACCOUNT_FUEL, payer.key().as_ref()],
        bump,
        token::mint = mint, 
        token::authority = resource_authority 
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
