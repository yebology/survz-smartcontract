use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
pub struct FillSurvey<'info> {
    #[account(
        init,
        payer=user,
        space=Answer::MAXIMUM_SIZE,
        seeds=[b"fill_survey", user.key().as_ref()],
        bump
    )]
    pub answer: Account<'info, Answer>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub survey: Account<'info, Survey>,
    pub system_program: Program<'info, System>
}

pub fn handler(
    ctx: Context<FillSurvey>,
    answer_list: [Vec<String>; 5]
) -> Result<()> {
    
    let answer = &mut ctx.accounts.answer;
    let user = &mut ctx.accounts.user;
    let survey = &mut ctx.accounts.survey;

    let rent = Rent::get()?.minimum_balance(survey.to_account_info().data_len());
    let survey_balance = survey.to_account_info().lamports();
    let amount = survey.reward_per_participant;

    if survey.state == SurvzState::Closed {
        return Err(SurvzError::SurveyNotStarted.into());
    }

    if (survey_balance - rent) < amount {
        return Err(SurvzError::InsufficientFunds.into());
    }

    if answer_list.len() != 5 {
        return Err(SurvzError::AllFieldMustBeAnswered.into());
    }

    answer.user = user.key();
    answer.answer_list = answer_list;

    ctx.accounts.survey.sub_lamports(amount)?;
    ctx.accounts.user.add_lamports(amount)?;

    Ok(())
}