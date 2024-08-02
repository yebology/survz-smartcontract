use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use errors::*;
use instructions::*;
use state::*;

declare_id!("GVVMixtE5VECdKHJzaJSMgbFNjifwmJTLgb4skuwDeMK");

#[program]
pub mod smartcontract {
    use super::*;

    pub fn create_survey(
        ctx: Context<CreateSurvey>, 
        title: String, 
        description: String, 
        open_timestamp: u64, 
        close_timestamp: u64, 
        target_participant: u64, 
        reward_per_participant: u64, 
        question_list: [Vec<String>; 5]
    ) -> Result<()> {
        instructions::create_survey::handler(
            ctx, 
            title, 
            description, 
            open_timestamp, 
            close_timestamp, 
            target_participant, 
            reward_per_participant, 
            question_list
        );
        Ok(())
    }

    // pub fn fill_survey() -> Result<()> {
    //     Ok(())
    // }

    // pub fn change_survey_status() -> Result<()> {
    //     Ok(())
    // }
}
