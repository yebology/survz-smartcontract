use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::*;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateSurvey<'info> {
    #[account(
        init,
        payer=user,
        space=Survey::MAXIMUM_SIZE,
        seeds=[
            b"survey".as_ref(), 
            user.key().as_ref(),
            &id.to_le_bytes()
        ],
        bump
    )]
    pub survey: Account<'info, Survey>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn handler(
    ctx: Context<CreateSurvey>, 
    id: u64,
    title: String, 
    description: String, 
    open_timestamp: u64, 
    close_timestamp: u64, 
    target_participant: u64, 
    total_reward: u64, 
    question_list: Vec<String>
) -> Result<()> {

    if id == 0 || 
    title.is_empty() || 
    description.is_empty() || 
    question_list.len() != 5 || 
    total_reward == 0 || 
    target_participant == 0 {
        return Err(SurvzError::InvalidSurveyInput.into());
    }
    
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp as u64;

    if 
    open_timestamp == 0 || 
    close_timestamp == 0 || 
    open_timestamp > close_timestamp || current_timestamp > close_timestamp {
        return Err(SurvzError::InvalidTime.into());
    }

    let cpi_account = system_program::Transfer {
        from: ctx.accounts.user.to_account_info().clone(),
        to: ctx.accounts.survey.to_account_info().clone()
    };
    let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_account);
    system_program::transfer(cpi_context, total_reward)?;
    
    let rent = Rent::get()?.minimum_balance(ctx.accounts.survey.to_account_info().data_len());
    let survey = &mut ctx.accounts.survey;

    survey.id = id;
    survey.title = title;
    survey.description = description;
    survey.creator = *ctx.accounts.user.key;
    survey.open_timestamp = open_timestamp;
    survey.close_timestamp = close_timestamp;
    survey.current_participant = 0;
    survey.target_participant = target_participant;
    survey.reward_per_participant = (total_reward - rent) / target_participant;
    survey.total_reward = total_reward;
    survey.state = match current_timestamp >= open_timestamp {
        true => SurvzState::Open,
        false => SurvzState::Closed
    };
    survey.question_list = question_list;

    Ok(())
}