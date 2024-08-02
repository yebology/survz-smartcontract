use anchor_lang::prelude::*;

use crate::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub enum SurvzState {
    Open,
    Closed
}

#[account]
pub struct SurveyAmount {
    pub amount: u64, // 8
}

#[account]
pub struct Survey {
    pub id: u64, // 8
    pub title: Vec<u8>, // 4 + 1
    pub description: Vec<u8>, // 4 + 1
    pub creator: Pubkey, // 32
    pub open_timestamp: u64, // 8
    pub close_timestamp: u64, // 8
    pub target_participant: u64, // 8
    pub reward_per_participant: u64, // 8
    pub question_list: [Vec<u8>; 5], // 4 + (1 * 5)
    pub answer_list: [Vec<u8>; 5] // 4 + (1 * 5)
}

impl Survey {
    pub const MAXIMUM_SIZE : usize = 8 + (0) + (0) + 32 + 8 + 8 + 8 + 8 + (4) + (4);
}