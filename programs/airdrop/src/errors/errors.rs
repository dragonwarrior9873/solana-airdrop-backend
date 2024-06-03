use anchor_lang::prelude::*;

// Not yet implemented

#[error_code]
pub enum AirdropError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Over hardcap amount.")]
    Overhardcap,
    #[msg("Not allowed")]
    NotAllowed,
    #[msg("Not allowed tokens.")]
    NotAllowedToken,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Already marked")]
    AlreadyMarked,
    #[msg("Airdop not started yet")]
    AirdropNotStarted,
    #[msg("Airdop already ended")]
    AirdropEnded,
    #[msg("Token amount mismatch")]
    TokenAmountMismatch,
    #[msg("Insufficient Tokens")]
    InsufficientFund,
    #[msg("Airdrop not ended yet")]
    AirdropNotEnded
}