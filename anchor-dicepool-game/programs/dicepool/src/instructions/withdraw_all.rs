use anchor_lang::{prelude::*, system_program};
use crate::{error::PoolError, DicePool};
pub fn _withdraw_all(ctx: Context<WithdrawContext>, id: u64) -> Result<()> {
    let dice_pool_account = &mut ctx.accounts.dice_pool;
    let creator = &mut ctx.accounts.creator;

    require!(
        creator.key() == dice_pool_account.creator,
        PoolError::YouAreNotOwner
    );

    // Amount to transfer = everything except rent-exempt minimum
    let rent_exempt = Rent::get()?.minimum_balance(dice_pool_account.to_account_info().data_len());
    let amount = **dice_pool_account.to_account_info().lamports.borrow() - rent_exempt;

    require!(amount > 0, PoolError::NoFundsAvailable);

    // Transfer lamports safely
    **dice_pool_account.to_account_info().try_borrow_mut_lamports()? -= amount;
    **creator.to_account_info().try_borrow_mut_lamports()? += amount;

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct WithdrawContext<'info> {
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
