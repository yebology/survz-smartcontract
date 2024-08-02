use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
pub struct FillSurvey<'info> {
    #[account(
        init,
        payer=user,
        space=5000,
        seeds=[b"fill_survey"],
        bump
    )]
    pub answer: Account<'info, Answer>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}