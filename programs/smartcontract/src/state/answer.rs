use anchor_lang::prelude::*;

#[account]
pub struct Answer {
    pub survey: Pubkey, // 32
    pub user: Pubkey, // 32
    pub answer_list: [Vec<String>; 5], // 4 + ((256 + 44) * 5) // maximum 256 bytes/question + 44 bytes other for safety
}

impl Answer {
    pub const MAXIMUM_SIZE : usize = 8 + 32 + 32 + (4 + ((256 + 44) * 5));
}