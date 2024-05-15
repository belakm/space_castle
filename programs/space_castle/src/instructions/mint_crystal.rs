use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Burn, Mint, MintTo, Token, TokenAccount}, 
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, Metadata,
    },
};
use crate::{resource::ResourceAuthority, seeds};

pub fn mint_init_crystal(ctx: Context<MintInitCrystal>) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[seeds::MINT_CRYSTAL, &[ctx.bumps.mint]]];
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
            name: "Crystal".to_string(),
            symbol: "rCRYS".to_string(),
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

pub fn mint_crystal(ctx: Context<MintCrystal>, amount: u64) -> Result<()> {
    process_mint_crystal(
    &ctx.accounts.token_program,
    (
        &ctx.accounts.token_account,
        (&ctx.accounts.mint, ctx.bumps.mint),
        &ctx.accounts.mint
    ),
    amount,
    ctx.accounts.mint.decimals)
}

pub fn process_mint_crystal<'info>(
    token_program: &Program<'info, Token>,
    (
        to,
        (mint, mint_bump),
        authority 
    ): (&Account<'info, TokenAccount>, (&Account<'info, Mint>, u8), &Account<'info, Mint>),
    amount: u64,
    decimals: u8 
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[seeds::MINT_CRYSTAL, &[mint_bump]]];
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

pub fn process_burn_crystal<'info>(
    token_program: &Program<'info, Token>,
    (
        from,
        mint,
        (authority, authority_bump)
    ): (&Account<'info, TokenAccount>, &Account<'info, Mint>, (&Account<'info, ResourceAuthority>, u8)),
    amount: u64
) -> Result<()>{
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

#[derive(Accounts)]
pub struct MintInitCrystal<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init, 
        payer = payer, 
        seeds = [seeds::MINT_CRYSTAL], 
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
pub struct MintCrystal<'info> {
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
        seeds = [seeds::MINT_CRYSTAL],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [seeds::ACCOUNT_CRYSTAL, payer.key().as_ref()],
        bump,
        token::mint = mint, 
        token::authority = resource_authority 
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

