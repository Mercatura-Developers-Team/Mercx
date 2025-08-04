use num::{BigRational, Zero};

use super::swap_amounts_reply::SwapAmountsTxReply;


use crate::helpers::math_helpers::price_rounded;
use crate::pool::handlers;
use crate::token::handlers as token_handlers;
use crate::swap::swap_calc::SwapCalc;

pub fn to_swap_amounts_tx_reply(swap: &SwapCalc) -> Option<SwapAmountsTxReply> {
    let pool = handlers::get_by_pool_id(swap.pool_id)?;
    let pay_token = token_handlers::get_by_token_id(swap.pay_token_id)?;
  //  let pay_chain = pay_token.chain();
    let pay_symbol = pay_token.symbol();
    let pay_address = pay_token.canister_id();
    let receive_token = token_handlers::get_by_token_id(swap.receive_token_id)?;
   // let receive_chain = receive_token.chain();
    let receive_symbol = receive_token.symbol();
    let receive_address = receive_token.canister_id();
    let price = swap.get_price().unwrap_or(BigRational::zero());
    let price_f64 = price_rounded(&price).unwrap_or(0_f64);
    Some(SwapAmountsTxReply {
        pool_symbol: pool.name(),
        pay_symbol,
        pay_address: pay_address.map(|p| p.to_text()).unwrap_or_default(),
        pay_amount: swap.pay_amount.clone(),
        receive_symbol,
        receive_address: receive_address.map(|p| p.to_text()).unwrap_or_default(),
        receive_amount: swap.receive_amount_with_fees_and_gas(),
        price: price_f64,
        lp_fee: swap.lp_fee.clone(),
        gas_fee: swap.gas_fee.clone(),
    })
}