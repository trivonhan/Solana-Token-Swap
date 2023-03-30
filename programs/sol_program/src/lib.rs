pub mod constant;
pub mod context;
pub mod error;
pub mod event;
pub mod state;
pub mod external;

use anchor_lang::prelude::*;
use context::*;
use {
  num_traits::{CheckedShl, CheckedShr, PrimInt},
  std::cmp::{
    Ordering,
    min,
  }
};

use crate::{
    external::anchor_spl_token::{
    mint_to,
    transfer_token,
  },
  constant::*,
};

declare_id!("DxsrhkT7S2EqipZoSyQd9bmx5eLurU49cCZ4yDMHDN52");

#[program]
mod ez_token_swap {
  use super::*;

  pub fn initialize_pool(ctx: Context<InitPoolContext>, bump_seed: u8, pool_authority_bump_seed: u8) -> Result<()> {

    let pool_state = &mut ctx.accounts.pool_state;
    let pool_authority = &ctx.accounts.pool_authority;
    let token_a_ata = &ctx.accounts.token_a_ata;
    let token_b_ata = &ctx.accounts.token_b_ata;
    let pool_token_mint = &ctx.accounts.pool_token_mint;
    let token_a_mint = &ctx.accounts.token_a_mint;
    let token_b_mint = &ctx.accounts.token_b_mint;

    pool_state.is_initialized = true;
    pool_state.bump_seed = bump_seed;
    pool_state.pool_authority = *pool_authority.key;
    pool_state.pool_authority_bump_seed = pool_authority_bump_seed;
    pool_state.token_a = *token_a_ata.key;
    pool_state.token_b = *token_b_ata.key;
    pool_state.pool_mint = pool_token_mint.key();
    pool_state.token_a_mint = token_a_mint.key();
    pool_state.token_b_mint = token_b_mint.key();
    pool_state.total_supply = 0;
    pool_state.amount_a = 0;
    pool_state.amount_b = 0;

    msg!("Pool initialized {:?}", pool_state.pool_authority_bump_seed);

    Ok(())
  }

  pub fn deposit_liquidity(ctx: Context<DepositLiquidityContext>, amount_a: u64, amount_b: u64) -> Result<()> {

    let pool_state = &mut ctx.accounts.pool_state;
    let pool_token_mint = &ctx.accounts.pool_token_mint;
    let pool_token_ata = &ctx.accounts.pool_token_ata;

    let liquidity_provider = &ctx.accounts.liquidity_provider;
    let pool_authority = &ctx.accounts.pool_authority;

    let pool_token_a_ata = &ctx.accounts.pool_token_a_ata;
    let pool_token_b_ata = &ctx.accounts.pool_token_b_ata;

    let token_a_ata = &ctx.accounts.token_a_ata;
    let token_b_ata = &ctx.accounts.token_b_ata;

    let mut pool_amount_a = pool_state.amount_a;
    let mut pool_amount_b = pool_state.amount_b;

    let total_supply = pool_token_mint.supply;

    let mut lp_token_amount: u128 = 0;
    let amount_a_u128 = u128::from(amount_a);
    let amount_b_u128 = u128::from(amount_b);

    // Transfer token A to pool
    transfer_token(liquidity_provider, token_a_ata, pool_token_a_ata, amount_a, &[]).expect("Transfer token A to pool failed");

    // Transfer token B to pool
    transfer_token(liquidity_provider, token_b_ata, pool_token_b_ata, amount_b, &[]).expect("Transfer token B to pool failed");

    msg!("Pool total supply {:?}", total_supply);

    if pool_amount_a == 0 && pool_amount_b == 0 {
      pool_amount_a += amount_a as u64;
      pool_amount_b += amount_b as u64;
      lp_token_amount = sqrt(amount_a_u128.checked_mul(amount_b_u128).unwrap()).unwrap();
    } else {
      let lp_token_amount_a = (amount_a_u128.checked_mul(total_supply as u128).unwrap() as u128)
                                    .checked_div(pool_amount_a as u128).unwrap();
      let lp_token_amount_b = amount_b_u128.checked_mul(total_supply as u128).unwrap()
                                    .checked_div(pool_amount_b as u128).unwrap();
      lp_token_amount = min(lp_token_amount_a, lp_token_amount_b);
    }

    let signer_seeds: &[&[u8]] = &[
      &POOL,
      &pool_state.to_account_info().key.as_ref(),
      &AUTHORITY,
      &[pool_state.pool_authority_bump_seed],
    ];

    mint_to(
      &pool_token_mint.to_account_info(),
      &pool_token_ata.to_account_info(),
      &pool_authority.to_account_info(),
      &[signer_seeds],
      lp_token_amount as u64
    ).expect("Mint to failed");

    pool_state.amount_a += pool_amount_a;
    pool_state.amount_b += pool_amount_b;
    msg!("LP token amount: {:?}", lp_token_amount);

    Ok(())
  }

}


pub fn sqrt<T: PrimInt + CheckedShl + CheckedShr>(radicand: T) -> Option<T> {
  match radicand.cmp(&T::zero()) {
      Ordering::Less => return None,             // fail for less than 0
      Ordering::Equal => return Some(T::zero()), // do nothing for 0
      _ => {}
  }

  // Compute bit, the largest power of 4 <= n
  let max_shift: u32 = T::zero().leading_zeros() - 1;
  let shift: u32 = (max_shift - radicand.leading_zeros()) & !1;
  let mut bit = T::one().checked_shl(shift)?;

  let mut n = radicand;
  let mut result = T::zero();
  while bit != T::zero() {
      let result_with_bit = result.checked_add(&bit)?;
      if n >= result_with_bit {
          n = n.checked_sub(&result_with_bit)?;
          result = result.checked_shr(1)?.checked_add(&bit)?;
      } else {
          result = result.checked_shr(1)?;
      }
      bit = bit.checked_shr(2)?;
  }
  Some(result)
}
