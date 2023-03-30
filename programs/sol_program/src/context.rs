use anchor_lang::prelude::*;
use solana_program::program_option::COption;

use crate::{
  constant::*,
  state::{
    PoolState,
  },
  external::{
    anchor_spl_token::{
      Mint,
      TokenAccount
    }
  }
};

use crate::error::ErrorCode::*;


#[derive(Accounts)]
pub struct HelloContext<'info> {

  /// CHECK: For warning removal
  pub owner: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct InitPoolContext<'info> {

  /// CHECK: Owner of the pool
  #[account(mut)]
  pub owner: Signer<'info>,

  /// CHECK: Pool state
  #[account(
    init,
    seeds=[
      &POOL,
      token_a_mint.key().as_ref(),
      token_b_mint.key().as_ref(),
    ],
    bump,
    payer = owner,
    space = 8 + PoolState::size(),
  )]
  pub pool_state: Account<'info, PoolState>,

  /// CHECK: Pool authority
  #[account(
    seeds=[
      &POOL,
      pool_state.to_account_info().key.as_ref(),
      &AUTHORITY,
    ],
    bump,
  )]
  pub pool_authority: AccountInfo<'info>,

  /// CHECK: Token A associated token account
  #[account(mut)]
  pub token_a_ata: AccountInfo<'info>,

  /// CHECK: Token B associated token account
  #[account(mut)]
  pub token_b_ata: AccountInfo<'info>,

  /// CHECK: Pool token mint
  #[account(
    mut,
    constraint = pool_token_mint.mint_authority == COption::Some(*pool_authority.key) @InvalidMint,
  )]
  pub pool_token_mint: Account<'info, Mint>,

  /// CHECK: Token A mint
  #[account(mut)]
  pub token_a_mint: Account<'info, Mint>,

  /// CHECK: Token B mint
  #[account(mut)]
  pub token_b_mint: Account<'info, Mint>,

  pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct DepositLiquidityContext<'info> {

    /// CHECK: Liquidity provider
    #[account(mut)]
    pub liquidity_provider: Signer<'info>,

    /// CHECK: Pool state
    #[account(
      mut,
      seeds=[
        &POOL,
        token_a_mint.key().as_ref(),
        token_b_mint.key().as_ref(),
      ],
      bump,
    )]
    pub pool_state: Account<'info, PoolState>,

    /// CHECK: Pool authority
    #[account(
      seeds=[
        &POOL,
        pool_state.to_account_info().key.as_ref(),
        &AUTHORITY,
      ],
      bump,
    )]
    pub pool_authority: AccountInfo<'info>,

    /// CHECK: Associated token account of A of user
    #[account(mut)]
    pub token_a_ata: AccountInfo<'info>,

    /// CHECK: Associated token account of A of user
    #[account(mut)]
    pub token_b_ata: AccountInfo<'info>,

    /// CHECK: Pool token associated token account of user
    #[account(mut)]
    pub pool_token_ata: AccountInfo<'info>,

    /// CHECK: Token A associated token account of pool
    #[account(
      mut,
      constraint = pool_token_a_ata.key == &pool_state.token_a @InvalidTokenAccount,
    )]
    pub pool_token_a_ata: AccountInfo<'info>,

    /// CHECK: Token B associated token account of pool
    #[account(
      mut,
      constraint = pool_token_b_ata.key == &pool_state.token_b @InvalidTokenAccount,
    )]
    pub pool_token_b_ata: AccountInfo<'info>,

    /// CHECK: Pool token mint
    #[account(mut,
      constraint = pool_token_mint.mint_authority == COption::Some(*pool_authority.key) @InvalidMint,
    )]
    pub pool_token_mint: Account<'info, Mint>,

    /// CHECK: Token A mint
    #[account(mut)]
    pub token_a_mint: Account<'info, Mint>,

    /// CHECK: Token B mint
    #[account(mut)]
    pub token_b_mint: Account<'info, Mint>,

    /// CHECK: Token program Id
    // #[account(mut)]
    pub token_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

