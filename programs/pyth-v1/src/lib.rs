use anchor_lang::prelude::*;
use pyth_client::*;

pub mod state;
use state::Feed;
declare_id!("37ETWfZQrpGUWrKrsf28qPvPdqqwr3iqDqT12fZw1Qk3");

#[program]
pub mod pyth_v1 {
    

    use super::*;
    #[inline(never)]
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
   
        let sol_price = Feed::get_price(Feed{price: 0}, &ctx.accounts.sol_product,&ctx.accounts.sol_price_key);
        msg!("sol_price {:?}",sol_price.unwrap().price);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    /// CHECK: this is safe cos it will be checked by pyth program
    pub sol_price_key: AccountInfo<'info>,
    /// CHECK: this is safe
    pub sol_product: AccountInfo<'info>,

}

#[error_code]
pub enum CustomError {
    #[msg("Invalid Price product account")]
    InvalidPriceAccount,
}