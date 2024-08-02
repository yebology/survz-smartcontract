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
    
    if answer_list.len() != 5 {
        return Err(SurvzError::AllFieldMustBeAnswered.into())
    }
    
    let answer = &mut ctx.accounts.answer;
    let user = &mut ctx.accounts.user;
    let survey = &mut ctx.accounts.survey;
    
    answer.user = user.key();
    answer.answer_list = answer_list;

    **survey.to_account_info().try_borrow_lamports()? -= survey.reward_per_participant;
    **user.to_account_info().try_borrow_lamports()? += survey.reward_per_participant;

    Ok(())
}