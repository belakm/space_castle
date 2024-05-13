use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct ResourceAuthority {
    pub metal_mint_bump: u8,
}
