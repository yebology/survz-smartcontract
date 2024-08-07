use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
#[instruction(survey_id: u64)]
pub struct FillSurvey<'info> {
    #[account(
        init_if_needed,
        payer=user,
        space=Answer::MAXIMUM_SIZE,
        seeds=[
            b"answer".as_ref(), 
            user.key().as_ref(),
            &survey_id.to_le_bytes()
        ],
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
    survey_id: u64,
    answer_list: Vec<String>
) -> Result<()> {
    
    let answer = &mut ctx.accounts.answer;
    let user = &mut ctx.accounts.user;
    let survey = &mut ctx.accounts.survey;
    let amount = survey.reward_per_participant;

    require!(survey.state == SurvzState::Open, SurvzError::SurveyIsClosed);

    require!(answer_list.len() == 5, SurvzError::AllFieldMustBeAnswered);

    for answer in answer_list.iter() {
        require!(!answer.trim().is_empty(), SurvzError::InvalidSurveyInput);
    }

    answer.user = user.key();
    answer.survey_id = survey_id;
    answer.answer_list = answer_list;

    survey.sub_lamports(amount)?;
    user.add_lamports(amount)?;

    survey.current_participant += 1;
    survey.total_reward -= amount;

    if survey.current_participant == survey.target_participant {
        survey.state = SurvzState::Closed;
    }

    Ok(())
}