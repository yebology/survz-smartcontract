use anchor_lang::prelude::*;

#[event]
pub struct SurveyCreated {
    pub creator: Pubkey,
    pub survey_account: Pubkey,
}

#[event]
pub struct SurveyFilled {
    pub user: Pubkey,
    pub survey_account: Pubkey
}