use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum SurvzState {
    Open,
    Closed
}

#[account]
pub struct Survey {
    pub id: u64, 
    pub title: String, 
    pub description: String,
    pub creator: Pubkey, 
    pub open_timestamp: u64, 
    pub close_timestamp: u64,
    pub current_participant: u64,
    pub target_participant: u64,
    pub total_reward: u64,
    pub reward_per_participant: u64,
    pub state: SurvzState, 
    pub question_list: [String; 5]
}

impl Survey {
    const DISCRIMINATOR_SPACE : usize = 8;
    const ID_SPACE : usize = 8;
    const TITLE_SPACE : usize = 4 + 64;
    const DESCRIPTION_SPACE : usize = 4 + 200;
    const CREATOR_SPACE : usize = 32;
    const OPEN_TIMESTAMP_SPACE : usize = 8;
    const CLOSE_TIMESTAMP_SPACE : usize = 8;
    const CURRENT_PARTICIPANT_SPACE : usize = 8;
    const TARGET_PARTICIPANT_SPACE : usize = 8;
    const TOTAL_REWARD_SPACE : usize = 8;
    const REWARD_PER_PARTICIPANT_SPACE : usize = 8;
    const STATE_SPACE : usize = 1;
    const QUESTION_LIST_SPACE : usize = 4 + (200 * 5);

    pub const MAXIMUM_SIZE : usize = 
        Self::DISCRIMINATOR_SPACE + 
        Self::ID_SPACE + 
        Self::TITLE_SPACE + 
        Self::DESCRIPTION_SPACE + 
        Self::CREATOR_SPACE + 
        Self::OPEN_TIMESTAMP_SPACE + 
        Self::CLOSE_TIMESTAMP_SPACE + 
        Self::CURRENT_PARTICIPANT_SPACE + 
        Self::TARGET_PARTICIPANT_SPACE + 
        Self::TOTAL_REWARD_SPACE + 
        Self::REWARD_PER_PARTICIPANT_SPACE + 
        Self::STATE_SPACE  +
        Self::QUESTION_LIST_SPACE;
}