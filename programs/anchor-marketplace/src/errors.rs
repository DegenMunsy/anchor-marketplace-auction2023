use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError {
    #[msg("Invalid name")]
    InvalidName,
    #[msg("Invalid bump")]
    BumpError,
    #[msg("Collection not set")]
    CollectionNotSet,
    #[msg("Invalid collection")]
    InvalidCollection,
    #[msg("Listing not expired")]
    ListingNotExpired
}

#[error_code]
pub enum EscrowError {
    #[msg("Unable to get auth bump")]
    AuthBumpError,
    #[msg("Unable to get vault bump")]
    VaultBumpError,
    #[msg("Unable to get escrow bump")]
    EscrowBumpError,
    #[msg("Your expiration is too far into the future")]
    MaxExpiryExceeded,
    #[msg("Escrow has expired")]
    Expired,
}