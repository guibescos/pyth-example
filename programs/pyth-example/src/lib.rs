use anchor_lang::prelude::*;
use pyth_sdk_solana::load_price_feed_from_account_info;

declare_id!("CE8dkMzNfG9Tfhehw6wdLM7qoYeDZTWDs19FVX7VXvLZ");
const SOL_USD_PRICEFEED_ID : &str = "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix";

#[program]
pub mod pyth_example {
    use std::str::FromStr;
    use anchor_lang::solana_program::{system_instruction, native_token::LAMPORTS_PER_SOL, program::invoke};
    use super::*;

    pub fn pay_usd(ctx: Context<PayUSD>, amount : u64) -> Result<()> {
        
        if Pubkey::from_str(SOL_USD_PRICEFEED_ID) != Ok(ctx.accounts.sol_usd_price_account.key()){
            return Err(error!(CustomError::WrongPriceFeedId))
        };
        let sol_usd_price_feed = load_price_feed_from_account_info(&ctx.accounts.sol_usd_price_account).unwrap();
        if let Some(current_price) =  sol_usd_price_feed.get_current_price(){
            let amount_in_lamports = amount *  LAMPORTS_PER_SOL * 10u64.pow(u32::try_from(-current_price.expo).unwrap()) / (u64::try_from(current_price.price).unwrap());
            let transfer_instruction = system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.to.key(), amount_in_lamports);
            invoke(&transfer_instruction, &ctx.accounts.to_account_infos())?;
        } else {
            return Err(error!(CustomError::PriceIsDown));
        }
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(amount : u64)]
pub struct PayUSD<'info> {
    pub from : Signer<'info>,
    /// CHECK : This is an unchecked receiver account
    #[account(mut)]
    pub to : AccountInfo<'info>,
    /// CHECK : We will manually check this against the Pubkey of the price feed
    pub sol_usd_price_account : AccountInfo<'info>,
    pub system_program : Program<'info, System>
}

#[error_code]
pub enum CustomError {
    PriceIsDown,
    WrongPriceFeedId
}