use anchor_lang::prelude::*;

#[account]
pub struct Answer {
    pub user: Pubkey,
    pub survey_id: u64,
    pub timestamp: u64,
    pub answer_list: [String; 5]
}

impl Answer {
    const USER_SPACE : usize = 32;
    const SURVEY_ID_SPACE : usize = 8;
    const ANSWER_LIST_SPACE : usize = 4 + (200 * 5);

    pub const MAXIMUM_SIZE : usize = 
        Self::USER_SPACE + 
        Self::SURVEY_ID_SPACE + 
        Self::ANSWER_LIST_SPACE;
}