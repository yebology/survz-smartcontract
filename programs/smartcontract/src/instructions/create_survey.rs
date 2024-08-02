use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
pub struct CreateSurvey<'info> {
    #[account(
        init,
        payer=user,
        space=50000,
        seeds=[b"create_survey", user.key().as_ref(), &survey_amount.amount.to_le_bytes()],
        bump
    )]
    pub survey: Account<'info, Survey>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds=[b"survey_count"],
        bump
    )]
    pub survey_amount: Account<'info, SurveyAmount>,
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
    let survey = &mut ctx.accounts.survey;
    let survey_amount = &mut ctx.accounts.survey_amount;

    survey.id = survey_amount.amount;
    survey.title = title;
    survey.description = description;
    survey.creator = *ctx.accounts.user.key;
    survey.open_timestamp = open_timestamp;
    survey.close_timestamp = close_timestamp;
    survey.target_participant = target_participant;
    survey.reward_per_participant = reward_per_participant;
    survey.question_list = question_list;

    survey_amount.amount += 1;
    Ok(())
}