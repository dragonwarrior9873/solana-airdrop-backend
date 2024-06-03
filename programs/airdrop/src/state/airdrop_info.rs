use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct AirdropInfo {
  // Mint address of the airdrop token
  pub token_mint_address: Pubkey,
  // Total amount of airdrop tokens available in the airdrop
  pub deposit_token_amount: u64,
  // Total amount of airdrop tokens sold during the airdrop
  pub airdrop_token_amount: u64,
  // End time of airdrop
  pub end_time: u64,
  // Airdrop is available
  pub is_live: bool,
  // Identifier for finding the PDA
  pub identifier: u8,
  // Authority of the airdrop
  pub authority: Pubkey,
  // Authority of the airdrop
  pub authority1: Pubkey,
  pub bump: u8
}