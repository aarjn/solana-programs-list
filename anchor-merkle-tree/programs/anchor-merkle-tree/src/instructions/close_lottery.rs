use anchor_lang::prelude::*;

use crate::state::Lottery;

#[derive(Accounts)]
pub struct CloseLottery<'info> {
    #[account(
        mut,
        seeds = [Lottery::SEED_PREFIX, authority.key().as_ref()],
        bump = lottery.bump,
        has_one = authority,
        close = authority
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

impl<'info> CloseLottery<'info> {
    pub fn close_lottery(&mut self) -> Result<()> {
        msg!("Lottery closed, rent returned to authority");
        Ok(())
    }
}
