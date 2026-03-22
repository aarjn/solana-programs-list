use {
    crate::state::Escrow,
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token, TokenCpi},
};

#[derive(Accounts)]
pub struct Make<'info> {
    pub maker: &'info mut Signer,
    #[account(init, payer = maker, seeds = [b"escrow", maker], bump)]
    pub escrow: &'info mut Account<Escrow>,
    pub mint_a: &'info Account<Mint>,
    pub mint_b: &'info Account<Mint>,
    pub maker_ta_a: &'info mut Account<Token>,
    #[account(init_if_needed, payer = maker, token::mint = mint_b, token::authority = maker)]
    pub maker_ta_b: &'info mut Account<Token>,
    #[account(init_if_needed, payer = maker, token::mint = mint_a, token::authority = escrow)]
    pub vault_ta_a: &'info mut Account<Token>,
    pub rent: &'info Sysvar<Rent>,
    pub token_program: &'info Program<Token>,
    pub system_program: &'info Program<System>,
}

pub fn make_escrow(&mut self, receive: u64, bumps: &MakeBumps) -> Result<(), ProgramError> {
    self.escrow.set_inner(
        *self.maker.address(),
        *self.mint_a.address(),
        *self.mint_b.address(),
        *self.maker_ta_b.address(),
        receive,
        bumps.escrow,
    );
    Ok(())
}
