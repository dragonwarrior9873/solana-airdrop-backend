use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token,
        associated_token,
    },
};

use crate::state::AirdropInfo;
use crate::constants::AIRDROP_SEED;
use crate::errors::AirdropError;

pub fn withdraw_token(
    ctx: Context<WithdrawToken>, 
    amount: u64,
    identifier: u8
) -> Result<()> {
    msg!("Transferring airdrop tokens {}...", identifier);
    msg!("Mint: {}", &ctx.accounts.mint_account.to_account_info().key());   
    msg!("From Token Address: {}", &ctx.accounts.from_associated_token_account.key());     
    msg!("To Token Address: {}", &ctx.accounts.to_associated_token_account.key());     
    
    let airdrop_info = &mut ctx.accounts.airdrop_info;
    let withdrwan_token = ctx.accounts.mint_account.key();
    let cur_timestamp = u64::try_from(Clock::get()?.unix_timestamp).unwrap();
    let bump = &[airdrop_info.bump];

    // if airdrop_info.end_time > cur_timestamp {
    //     msg!("Airdrop not ended yet.");
    //     return Err(AirdropError::AirdropNotEnded.into());
    // }

    if withdrwan_token == airdrop_info.token_mint_address {
        msg!("withdraw signer : {}", &ctx.accounts.airdrop_authority.key());

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.from_associated_token_account.to_account_info(),
                    to: ctx.accounts.to_associated_token_account.to_account_info(),
                    authority: airdrop_info.to_account_info(),
                },
                &[&[AIRDROP_SEED, ctx.accounts.airdrop_authority.key().as_ref(), [identifier].as_ref(), bump]],
            ),
            amount,
        )?;

        if airdrop_info.deposit_token_amount >= amount {
            airdrop_info.deposit_token_amount = airdrop_info.deposit_token_amount - amount;
        } else {
            airdrop_info.deposit_token_amount = 0;
        }

        msg!("Airdrop tokens withdrawn successfully.");
    }

    Ok(())
}


#[derive(Accounts)]
#[instruction(
    amount: u64,
    identifier: u8
)]
pub struct WithdrawToken<'info> {
    // airdrop token accounts
    #[account(mut)]
    pub mint_account: Box<Account<'info, token::Mint>>,
    pub airdrop_authority: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = airdrop_info,
    )]
    pub from_associated_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = airdrop_authority,
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