    use anchor_lang::prelude::*;

    pub mod errors;
    pub mod instructions;
    pub mod state;

    use errors::*;
    use state::*;
    use instructions::*;

    declare_id!("GVVMixtE5VECdKHJzaJSMgbFNjifwmJTLgb4skuwDeMK");

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
            reward_per_participant: u64, 
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
                reward_per_participant, 
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

        pub fn change_status(ctx: Context<ChangeStatus>) -> Result<()> {
            instructions::change_status::handler(ctx)?;
            Ok(())
        }
    }
