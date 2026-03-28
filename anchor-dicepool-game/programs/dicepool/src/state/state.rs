use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct DicePool {
    pub id : u64,
    pub end_time:i64,
    pub start_time:i64,
    pub total_amount:u64,
    pub capacity:u64,
    pub remaining_seats:u64,
    pub result:u64,
    pub ended:bool,
    pub base_amount:u64,
    pub creator:Pubkey,
    pub clamied_amount:u64,
    #[max_len(20)] 
    pub betters : Vec<Pubkey>
}

#[account]
#[derive(InitSpace)]
pub struct DicePlayer {
    pub user : Pubkey,
    pub amount : u64,
    pub target: u64,
    pub claimed_amount : u64,
    pub claimed: bool
}


