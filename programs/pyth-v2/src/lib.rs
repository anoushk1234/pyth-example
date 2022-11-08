use anchor_lang::prelude::*;
pub mod state;
use state::Feed;

declare_id!("8ZKHmrEvmrC6DskTRyk2sD8FkWrQs8Uv6NgiiJv8WosL");

#[program]
pub mod pyth_v2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let sol_feed = Feed::get_price( Feed{
            decimals:0,
            price:0,
            qty:1
        }, &ctx.accounts.sol_price_key).unwrap();
        msg!(
            "The maximum value is {} * 10^({}).",
            sol_feed.price,
            sol_feed.decimals
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    /// CHECK: safe
    pub sol_price_key: AccountInfo<'info>,
}

#[error_code]
pub enum CustomError {
    #[msg("Invalid Price product account")]
    InvalidPriceAccount,
}