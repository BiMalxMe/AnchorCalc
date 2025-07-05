use anchor_lang::prelude::*;

declare_id!("9rQF7W8qTuQmzn6hzWgBwT7CbakXxG8Nfj78G7pHMWiK");

#[program]
pub mod anchor {

    use super::*;

    pub fn init(ctx :Context<Initaialize>,numto_init : u32) -> Result<()>{
        // initalizing a numto_add
        ctx.accounts.account.num = numto_init;
        Ok(())
    }
    pub fn Double(ctx : Context<Double>) -> Result<()>{
        ctx.accounts.account.num  = ctx.accounts.account.num * 2;
        Ok(())
    }
    pub fn add(ctx : Context<Add>,numto_add : u32) -> Result<()>{
        ctx.accounts.account.num += numto_add;
        Ok(())
    }
    }
 
    
#[account]
pub struct AccountDataShape{
    pub num : u32
}
#[derive(Accounts)]
pub struct Initaialize<'info>{
    #[account(init,payer = signer ,space = 8 + 8)]
    pub account : Account<'info,AccountDataShape>,
    pub system_program : Program<'info,System>,
    #[account(mut)]
    signer : Signer<'info>
}

#[derive(Accounts)]
pub struct Double<'info>{
    #[account(mut)]
    pub account : Account<'info,AccountDataShape>,
    #[account(mut)]
    signer : Signer<'info>
}
#[derive(Accounts)]
pub struct Add<'info>{
    #[account(mut)]
    pub account : Account<'info,AccountDataShape>,
    #[account(mut)]
    signer : Signer<'info>
}