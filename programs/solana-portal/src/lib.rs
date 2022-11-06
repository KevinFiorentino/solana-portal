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

    pub fn counter_add(ctx: Context<CounterAdd>, url_image: String) -> Result<()> {
        let base_counter = &mut ctx.accounts.base_counter;
        let user = &mut ctx.accounts.user;

        let item = ImageStruct {
            url_image: url_image.to_string(),
            user_address: *user.to_account_info().key,
        };

        base_counter.image_list.push(item);
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
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ImageStruct {
    pub url_image: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseCounter {
    pub total: u64,
    pub image_list: Vec<ImageStruct>,
}
