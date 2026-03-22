pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::state::*;

declare_id!("2RrkEbo9XkFAgqtz5wL3xDgj3x9o6SiSw6meA9CjManb");

#[program]
pub mod dicepool {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreateDicePoolContext>,
        id: u64,
        start_time: i64,
        end_time: i64,
        capacity: u64,
        base_amount: u64,
    ) -> Result<()> {
        _create_pool(ctx, id, start_time, end_time, capacity, base_amount)
    }

    pub fn join_pool(ctx:Context<PlaceBetContext>,id:u64,amount:u64,target_value:u64) -> Result<()>{
        _place_bet(ctx, id, amount, target_value)
    }

    pub fn withdraw_all(ctx:Context<WithdrawContext>,id: u64) -> Result<()> {
        _withdraw_all(ctx,id)
    }

    pub fn set_result(ctx:Context<SetResultContext>,id: u64,result:u64,claimed_amount:u64) -> Result<()>{
        _set_result(ctx,id,result,claimed_amount)
    }

    pub fn claim_amount(ctx:Context<ClaimContext>,id:u64) -> Result<()>{
        _claim(ctx, id)
    }
}
