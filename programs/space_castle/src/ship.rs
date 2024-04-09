use anchor_lang::prelude::*;

#[account]
pub struct Ship {
    pub owner: Pubkey,
    pub ship_type: ShipType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum ShipType {
    Warship,
    MiningShip,
}
