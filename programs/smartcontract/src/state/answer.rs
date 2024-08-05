use anchor_lang::prelude::*;

#[account]
pub struct Answer {
    pub user: Pubkey, // 32
    pub survey_id: u64, // 8
    pub answer_list: Vec<String>, // 4 + ((256 + 44) * 5) // maximum 256 bytes/question + 44 bytes other for safety
}

impl Answer {
    pub const MAXIMUM_SIZE : usize = 8 + 32 + 8 + (4 + ((256 + 44) * 5));
}