use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer as SplTransfer, transfer as spl_transfer}, associated_token::AssociatedToken};
use crate::{state::Marketplace, state::Whitelist, state::Listing, errors::MarketplaceError};

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
    #[account(
        constraint = highest_bidder_ata.key() == listing.highest_bidder
    )]
    highest_bidder_ata: Account<'info, TokenAccount>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

impl<'info> Delist<'info> {
    pub fn withdraw_nft(&self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;

        if current_time < self.listing.expiry {
            return Err(MarketplaceError::ListingNotExpired.into());
        }

        // Prepare the seeds for signing.
        let seeds = [b"auth", &self.maker_mint.key().to_bytes()[..], &[self.listing.auth_bump]];
        let signer_seeds = &[&seeds[..]][..];

        // Define local variables to hold AccountInfo values.
        let maker_account_info = self.maker_ata.to_account_info();

        if self.listing.highest_bid < self.listing.reserve_price {
            // Transfer NFT directly to maker
            let accounts = SplTransfer {
                from: self.vault.to_account_info(),
                to: maker_account_info,
                authority: self.vault.to_account_info()
            };

            let ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                accounts,
                &signer_seeds
            );
            
            spl_transfer(ctx, 1)?;
        } else {
            // First use of bidder_account_info for refunding the highest bidder
            let bidder_account_info = self.highest_bidder_ata.to_account_info();
            let refund_accounts = SplTransfer {
                from: self.vault.to_account_info(),
                to: bidder_account_info,
                authority: self.vault.to_account_info()
            };
            let refund_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                refund_accounts,
                &signer_seeds
            );
            spl_transfer(refund_ctx, self.listing.highest_bid)?;

            // Re-create bidder_account_info for the second use
            let bidder_account_info = self.highest_bidder_ata.to_account_info();
            // Transfer NFT to highest bidder
            let accounts = SplTransfer {
                from: self.vault.to_account_info(),
                to: bidder_account_info,
                authority: self.vault.to_account_info()
            };

            let ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                accounts,
                &signer_seeds
            );
            
            spl_transfer(ctx, 1)?;
        }

        Ok(())
    }
}
