use anchor_lang::prelude::*;

#[error_code]
pub enum PoolError {
    #[msg("20 betters can be in one pool")]
    MaxCapacityYouCanDo,
    #[msg("Pool is overflow")]
    PoolOverflow,
    #[msg("Please Deposit Base Amount")]
    BaseAmountError,
    #[msg("Start Date should in past")]
    StartDateInThePast,
    #[msg("End Date should in future")]
    EndDateShouldInFuture,
    #[msg("End Date should in future")]
    YouAreNotOwner,
    #[msg("No funds Available")]
    NoFundsAvailable,
    #[msg("Pool not over yet")]
    PoolNotOver,
    #[msg("Pool over")]
    PoolOver,
    #[msg("You are not in pool")]
    NotYourAccount,
    #[msg("You are not Authorised")]
    UnAuthorised,
}
