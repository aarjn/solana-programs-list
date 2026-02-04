use anchor_lang::prelude::*;

use crate::state::Lottery;

#[derive(Accounts)]
pub struct InitializeLottery<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Lottery::INIT_SPACE,
        seeds = [Lottery::SEED_PREFIX, authority.key().as_ref()],
        bump
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeLottery<'info> {
    pub fn initialize_lottery(
        &mut self,
        merkle_root: [u8; 32],
        total_weight: u128,
        bumps: &InitializeLotteryBumps,
    ) -> Result<()> {
        self.lottery.set_inner(Lottery {
            authority: self.authority.key(),
            merkle_root,
            vrf_result: [0u8; 32],
            total_weight,
            winner: None,
            winner_weight: 0,
            finalized: false,
            bump: bumps.lottery,
        });

        msg!("Lottery initialized with merkle root");
        msg!("Merkle Root: {}", total_weight);

        Ok(())
    }
}
