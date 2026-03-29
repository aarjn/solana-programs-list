use anchor_lang::prelude::*;
use mpl_core::{
    ID as MPL_CORE_PROGRAM_ID,
    instructions::{
        CreateV2CpiBuilder, CreateCollectionV2CpiBuilder,
        TransferV1CpiBuilder, UpdateV2CpiBuilder, BurnV1CpiBuilder,
    }
};

declare_id!("4AgaC13ZKg8PhS2LU8C3EsxyrhHyvdcGJgvqiUTkdwwS");

#[program]
pub mod anchor_nft_metaplex {
    use super::*;

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        CreateCollectionV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .collection(&ctx.accounts.collection.to_account_info())
            .payer(&ctx.accounts.payer.to_account_info())
            .update_authority(Some(&ctx.accounts.update_authority.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .invoke()?;
        Ok(())
    }

    pub fn create_asset(
        ctx: Context<CreateAsset>,
        args: CreateAssetArgs,
    ) -> Result<()> {
        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection.to_account_info()))
            .authority(Some(&ctx.accounts.authority.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .owner(Some(&ctx.accounts.owner.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .invoke()?;
        Ok(())
    }

    pub fn transfer_asset(ctx: Context<TransferAsset>) -> Result<()> {
        TransferV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .authority(Some(&ctx.accounts.authority.to_account_info()))
            .new_owner(&ctx.accounts.new_owner.to_account_info())
            .system_program(Some(&ctx.accounts.system_program.to_account_info()))
            .invoke()?;
        Ok(())
    }

    pub fn update_asset(
        ctx: Context<UpdateAsset>,
        args: UpdateAssetArgs,
    ) -> Result<()> {
        UpdateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .authority(Some(&ctx.accounts.authority.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .new_name(args.new_name)
            .new_uri(args.new_uri)
            .invoke()?;
        Ok(())
    }

    pub fn burn_asset(ctx: Context<BurnAsset>) -> Result<()> {
        BurnV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .authority(Some(&ctx.accounts.authority.to_account_info()))
            .system_program(Some(&ctx.accounts.system_program.to_account_info()))
            .invoke()?;
        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateCollectionArgs {
    pub name: String,
    pub uri: String,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateAssetArgs {
    pub name: String,
    pub uri: String,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateAssetArgs {
    pub new_name: String,
    pub new_uri: String,
}

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub collection: Signer<'info>,
    pub update_authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub asset: Signer<'info>,
    /// CHECK: mpl_core validates
    #[account(mut)]
    pub collection: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: mpl_core validates
    pub owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct TransferAsset<'info> {
    /// CHECK: mpl_core validates
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,
    /// CHECK: mpl_core validates
    #[account(mut)]
    pub collection: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    /// CHECK: mpl_core validates
    pub new_owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct UpdateAsset<'info> {
    /// CHECK: mpl_core validates
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,
    /// CHECK: mpl_core validates
    #[account(mut)]
    pub collection: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct BurnAsset<'info> {
    /// CHECK: mpl_core validates
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,
    /// CHECK: mpl_core validates
    #[account(mut)]
    pub collection: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}