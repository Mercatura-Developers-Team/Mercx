use candid::Nat;
use ic_cdk::query;

use super::swap_amounts_reply::SwapAmountsReply;
use super::swap_amounts_reply_impl::to_swap_amounts_tx_reply;


use crate::token::handlers;
use crate::swap;

#[query]
pub fn swap_amounts(pay_token: String, pay_amount: Nat, receive_token: String) -> Result<SwapAmountsReply, String> {
    // Pay token
    let pay_token = handlers::get_by_token(&pay_token)?;
    //let pay_chain = pay_token.chain();
    let pay_symbol = pay_token.symbol();
    let pay_address = pay_token.canister_id();
    // Receive token
    let receive_token = handlers::get_by_token(&receive_token)?;
    //let receive_chain = receive_token.chain();
    let receive_symbol = receive_token.symbol();
    let receive_address = receive_token.canister_id();

    let (receive_amount, price, mid_price, slippage, txs) =
        swap::swap_amounts::swap_amounts(&pay_token, Some(&pay_amount), &receive_token)?;
    let swap_amounts_tx_reply: Vec<_> = txs.iter().filter_map(to_swap_amounts_tx_reply).collect();

    Ok(
        
        
        {SwapAmountsReply {
        pay_symbol,
        pay_amount,
        pay_address: pay_address.map(|p| p.to_text()).unwrap_or_default(),
        receive_symbol,
        receive_address: receive_address.map(|p| p.to_text()).unwrap_or_default(),
        receive_amount,
        price,
        mid_price,
        slippage,
        txs: swap_amounts_tx_reply,
    }})
}