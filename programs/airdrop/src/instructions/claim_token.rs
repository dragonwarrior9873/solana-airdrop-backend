use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token,
        associated_token,
    },
};

use crate::errors::AirdropError;
use crate::state::{AirdropInfo, UserInfo};
use crate::constants::{AIRDROP_SEED, USER_SEED};

pub fn claim_token(
    ctx: Context<ClaimToken>, 
    identifier: u8
) -> Result<()> {

    let airdrop_info: &mut Box<Account<AirdropInfo>> = &mut ctx.accounts.airdrop_info;
    let bump = &[airdrop_info.bump];

    let cur_timestamp = u64::try_from(Clock::get()?.unix_timestamp).unwrap();

    // get time and compare with end time
    // if airdrop_info.end_time > cur_timestamp {
    //     msg!("Airdrop not ended yet.");
    //     return Err(AirdropError::AirdropNotEnded.into());
    // }

    let user_info = &mut ctx.accounts.user_info;
    let claim_amount = 100;
    user_info.claim_amount = user_info.claim_amount + claim_amount;
    airdrop_info.deposit_token_amount -= claim_amount;

    msg!("Transferring airdrop tokens to claimer {}...", &ctx.accounts.claimer.key());
    msg!("Mint: {}", &ctx.accounts.mint_account.to_account_info().key());   
    msg!("From Token Address: {}", &ctx.accounts.deposited_token_ata.key());     
    msg!("To Token Address: {}", &ctx.accounts.claimer_ata.key());     
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.deposited_token_ata.to_account_info(),
                to: ctx.accounts.claimer_ata.to_account_info(),
                authority: ctx.accounts.airdrop_info.to_account_info(),
            },
            &[&[AIRDROP_SEED, ctx.accounts.airdrop_authority.key().as_ref(), [identifier].as_ref(), bump]],
        ),
        claim_amount,
    )?;

    msg!("Airdrop tokens transferred successfully.");

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    identifier: u8
)]
pub struct ClaimToken<'info> {
    // Airdrop token accounts
    #[account(mut)]
    pub mint_account: Box<Account<'info, token::Mint>>,
    pub airdrop_authority: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = claimer,
        associated_token::mint = mint_account,
        associated_token::authority = airdrop_info,
    )]
    pub deposited_token_ata: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init_if_needed,
        payer = claimer,
        associated_token::mint = mint_account,
        associated_token::authority = claimer,
    )]
    pub claimer_ata: Box<Account<'info, token::TokenAccount>>,
    
    #[account(
        init,
        payer = claimer,
        space = 8 + std::mem::size_of::<UserInfo>(),
        seeds = [USER_SEED, claimer.key().as_ref(), [identifier].as_ref()],
        bump
    )]
    pub user_info: Box<Account<'info, UserInfo>>,
    #[account(
        mut,
        seeds = [AIRDROP_SEED, airdrop_authority.key().as_ref(), [identifier].as_ref()],
        bump
    )]
    pub airdrop_info: Box<Account<'info, AirdropInfo>>,
    #[account(mut)]
    pub claimer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}