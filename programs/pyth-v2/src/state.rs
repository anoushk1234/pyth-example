use anchor_lang::prelude::*;
use pyth_sdk_solana::*;

use crate::CustomError;

#[derive(AnchorSerialize,AnchorDeserialize)]
pub struct Feed{
    pub price: i64,
    pub qty: i64,
    pub decimals: i32
}

impl Feed{
    pub fn get_price(mut self,price_key:&AccountInfo) -> Result<(Feed)>{
        let feed1 = load_price_feed_from_account_info(price_key);
        let current_timestamp1 = Clock::get()?.unix_timestamp;
        let result1 = feed1.unwrap()
            .get_latest_available_price_within_duration(current_timestamp1, 60)
            .ok_or(ProgramError::Custom(3))?;
        let loan_max_price = result1
            .price
            .checked_add(result1.conf as i64)
            .ok_or(ProgramError::Custom(4))?;
        let loan_max_value = loan_max_price
            .checked_mul(self.qty)
            .ok_or(ProgramError::Custom(4))?;
        
        self.price = loan_max_value;
        self.decimals = result1.expo;
        Ok(self)
    }
}
