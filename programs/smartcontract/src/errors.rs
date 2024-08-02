use anchor_lang::prelude::*;

#[error_code]
pub enum SurvzError {
    SurveyNotStarted,
    InvalidSurveyInput,
    AllFieldMustBeAnswered,
    InvalidTime,
    AlreadyFillThisSurvey
}