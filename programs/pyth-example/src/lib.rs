use std::str::FromStr;
use anchor_lang::prelude::*;
use pyth_sdk_solana::{load_price_feed_from_account_info, Price, PriceFeed};

declare_id!("EX9d7TRhTi7zWnocwRfALSkjePjpqLaVFhqyUsC2biCG");

#[program]
pub mod pyth_example {
    use super::*;
    use anchor_lang::solana_program::{
        native_token::LAMPORTS_PER_SOL, program::invoke, system_instruction,
    };

    pub fn pay_usd(ctx: Context<PayUSD>, amount: u64) -> Result<()> {
        let current_price = get_sol_usd_price(&ctx.accounts.sol_usd_price_account)?;
        let amount_in_lamports =
            amount * LAMPORTS_PER_SOL * 10u64.pow(u32::try_from(-current_price.expo).unwrap())
                / (u64::try_from(current_price.price).unwrap());
        let transfer_instruction = system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount_in_lamports,
        );
        invoke(&transfer_instruction, &ctx.accounts.to_account_infos())?;
        return Err(error!(CustomError::PriceIsDown));
    }
}

pub fn get_sol_usd_price(account: &AccountInfo) -> Result<Price> {
    const SOL_USD_PRICEFEED_ID: &str = "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix";
    const STALENESS_THRESHOLD: u64 = 60; // 60 seconds

    if Pubkey::from_str(SOL_USD_PRICEFEED_ID) != Ok(account.key()) {
        return Err(error!(CustomError::AccountNotTheSolUsdPriceAccount));
    };

    let sol_usd_price_feed: PriceFeed = load_price_feed_from_account_info(account)
        .map_err(|_| error!(CustomError::DeserializationError))?;

    return sol_usd_price_feed
        .get_latest_available_price_within_duration(
            Clock::get().unwrap().unix_timestamp,
            STALENESS_THRESHOLD,
        )
        .ok_or(error!(CustomError::RecentPriceNotAvailable));
}
#[derive(Accounts)]
#[instruction(amount : u64)]
pub struct PayUSD<'info> {
    pub from: Signer<'info>,
    /// CHECK : This is an unchecked receiver account
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// CHECK : We will manually check this against the Pubkey of the price feed
    pub sol_usd_price_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum CustomError {
    PriceIsDown,
    WrongPriceFeedId,
    AccountNotTheSolUsdPriceAccount,
    DeserializationError,
    RecentPriceNotAvailable,
}
