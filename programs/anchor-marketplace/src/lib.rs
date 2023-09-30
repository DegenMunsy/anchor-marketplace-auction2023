mod contexts;
use contexts::*;

mod state;
mod helpers;
mod errors;

use anchor_lang::prelude::*;

declare_id!("mktYdagPAAnuHigRD62zLpHshZqx7vpKHjN3fN6MPjy");

#[program]
pub mod anchor_marketplace {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, name, fee)
    }

    pub fn whitelist_collection(ctx: Context<WhitelistCollection>) -> Result<()> {
        ctx.accounts.whitelist(&ctx.bumps)
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(&ctx.bumps, price)?;
        ctx.accounts.deposit_nft()
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()
    }

    pub fn bid(ctx: Context<Bid>) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.send_nft()?;
        ctx.accounts.close_mint_ata()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.empty_vault()?;
        ctx.accounts.close_vault()
    }
}