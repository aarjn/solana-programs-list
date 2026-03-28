use anchor_lang::{prelude::*, system_program};

use crate::{error::PoolError, DicePlayer, DicePool};
pub fn _place_bet(ctx:Context<PlaceBetContext>,id:u64,amount:u64,target_value:u64) -> Result<()>{
    let dice_pool_account = &mut ctx.accounts.dice_pool;
    let dice_player = &mut ctx.accounts.dice_player;
    let user = &mut ctx.accounts.payer;
    let system_program = &mut ctx.accounts.system_program;
    require!(dice_pool_account.betters.len()<=20 , PoolError::PoolOverflow);
    require!(!dice_pool_account.ended,PoolError::PoolOver);
    require!(amount>=dice_pool_account.base_amount,PoolError::BaseAmountError);
    require!(
        dice_pool_account.start_time < Clock::get()?.unix_timestamp,
        PoolError::StartDateInThePast
    );
    require!(
        dice_pool_account.end_time > Clock::get()?.unix_timestamp,
        PoolError::EndDateShouldInFuture
    );
    let cpi_context = CpiContext::new(
        system_program.to_account_info(),
        system_program::Transfer {
            from: user.to_account_info(),
            to: dice_pool_account.to_account_info(),
        },
    );
    system_program::transfer(cpi_context, amount)?;
    dice_player.amount = amount;
    dice_player.claimed = false;
    dice_player.target = target_value;
    dice_player.user = *ctx.accounts.payer.key;

    dice_pool_account.total_amount+=amount;
    dice_pool_account.remaining_seats -=1;
    dice_pool_account.betters.push(dice_player.key());
    Ok(())
}

#[derive(Accounts)]
#[instruction(id:u64)]
pub struct PlaceBetContext<'info>{
    #[account(mut)]
    pub payer : Signer<'info>,
    #[account(
        mut
    )]
    pub dice_pool: Account<'info, DicePool>,
    #[account(
        init,
        payer = payer,
        space = 8 + DicePlayer::INIT_SPACE,
        seeds = [b"dice_player",payer.key().as_ref(),&id.to_le_bytes()], 
        bump
    )]
    pub dice_player: Account<'info, DicePlayer>,
    pub system_program: Program<'info, System>,
}