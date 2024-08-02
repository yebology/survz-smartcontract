use anchor_lang::prelude::*;

#[error_code]
pub enum SurvzError {
    SurveyNotStarted,
    AllFieldMustBeFilled,
    AmountMustBeGreaterThanZero,
    ParticipantTargetMustBeGreaterThanZero
}