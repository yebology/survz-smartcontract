use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
pub struct ChangeStatus<'info> {
    #[account(mut)]
    pub survey: Account<'info, Survey>,
    #[account(mut)]
    pub user: Signer<'info>
}

pub fn handler(ctx: Context<ChangeStatus>) -> Result<()> {
    let survey = &mut ctx.accounts.survey;
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp as u64;

    if current_timestamp >= survey.open_timestamp && 
    current_timestamp < survey.close_timestamp && 
    survey.state == SurvzState::Closed {
        survey.state = SurvzState::Open;
    }
    else if current_timestamp >= survey.close_timestamp && 
    survey.state == SurvzState::Open {
        survey.state = SurvzState::Closed;
    }
    Ok(())
}