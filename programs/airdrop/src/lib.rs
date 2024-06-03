use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("DATFQZy5MsVSwgj7TUdwRQdNCxsfGJfMvi9n3BP2zmac");

#[program]
pub mod airdrop {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Reference to the airdrop account from the Initialize struct
        let airdrop = &ctx.accounts.airdrop;
        msg!("Airdrop account created! Current token amount: {}", airdrop.token_amount);
        Ok(())
    }
    
    pub fn create_airdrop(ctx: Context<CreateAirdrop>,
        token_mint_address: Pubkey,
        amount: u64,
        end_time: u64,
        identifier: u8
    ) -> Result<()> {
        return create_airdrop::create_airdrop(
            ctx,
            token_mint_address,
            amount,
            end_time,
            identifier
        );
    }

    // pub fn start_airdrop(
    //     ctx: Context<StartAirdrop>,
    //     start_time: u64,
    //     identifier: u8,
    // ) -> Result<()> {
    //     return start_airdrop::start_airdrop(
    //         ctx,
    //         start_time,
    //         identifier
    //     );
    // }

    pub fn deposit_token(
        ctx: Context<DepositToken>,
        amount: u64,
        identifier: u8
    ) -> Result<()> {
        return deposit_token::deposit_token(
            ctx,
            amount,
            identifier
        );
    }


    pub fn claim_token(
        ctx: Context<ClaimToken>,
        identifier: u8
    ) -> Result<()> {
        return claim_token::claim_token(
            ctx,
            identifier
        );
    }

    pub fn withdraw_token(
        ctx: Context<WithdrawToken>,
        amount: u64,
        identifier: u8
    ) -> Result<()> {
        return withdraw_token::withdraw_token(
            ctx,
            amount,
            identifier
        );
    }

    // Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // the account paying to create the airdrop account
    #[account(mut)]
    pub user: Signer<'info>,    // specific account must be signer on the transaaction

   // The airdrop account being created and initialized in the instruction
    #[account(
        init,         // specifies we are creating this account
        payer = user, // specifies account paying for the creation of the account
        space = 8 + 8 // space allocated to the new account (8 byte discriminator + 8 byte for u64)
    )]
    pub airdrop: Account<'info, Airdrop>, // specify account is 'Airdrop' type
    pub system_program: Program<'info, System>, // specify account must be System Program  
}
// Define structure of `Counter` account
#[account]
pub struct Airdrop {
    pub token_amount: u64, // define amount value type as u64
}