use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
#[instruction(survey_id: u64)]
pub struct FillSurvey<'info> {
    #[account(
        init,
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

pub fn fill_handler(
    ctx: Context<FillSurvey>,
    survey_id: u64,
    answer_list: Vec<String>
) -> Result<()> {
    
    let answer = &mut ctx.accounts.answer;
    let user = &mut ctx.accounts.user;
    let survey = &mut ctx.accounts.survey;
    let amount = survey.reward_per_participant;
    let current_timestamp = Clock::get().unwrap().unix_timestamp as u64;

    survey.state = match 
    current_timestamp >= survey.open_timestamp && 
    current_timestamp < survey.close_timestamp && 
    survey.current_participant != survey.target_participant {
        true => SurvzState::Open,
        false => SurvzState::Closed
    };

    require!(
        (
            survey.state == SurvzState::Open
        ),
        SurvzError::SurveyIsClosed
    );

    require!(
        (
            answer_list.len() == 5 &&
            answer_list.iter().all(|answer| !answer.trim().is_empty())
        ),
        SurvzError::AllFieldMustBeAnswered
    );

    answer.user = user.key();
    answer.survey_id = survey_id;
    answer.answer_list = answer_list;

    survey.sub_lamports(amount)?;
    user.add_lamports(amount)?;

    survey.current_participant += 1;
    survey.total_reward -= amount;

    emit!(SurveyFilled {
        user: user.key(),
        survey_account: survey.key()
    });

    Ok(())
}