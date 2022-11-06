use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_portal {
    use super::*;
    pub fn create_counter(ctx: Context<Counter>) -> Result<()> {
        let base_counter = &mut ctx.accounts.base_counter;
        base_counter.total = 0;
        Ok(())
    }

    pub fn counter_add(ctx: Context<CounterAdd>) -> Result<()> {
        let base_counter = &mut ctx.accounts.base_counter;
        base_counter.total += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Counter<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_counter: Account<'info, BaseCounter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CounterAdd<'info> {
    #[account(mut)]
    pub base_counter: Account<'info, BaseCounter>,
}

#[account]
pub struct BaseCounter {
    pub total: u64,
}
