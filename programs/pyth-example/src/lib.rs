use anchor_lang::prelude::*;
use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};

declare_id!("6mg6PeANjaa931s5zieAmb7B3NAyU9RXin2id2VVc48D");

#[program]
pub mod pyth_example {
    use super::*;

    pub fn get_price(ctx: Context<GetPrice>) -> Result<()> {
        let price_feed : PriceFeed = load_price_feed_from_account_info(&ctx.accounts.price).unwrap();
        msg!("{:}", price_feed.get_current_price().unwrap().price);
        msg!("{:}", price_feed.get_current_price().unwrap().expo);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GetPrice<'info> {
    #[account(mut)]
    pub funding: Signer<'info>,
    /// CHECK: Pyth does not use Anchor
    pub price: UncheckedAccount<'info>,
    
}
