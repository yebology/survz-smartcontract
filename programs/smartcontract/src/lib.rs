    use anchor_lang::prelude::*;

    pub mod errors;
    pub mod events;
    pub mod instructions;
    pub mod state;

    use errors::*;
    use events::*;
    use state::*;
    use instructions::*;

    declare_id!("E4ToFZT9sWoMewsLiVKweYt2QRHwUFBZ2dJcw92br2Dx");

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
            question_list: Vec<String>
        ) -> Result<()> {
            instructions::create_survey::handler(
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
            answer_list: Vec<String>
        ) -> Result<()> {
            instructions::fill_survey::handler(
                ctx,
                survey_id,
                answer_list
            )?;
            Ok(())
        }

    }
