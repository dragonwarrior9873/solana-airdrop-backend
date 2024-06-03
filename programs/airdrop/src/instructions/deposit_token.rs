use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token,
        associated_token,
    },
};

use crate::state::AirdropInfo;
// use crate::state::UserInfo;
use crate::constants::{AIRDROP_SEED, USER_SEED};
use crate::errors::AirdropError;

pub fn deposit_token(
    ctx: Context<DepositToken>,
    amount: u64,
    identifier: u8,
) -> Result<()> {

    msg!("Depositing airdrop tokens to airdrop {}...", identifier);
    msg!("Mint: {}", &ctx.accounts.mint_account.to_account_info().key());   
    msg!("From Token Address: {}", &ctx.accounts.from_associated_token_account.key());     
    msg!("To Token Address: {}", &ctx.accounts.to_associated_token_account.key()); 
    
    let airdrop_info = &mut ctx.accounts.airdrop_info;
    let deposit_token_address = ctx.accounts.mint_account.key();
    let cur_timestamp = u64::try_from(Clock::get()?.unix_timestamp).unwrap();

    if deposit_token_address == airdrop_info.token_mint_address {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.from_associated_token_account.to_account_info(),
                    to: ctx.accounts.to_associated_token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            amount,
        )?;
        airdrop_info.deposit_token_amount = airdrop_info.deposit_token_amount + amount;
        msg!("Tokens deposited successfully.");

        return Ok(());
    }

    if airdrop_info.end_time < cur_timestamp {
        msg!("Airdrop already ended.");
        return Err(AirdropError::AirdropEnded.into())
    }

    let token_amount = 0;

    if token_amount > airdrop_info.deposit_token_amount - airdrop_info.airdrop_token_amount {
        msg!("Insufficient tokens in airdrop");
        return Err(AirdropError::InsufficientFund.into())
    }

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.from_associated_token_account.to_account_info(),
                to: ctx.accounts.to_associated_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        amount,
    )?;

    msg!("Tokens deposited successfully.");

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    amount: u64,
    identifier: u8,
)]
pub struct DepositToken<'info> {
    #[account(mut)]
    pub mint_account: Box<Account<'info, token::Mint>>,
    pub airdrop_authority: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = airdrop_authority,
    )]
    pub from_associated_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = airdrop_info,
    )]
    pub to_associated_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(
        mut,
        seeds = [AIRDROP_SEED, airdrop_authority.key().as_ref(), [identifier].as_ref()],
        bump
    )]
    pub airdrop_info: Box<Account<'info, AirdropInfo>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}