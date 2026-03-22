use crate::{error::PoolError, DicePool};
use anchor_lang::{prelude::*};

pub fn _set_result(ctx: Context<SetResultContext>, id: u64, result: u64,claimed_amount:u64) -> Result<()> {
    let dice_pool_account = &mut ctx.accounts.dice_pool;
    // Only allow after pool end
    require!(
        dice_pool_account.end_time < Clock::get()?.unix_timestamp,
        PoolError::PoolNotOver
    );
    require!(dice_pool_account.creator == ctx.accounts.creator.key(), PoolError::UnAuthorised);
    dice_pool_account.result = result;
    dice_pool_account.ended = true;
    dice_pool_account.clamied_amount = claimed_amount;
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct SetResultContext<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"dice_pool", creator.key().as_ref(), &id.to_le_bytes()],
        bump,
        has_one = creator
    )]
    pub dice_pool: Account<'info, DicePool>,

}
