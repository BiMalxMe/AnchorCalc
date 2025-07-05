use anchor_lang::prelude::*;

declare_id!("9rQF7W8qTuQmzn6hzWgBwT7CbakXxG8Nfj78G7pHMWiK");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
