use anchor_lang::prelude::*;

use crate::{error::PoolError, DicePool};

pub fn _create_pool(
    ctx: Context<CreateDicePoolContext>,
    id: u64,
    start_time: i64,
    end_time: i64,
    capacity: u64,
    base_amount: u64,
) -> Result<()> {
    let dice_pool_account = &mut ctx.accounts.dice_pool;
    dice_pool_account.id = id;
    dice_pool_account.start_time = start_time;
    dice_pool_account.end_time = end_time;
    require!(capacity <= 20, PoolError::MaxCapacityYouCanDo);
    dice_pool_account.capacity = capacity;
    dice_pool_account.base_amount = base_amount;
    dice_pool_account.betters = Vec::new();
    dice_pool_account.creator = *ctx.accounts.payer.key;
    dice_pool_account.ended = false;
    dice_pool_account.result = 0;
    dice_pool_account.remaining_seats = capacity;
    dice_pool_account.total_amount = 0;
    dice_pool_account.clamied_amount = 0;
    Ok(())
}
#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateDicePoolContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + DicePool::INIT_SPACE,
        seeds = [b"dice_pool", payer.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub dice_pool: Account<'info, DicePool>,

    pub system_program: Program<'info, System>,
}
