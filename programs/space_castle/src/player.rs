use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Player {
    #[max_len(24)]
    pub name: String,
}

#[derive(Accounts)]
pub struct CreatePlayer<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"player", signer.key().as_ref()], 
        bump,
        space = 8 + Player::INIT_SPACE 
    )]
    pub player: Account<'info, Player>,
    pub system_program: Program<'info, System>,
}
