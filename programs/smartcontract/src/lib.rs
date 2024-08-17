    use anchor_lang::prelude::*;

    pub mod errors;
    pub mod events;
    pub mod instructions;
    pub mod state;

    use errors::*;
    use events::*;
    use state::*;
    use instructions::*;

    declare_id!("7bfa2DiKuMvy6vDxyTjfnvPt62JuQG39XWbPtgJyHwYd");

    #[program]
    pub mod smartcontract {
        use super::*;

        pub fn create_survey(
            ctx: Context<CreateSurvey>, 
            id: u64,
            title: String, 
            description: String, 
            open_timestamp: u64, 
            close_timestamp: u64, 
            target_participant: u64, 
            total_reward: u64, 
            question_list: [String; 5]
        ) -> Result<()> {
            instructions::create_survey::create_handler(
                ctx, 
                id,
                title, 
                description, 
                open_timestamp, 
                close_timestamp, 
                target_participant, 
                total_reward, 
                question_list
            )?;
            Ok(())
        }

        pub fn fill_survey(
            ctx: Context<FillSurvey>,
            survey_id: u64,
            answer_list: [String; 5]
        ) -> Result<()> {
            instructions::fill_survey::fill_handler(
                ctx,
                survey_id,
                answer_list
            )?;
            Ok(())
        }

    }
