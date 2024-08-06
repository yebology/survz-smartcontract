use anchor_lang::prelude::*;

#[error_code]
pub enum SurvzError {
    #[msg("Survey is closed.")]
    SurveyIsClosed,
    #[msg("Invalid input.")]
    InvalidSurveyInput,
    #[msg("All fields must be answered.")]
    AllFieldMustBeAnswered,
    #[msg("Invalid open or close time.")]
    InvalidTime,
    #[msg("Insufficient funds.")]
    InsufficientFunds
}
