use anchor_lang::prelude::*;

use crate::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub enum SurvzState {
    Open,
    Closed
}

#[account(mut)]
pub struct Survey {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub creator: Pubkey,
    pub open_timestamp: u64,
    pub close_timestamp: u64,
    pub target_participant: u64,
    pub reward_per_participant: u64,
    pub question_list: [Vec<String>; 5],
    pub answer_list: [Vec<String>; 5]
}