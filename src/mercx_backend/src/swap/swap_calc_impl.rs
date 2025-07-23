use candid::Nat;
use num::rational::{BigRational, Ratio};
use num::BigInt;

use super::swap_calc::SwapCalc;

use crate::helpers::math_helpers::{nat_add, nat_is_zero, nat_subtract, nat_to_bigint, nat_to_decimal_precision, nat_zero};
use crate::pool::handlers;
// use crate::stable_token::token::Token;
use crate::token::handlers as token_handlers;

impl SwapCalc {
    /// this is the net amount the user will receive after the fees and gas are taken off
    /// this is used for price calculations
    pub fn receive_amount_with_fees_and_gas(&self) -> Nat {
        let total_fees = nat_add(&self.lp_fee, &self.gas_fee);
        nat_subtract(&self.receive_amount, &total_fees).unwrap_or(nat_zero())
    }

    // if the swap is zero-amounts, then it will query the pool and return the mid price
    // this is for swap_price where no amount is specified
    pub fn get_price(&self) -> Option<BigRational> {
        //This happens when the user hasn't specified a swap amount (e.g., when just checking current pool price).
        //User just wants to check the current price in the pool (no swap).
        if nat_is_zero(&self.pay_amount) {
            return self.get_mid_price();
        }

        //The user is actually swapping tokens.
        let pay_token = token_handlers::get_by_token_id(self.pay_token_id)?;
        let receive_token = token_handlers::get_by_token_id(self.receive_token_id)?;
        let max_decimals = std::cmp::max(pay_token.decimals(), receive_token.decimals());
        //Converts the pay amount into BigInt, scaled to the shared max_decimals.
        let pay_amount_in_max_decimals = nat_to_bigint(&nat_to_decimal_precision(&self.pay_amount, pay_token.decimals(), max_decimals));
        //Gets the actual amount the user will receive after fees (LP + gas).
        let receive_amount_in_max_decimals = nat_to_bigint(&nat_to_decimal_precision(
            &self.receive_amount_with_fees_and_gas(),
            receive_token.decimals(),
            max_decimals,
        ));

        Some(BigRational::new(receive_amount_in_max_decimals, pay_amount_in_max_decimals)) 
    }


  //This get_mid_price function returns the current price of a token pair in a liquidity pool, 
    pub fn get_mid_price(&self) -> Option<BigRational> {
        let pool = handlers::get_by_pool_id(self.pool_id)?;
        // check if swap is inverted to the pool and if so return the reciprocal price
        // receive_token != pool.token_1 (ckUSDT) means the swap is inverted to the pool
        let price = pool.get_price()?;
        //This means you're receiving token_1, which is the default direction of the pool.So, price is as-is: how many token_1 units per token_0.
        if self.receive_token_id == pool.token_id_1 { 
            Some(price)
        } else if price == Ratio::from_integer(BigInt::from(0)) { //If price is zero, return None.
            // prevent reciprocal of 0
            None
        } 
        //If you're receiving token_0, that means the swap is in the reverse direction.
        else {
            Some(price.recip()) //1 / (token_1 per token_0)
        }
    }
}