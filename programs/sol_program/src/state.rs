use anchor_lang::prelude::*;

#[account]
pub struct PoolState {
    // Initialized state.
    pub is_initialized: bool,
    /// Bump seed used in program address.
    /// The program address is created deterministically with the bump seed,
    /// swap program id, and swap account pubkey.  This program address has
    /// authority over the swap's token A account, token B account, and pool
    /// token mint.
    pub bump_seed: u8,

    pub pool_authority: Pubkey,

    pub pool_authority_bump_seed: u8,

    /// Token A
    pub token_a: Pubkey,
    /// Token B
    pub token_b: Pubkey,

    /// Pool tokens are issued when A or B tokens are deposited.
    /// Pool tokens can be withdrawn back to the original A or B token.
    pub pool_mint: Pubkey,

    /// Mint information for token A
    pub token_a_mint: Pubkey,
    /// Mint information for token B
    pub token_b_mint: Pubkey,

    // Total LP supply
    pub total_supply: u64,

    // Amount token A
    pub amount_a: u64,

    // Amount token B
    pub amount_b: u64,

}

impl PoolState {
    pub fn size() -> usize {
        1 + 1 + 32 + 32 + 32 + 32 + 32 + 32 + 32
    }
}
