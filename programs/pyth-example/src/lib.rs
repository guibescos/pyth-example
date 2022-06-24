use anchor_lang::prelude::*;

declare_id!("6mg6PeANjaa931s5zieAmb7B3NAyU9RXin2id2VVc48D");

#[program]
pub mod pyth_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
