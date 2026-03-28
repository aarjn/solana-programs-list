use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;

declare_id!("3XXtmiUzi16kNFNgg2AErEx55ZUn3SB5DsmzVBiNQttE");
pub fn transfer<'a>(
    system_program: AccountInfo<'a>,
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    amount: u64,
    seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    let amount_needed = amount;
    if amount_needed > from.lamports() {
        msg!(
            "Need {} lamports, but only have {}",
            amount_needed,
            from.lamports()
        );

        return Err(ErrorCode::NotEnoughFundsToPlay.into());
    }

    let transfer_accounts = anchor_lang::system_program::Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
    };

    let transfer_ctx = match seeds {
        Some(seeds) => CpiContext::new_with_signer(system_program, transfer_accounts, seeds),
        None => CpiContext::new(system_program, transfer_accounts),
    };

    anchor_lang::system_program::transfer(transfer_ctx, amount)
}
#[program]
pub mod coinflip {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let player_account = &mut ctx.accounts.player;
        player_account.latest_flip_result = false;
        player_account.randomness_account = Pubkey::default();
        player_account.wager = 100;
        player_account.bump = ctx.bumps.player;
        player_account.allowed_user = ctx.accounts.user.key();
        Ok(())
    }

    pub fn coin_flip(ctx: Context<CoinFlipContext>, sb_account: Pubkey, guess: bool) -> Result<()> {
        let clock = Clock::get()?;
        let player = &mut ctx.accounts.player;

        player.current_guess = guess;
        let randomness_data =
            RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow())
                .unwrap();

        if randomness_data.seed_slot != clock.slot - 1 {
            msg!("seed_slot: {}", randomness_data.seed_slot);
            msg!("slot: {}", clock.slot);
            return Err(ErrorCode::RandomnessAlreadyRevealed.into());
        }

        transfer(
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.escrow_account.to_account_info(),
            player.wager,
            None,
        )?;

        player.commit_slot = randomness_data.seed_slot;
        player.randomness_account = sb_account;

        Ok(())
    }

    pub fn settle_flip(ctx: Context<SettleFlip>, escrow_bump: u8) -> Result<()> {
        let player = &mut ctx.accounts.player_state;
        let clock: Clock = Clock::get()?;

        if ctx.accounts.randomness_account_data.key() != player.randomness_account {
            return Err(ErrorCode::InvalidRandomnessAccount.into());
        }

        let random_data =
            RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow())
                .unwrap();

        if random_data.seed_slot != player.commit_slot {
            return Err(ErrorCode::RandomnessExpired.into());
        }

        let revealed_random_value = random_data
            .get_value(&clock)
            .map_err(|_| ErrorCode::RandomnessNotResolved)?;

        let randomness_result = revealed_random_value[0] % 2 == 0;

        player.latest_flip_result = randomness_result;

        let seed_prefix = b"stateEscrow".as_ref();
        let escrow_seed = &[&seed_prefix[..], &[escrow_bump]];
        let seeds_slice: &[&[u8]] = escrow_seed;
        let binding = [seeds_slice];
        let seeds: Option<&[&[&[u8]]]> = Some(&binding);

        if randomness_result {
            msg!("FLIP_RESULT: Heads");
        } else {
            msg!("FLIP_RESULT: Tails");
        }

        if randomness_result == player.current_guess {
            msg!("You win!");
            let rent = Rent::get()?;
            let needed_lamports =
                player.wager * 2 + rent.minimum_balance(ctx.accounts.escrow_account.data_len());
            if needed_lamports > ctx.accounts.escrow_account.lamports() {
                msg!("Not enough funds in treasury to pay out the user. Please try again later");
            } else {
                transfer(
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.escrow_account.to_account_info(), // Transfer from the escrow
                    ctx.accounts.user.to_account_info(),           // Payout to the user's wallet
                    player.wager * 2, // If the player wins, they get double their wager if the escrow account has enough funds
                    seeds,            // Include seeds
                )?;
            }
        } else {
            // On lose, we keep the user's initial colletaral and they are
            // allowed to play again.
            msg!("You lose!");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SettleFlip<'info> {
    #[account(mut,
        seeds = [b"playerState".as_ref(), user.key().as_ref()],
        bump = player_state.bump)]
    pub player_state: Account<'info, PlayerAccount>,
    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
    /// CHECK: This is a simple Solana account holding SOL.
    #[account(mut, seeds = [b"stateEscrow".as_ref()], bump)]
    pub escrow_account: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CoinFlipContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds=[b"playerState".as_ref(), user.key().as_ref()],
        bump = player.bump
    )]
    pub player: Account<'info, PlayerAccount>,
    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
    /// CHECK: This is a simple Solana account holding SOL.
    #[account(mut, seeds = [b"stateEscrow".as_ref()], bump)]
    pub escrow_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds=[b"playerState".as_ref(), user.key().as_ref()],
        bump,
        space= 8 + PlayerAccount::INIT_SPACE

    )]
    pub player: Account<'info, PlayerAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct PlayerAccount {
    allowed_user: Pubkey,
    latest_flip_result: bool,
    randomness_account: Pubkey,
    current_guess: bool,
    wager: u64, // The wager amount
    bump: u8,
    commit_slot: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access attempt.")]
    Unauthorized,
    GameStillActive,
    NotEnoughFundsToPlay,
    RandomnessAlreadyRevealed,
    RandomnessNotResolved,
    RandomnessExpired,
    InvalidRandomnessAccount,
}
