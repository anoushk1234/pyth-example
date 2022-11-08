use anchor_lang::prelude::*;
use pyth_client::*;

use crate::{ CustomError};

#[derive(AnchorSerialize,AnchorDeserialize)]
pub struct Feed{
    pub price: i64
}

impl Feed{
    pub fn get_price(mut self,product:&AccountInfo,price_key:&AccountInfo) -> Result<(Feed)>{
        let pyth_product_account = product.try_borrow_data()?;
    
        let product_account = *load_product(&pyth_product_account).unwrap();
      
        let pyth_price_pubkey = Pubkey::new(&product_account.px_acc.val);
     
        
        require!(product_account.px_acc.is_valid(), CustomError::InvalidPriceAccount);
       
        require_keys_eq!(pyth_price_pubkey,price_key.key(), CustomError::InvalidPriceAccount);
  

        let pyth_price_data =price_key.try_borrow_data()?;
        
        let price_account = *load_price(&pyth_price_data).unwrap();
        self.price = price_account.agg.price;
        Ok(self)
    }
}
