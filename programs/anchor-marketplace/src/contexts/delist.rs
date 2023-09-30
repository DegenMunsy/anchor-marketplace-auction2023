use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer as SplTransfer, transfer as spl_transfer}, associated_token::AssociatedToken};
use crate::{state::Marketplace, state::Whitelist, state::Listing};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = maker_mint
    )]
    maker_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"auth", maker_mint.key().as_ref()],
        bump,
        token::authority = vault,
        token::mint = maker_mint
    )]
    vault: Account<'info, TokenAccount>,
    maker_mint: Account<'info, Mint>,
    collection_mint: Account<'info, Mint>,
    #[account(
        seeds = [marketplace.key().as_ref(), collection_mint.key().as_ref()],
        bump = whitelist.bump
    )]
    whitelist: Account<'info, Whitelist>,
    #[account(
        mut,
        close = maker,
        seeds = [whitelist.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump
    )]
    listing: Account<'info, Listing>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}
// add logic to check if highest bidder is above reserve price and time is expired
// pass in creator's address to reclaim the nft
// delisting refund highest bidder and send nft to the lister

impl<'info> Delist<'info> {
    pub fn withdraw_nft(&self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        
        if current_time < self.listing.expiry {
            return Err(MarketplaceError::ListingNotExpired.into());
        }

        // Check if highest bid is below the reserve price and send nft back to maker_ata or send nft to highest bidder
        let to;
        if self.listing.highest_bid < self.listing.reserve_price {
            to = self.maker_ata;  
        } else {
            to = self.listing.highest_bidder; 
            // Refund the highest bidder
            let refund_accounts = SplTransfer {
                from: self.vault.to_account_info(),
                to: self.listing.highest_bidder.to_account_info(),
                authority: self.vault.to_account_info()
            };
            let refund_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                refund_accounts,
                &signer_seeds // maybe just seeds?
            );
            spl_transfer(refund_ctx, self.listing.highest_bid)?;
        }

        // Transfer NFT
        let accounts = SplTransfer {
            from: self.vault.to_account_info(),
            to: to.to_account_info(),
            authority: self.vault.to_account_info()
        };

        let seeds = [b"auth", &self.maker_mint.key().to_bytes()[..], &[self.listing.auth_bump]];
        let signer_seeds = &[&seeds[..]][..];
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds
        );
        
        spl_transfer(ctx, 1)
    }
}
