use anchor_lang::prelude::*;

use crate::state::AirdropInfo;
use crate::constants::AIRDROP_SEED;

// Edit the details for a airdrop
#[allow(clippy::too_many_arguments)]
pub fn create_airdrop(
    ctx: Context<CreateAirdrop>,
    token_mint_address: Pubkey,
    amount: u64,
    end_time: u64,
    identifier: u8
) -> Result<()> {
    
    let airdrop_info = &mut ctx.accounts.airdrop_info;
    let authority = &ctx.accounts.authority;

    // Set the presale details to the parameters given
    airdrop_info.is_live = false;
    airdrop_info.token_mint_address = token_mint_address;
    airdrop_info.deposit_token_amount = 0;
    airdrop_info.end_time = end_time;
    airdrop_info.airdrop_token_amount = amount;
    airdrop_info.identifier = identifier;
    airdrop_info.authority = authority.key();
    airdrop_info.authority1 = authority.key();
    airdrop_info.bump =  ctx.bumps.airdrop_info;

    msg!(
        "Airdrop has created for token: {}",
        airdrop_info.token_mint_address
    );

    Ok(())
}

#[derive(Accounts)]
#[allow(clippy::too_many_arguments)]
#[instruction(
    token_mint_address: Pubkey,
    amount: u64,
    end_time: u64,
    identifier: u8
)]
pub struct CreateAirdrop<'info> {
    // Initialize the airdrop_info account
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<AirdropInfo>(),
        seeds = [AIRDROP_SEED.as_ref(), authority.key().as_ref(), [identifier].as_ref()],
        bump
    )]
    pub airdrop_info: Box<Account<'info, AirdropInfo>>,
    
    // Set the authority to the transaction signer
    #[account(mut)]
    pub authority: Signer<'info>,
    
    // Must be included when initializing an account
    pub system_program: Program<'info, System>,
}