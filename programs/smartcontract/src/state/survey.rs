use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub enum SurvzState {
    Open,
    Closed
}

#[account]
pub struct Survey {
    pub title: String, // 4 + 150 (maximum 100 bytes + 50 bytes other for safety)
    pub description: String, // 4 + 300 (maximum 256 bytes + 44 bytes other for safety)
    pub creator: Pubkey, // 32
    pub open_timestamp: u64, // 8
    pub close_timestamp: u64, // 8
    pub target_participant: u64, // 8
    pub reward_per_participant: u64, // 8
    pub balance_deposited: u64, // 8
    pub state: SurvzState, // 1
    pub question_list: [Vec<String>; 5], // 4 + ((256 + 44) * 5) // maximum 256 bytes/question + 44 bytes other for safety
}

impl Survey {
    pub const MAXIMUM_SIZE : usize = 8 + (4 + 150) + (4 + 300) + 32 + 8 + 8 + 8 + 8 + 8 + 1 + (4 + ((256 + 44) * 5));
}