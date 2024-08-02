use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
pub struct CreateSurvey<'info> {
    #[account(
        init,
        payer=user,
        space=Survey::MAXIMUM_SIZE,
        seeds=[b"create_survey", user.key().as_ref()],
        bump
    )]
    pub survey: Account<'info, Survey>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn handler(
    ctx: Context<CreateSurvey>, 
    title: String, 
    description: String, 
    open_timestamp: u64, 
    close_timestamp: u64, 
    target_participant: u64, 
    reward_per_participant: u64, 
    question_list: [Vec<String>; 5]
) -> Result<()> {

    if title.is_empty() || 
    description.is_empty() || 
    question_list.len() == 0 || 
    reward_per_participant == 0 || 
    target_participant == 0 {
        return Err(SurvzError::InvalidSurveyInput.into());
    }
    
    if 
    open_timestamp == 0 || 
    close_timestamp == 0 || 
    open_timestamp > close_timestamp {
        return Err(SurvzError::InvalidTime.into());
    }

    let survey = &mut ctx.accounts.survey;

    survey.title = title;
    survey.description = description;
    survey.creator = *ctx.accounts.user.key;
    survey.open_timestamp = open_timestamp;
    survey.close_timestamp = close_timestamp;
    survey.target_participant = target_participant;
    survey.reward_per_participant = reward_per_participant;
    survey.balance_deposited = target_participant * reward_per_participant;
    survey.state = SurvzState::Closed;
    survey.question_list = question_list;

    Ok(())
}