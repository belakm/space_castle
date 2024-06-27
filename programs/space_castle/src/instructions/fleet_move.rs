use crate::{
    fleet::{Fleet, FleetErrorCode},
    resource::{process_burn_resource, ResourceAuthority},
    seeds,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn fleet_move(
    ctx: Context<FleetMove>,
    x: u16,
    y: u16,
    move_to_x: u16,
    move_to_y: u16,
) -> Result<()> {
    let fleet_to = &mut ctx.accounts.fleet_to;
    let fleet_from = &mut ctx.accounts.fleet_from;
    fleet_to.replace_with_another_fleet(fleet_from);
    fleet_from.reset();
    process_burn_resource(
        &ctx.accounts.token_program,
        (
            &ctx.accounts.account_fuel,
            &ctx.accounts.mint_fuel,
            (
                &ctx.accounts.resource_authority,
                ctx.bumps.resource_authority,
            ),
        ),
        ctx.accounts.fleet_to.get_move_quote((x, y), (move_to_x, move_to_y)),
    )
}

#[derive(Accounts)]
#[instruction(x: u16, y: u16, move_to_x: u16, move_to_y: u16)]
pub struct FleetMove<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // From
    #[account(
        mut,
        seeds = [
            seeds::FLEET, 
            x.to_le_bytes().as_ref(), 
            y.to_le_bytes().as_ref(), 
        ], 
        bump,
        constraint = fleet_from.is_present() @ FleetErrorCode::FleetNotPresent,
        constraint = fleet_from.is_owned_by(&signer.key()) @ FleetErrorCode::NoAuthority,
    )]
    pub fleet_from: Account<'info, Fleet>,
    // To
    #[account(
        init_if_needed,
        seeds = [
            seeds::FLEET,
            move_to_x.to_le_bytes().as_ref(),
            move_to_y.to_le_bytes().as_ref()
        ],
        bump,
        space = 8 + Fleet::INIT_SPACE,
        payer = signer,
        constraint = !fleet_to.is_present() @ FleetErrorCode::IllegalMoveAlreadyOccupied,
    )]
    pub fleet_to: Account<'info, Fleet>,
    // Resource authority
    #[account(mut, seeds = [seeds::RESOURCE_AUTHORITY], bump)]
    pub resource_authority: Account<'info, ResourceAuthority>,
    #[account(mut, seeds = [seeds::MINT_FUEL], bump)]
    pub mint_fuel: Account<'info, Mint>,
    #[account(mut, seeds = [seeds::ACCOUNT_FUEL, signer.key().as_ref()], bump)]
    pub account_fuel: Account<'info, TokenAccount>,
    // Programs
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
