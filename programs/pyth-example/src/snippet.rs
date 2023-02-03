use std::str::FromStr;
use anchor_lang::prelude::*;
use pyth_sdk_solana::{load_price_feed_from_account_info, Price, PriceFeed};

pub fn get_sol_usd_price(account: &AccountInfo) -> Result<Price> {
    const SOL_USD_PRICEFEED_ID: &str = "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG";
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