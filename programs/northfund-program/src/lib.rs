use anchor_lang::prelude::*;

declare_id!("G12WJ5jg5BUuQtf1H3L4d3WRqNDKHo4RG5qjFHC1NqK2");

#[program]
pub mod northfund_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
