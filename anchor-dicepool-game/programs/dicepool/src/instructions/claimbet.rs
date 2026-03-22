use anchor_lang::{prelude::*, system_program};

use crate::{error::PoolError, DicePlayer, DicePool};

pub fn _claim(ctx: Context<ClaimContext>, id: u64) -> Result<()> {
    let dice_pool = &ctx.accounts.dice_pool;
    let dice_player = &mut ctx.accounts.dice_player;
    let user = &ctx.accounts.user;
    require!(dice_pool.ended, PoolError::PoolNotOver);
    require!(dice_player.user == user.key(), PoolError::NotYourAccount);

    if dice_player.target == dice_pool.result {
        dice_player.claimed_amount = dice_pool.clamied_amount;
        **ctx.accounts
            .dice_pool
            .to_account_info()
            .try_borrow_mut_lamports()? -= dice_player.claimed_amount;
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += dice_player.claimed_amount;
    } else {
        dice_player.claimed_amount = 0;
    }
    dice_player.claimed = true;
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct ClaimContext<'info> {
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"dice_player", &id.to_le_bytes(), user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub dice_player: Account<'info, DicePlayer>,

    #[account(
        seeds = [b"dice_pool", dice_pool.creator.as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub dice_pool: Account<'info, DicePool>,
}
