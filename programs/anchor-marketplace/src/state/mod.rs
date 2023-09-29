use anchor_lang::prelude::*;

// TODO: add bumps
#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u16,
    pub bump: u8,
    pub treasury_bump: u8,
    pub name: String,
}


impl Marketplace {
    pub const LEN: usize = 8 + 32 + 2 + 1 + 1 + 4 + 32;
}

#[account]
pub struct Whitelist {
    pub bump: u8
}

impl Whitelist {
    pub const LEN: usize = 8 + 1;
}

// added expiry, highest_bid, Highest_bidder, reserve_price
#[account]
pub struct Listing {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub reserve_price: u64,
    pub bump: u8,
    pub auth_bump: u8,
    pub expiry: i64,
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
}

#[account]
pub struct Refund {
    
}

impl Listing {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1 + 1;
}