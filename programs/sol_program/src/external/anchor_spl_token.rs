use anchor_lang::prelude::*;
use solana_program::{
  instruction::{
    Instruction,
  },
  program::{
    invoke,
    invoke_signed,
  },
  program_pack::{
    Pack,
  },
};
use std::ops::{
  Deref,
};
use crate::external::spl_token;
use crate::external::spl_token::{
  ID as TOKEN_PROGRAM_ID,
};

#[derive(Clone)]
pub struct TokenAccount(spl_token::TokenAccount);

impl TokenAccount {
  pub const LEN: usize = spl_token::TokenAccount::LEN;
}

impl anchor_lang::AccountDeserialize for TokenAccount {
  fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
    spl_token::TokenAccount::unpack(buf)
      .map(TokenAccount)
      .map_err(Into::into)
  }
}

impl anchor_lang::AccountSerialize for TokenAccount {}

impl anchor_lang::Owner for TokenAccount {
  fn owner() -> Pubkey {
    spl_token::ID
  }
}

impl Deref for TokenAccount {
  type Target = spl_token::TokenAccount;

  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

#[derive(Clone)]
pub struct Mint(spl_token::Mint);

impl anchor_lang::AccountDeserialize for Mint {
  fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
    spl_token::Mint::unpack(buf)
      .map(Mint)
      .map_err(Into::into)
  }
}

impl anchor_lang::AccountSerialize for Mint {}

impl anchor_lang::Owner for Mint {
  fn owner() -> Pubkey {
    spl_token::ID
  }
}

impl Deref for Mint {
  type Target = spl_token::Mint;

  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct TransferTokenParams {
  pub instruction: u8,
  pub amount: u64,
}

pub fn transfer_token<'a>(
  owner: &AccountInfo<'a>,
  from_pubkey: &AccountInfo<'a>,
  to_pubkey: &AccountInfo<'a>,
  amount: u64,
  signer_seeds: &[&[&[u8]]],
) -> std::result::Result<(), ProgramError> {
  let data = TransferTokenParams {
    instruction: 3,
    amount,
  };
  let instruction = Instruction {
    program_id: TOKEN_PROGRAM_ID,
    accounts: vec![
      AccountMeta::new(*from_pubkey.key, false),
      AccountMeta::new(*to_pubkey.key, false),
      AccountMeta::new_readonly(*owner.key, true),
    ],
    data: data.try_to_vec().unwrap(),
  };
  if signer_seeds.len() == 0 {
    invoke(&instruction, &[from_pubkey.clone(), to_pubkey.clone(), owner.clone()])
  }
  else {
    invoke_signed(&instruction, &[from_pubkey.clone(), to_pubkey.clone(), owner.clone()], &signer_seeds)
  }
}


#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct MintToTokenParams {
    pub instruction: u8,
    pub amount: u64,
}

/// Creates a `MintTo` instruction.
pub fn mint_to<'info>(
  mint_pubkey: &AccountInfo<'info>,
  account_pubkey: &AccountInfo<'info>,
  owner_pubkey: &AccountInfo<'info>,
  signer_seeds: &[&[&[u8]]],
  amount: u64,
) -> std::result::Result<(), ProgramError> {
  let data = MintToTokenParams {
    instruction: 7,
    amount,
  };

  let instruction = Instruction {
    program_id: TOKEN_PROGRAM_ID,
    accounts: vec![
      AccountMeta::new(*mint_pubkey.key, false),
      AccountMeta::new(*account_pubkey.key, false),
      AccountMeta::new_readonly(*owner_pubkey.key, true),
    ],
    data: data.try_to_vec().unwrap(),
  };
  if signer_seeds.len() == 0 {
    invoke(&instruction, &[mint_pubkey.clone(), account_pubkey.clone(), owner_pubkey.clone()])
  }
  else {
    invoke_signed(&instruction, &[mint_pubkey.clone(), account_pubkey.clone(), owner_pubkey.clone()], &signer_seeds)
  }
}
