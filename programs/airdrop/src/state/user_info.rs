use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserInfo {
    pub claim_amount: u64,
    // claim time
    pub claim_time: u64,
}