use anchor_lang::prelude::*;

#[error_code]
pub enum SurvzError {
    SurveyIsClosed,
    InvalidSurveyInput,
    AllFieldMustBeAnswered,
    InvalidTime,
    InsufficientFunds
}