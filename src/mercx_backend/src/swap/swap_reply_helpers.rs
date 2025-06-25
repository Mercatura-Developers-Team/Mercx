use candid::Nat;
use num::{BigRational, Zero};

use super::swap_calc::SwapCalc;
use super::swap_reply::{SwapReply, SwapTxReply};

use crate::helpers::math_helpers::price_rounded;
use crate::helpers::math_helpers::nat_zero;
use crate::pool::handlers;
use crate::token::stable_token::StableToken;
use crate::token::handlers as token_handler;
// use crate::stable_tx::status_tx::StatusTx;
// use crate::stable_tx::swap_tx::SwapTx;
use crate::transfers::transfer_reply_helpers::to_transfer_ids;

fn to_swap_tx_reply(swap: &SwapCalc, ts: u64) -> Option<SwapTxReply> {
    let pool = handlers::get_by_pool_id(swap.pool_id)?;
    let pay_token = token_handler::get_by_token_id(swap.pay_token_id)?;
   // let pay_chain = pay_token.chain().to_string();
    let pay_address: String = pay_token.canister_id()?.to_string();
    let pay_symbol = pay_token.symbol().to_string();
    let receive_token = token_handler::get_by_token_id(swap.receive_token_id)?;
   // let receive_chain = receive_token.chain().to_string();
    let receive_address: String = receive_token.canister_id()?.to_string();
    let receive_symbol = receive_token.symbol().to_string();
    let price = swap.get_price().unwrap_or(BigRational::zero());
    let price_f64 = price_rounded(&price).unwrap_or(0_f64);
    Some(SwapTxReply {
        pool_symbol: pool.name(),
       // pay_chain,
        pay_address,
        pay_symbol,
        pay_amount: swap.pay_amount.clone(),
      //  receive_chain,
        receive_address,
        receive_symbol,
        receive_amount: swap.receive_amount.clone(),
        price: price_f64,
        lp_fee: swap.lp_fee.clone(),
        gas_fee: swap.gas_fee.clone(),
        ts,
    })
}

//txs:A slice of SwapCalc objects — each one represents a swap calculation (i.e., what the user will pay/receive, the fees, the pool used, etc.).
fn to_txs(txs: &[SwapCalc], ts: u64) -> Vec<SwapTxReply> {
    txs.iter().filter_map(|tx| to_swap_tx_reply(tx, ts)).collect()
}

fn get_tokens_info(pay_token_id: u32, receive_token_id: u32) -> ( String, String, String, String) {
    let pay_token = token_handler::get_by_token_id(pay_token_id);
    let (pay_address, pay_symbol) = pay_token.map_or_else(
        || {
            (
                "Pay address not found".to_string(),
                "Pay symbol not found".to_string(),
            )
        },
        |token| ( token.canister_id().map(|id| id.to_string()).unwrap_or("Pay address not found".to_string()), token.symbol().to_string()),
    );
    let receive_token = token_handler::get_by_token_id(receive_token_id);
    let ( receive_address, receive_symbol) = receive_token.map_or_else(
        || {
            (
                "Receive address not found".to_string(),
                "Receive symbol not found".to_string(),
            )
        },
        |token| ( token.canister_id().map(|id| id.to_string()).unwrap_or("Pay address not found".to_string()), token.symbol().to_string()),
    );
    ( pay_address, pay_symbol, receive_address, receive_symbol)
}

// pub fn to_swap_reply(swap: &SwapCalc, ts: u64) -> SwapReply {
//     let (pay_chain, pay_address, pay_symbol, receive_chain, receive_address, receive_symbol) =
//     get_tokens_info(pay_token_id, receive_token_id);
//     SwapReply {
//      //   tx_id: swap_tx.tx_id,
//      //   request_id: swap_tx.request_id,
//       //  status: swap_tx.status.to_string(),
//       //  pay_chain,
//         pay_address,
//         pay_symbol,
//         pay_amount: swap.pay_amount.clone(),
//         receive_address,
//         receive_symbol,
//         receive_amount: swap.receive_amount.clone(),
//         mid_price: swap_tx.mid_price,
//         price: swap_tx.price,
//         slippage: swap_tx.slippage,
//         txs: to_txs(&swap_tx.txs, swap_tx.ts),
//         transfer_ids: to_transfer_ids(&swap_tx.transfer_ids).expect("REASON"),
//         ts: swap_tx.ts,
//     }
// }


pub fn to_swap_reply_failed(
   // request_id: u64,
    pay_token: &StableToken,
    pay_amount: &Nat,
    receive_token: Option<&StableToken>,
    transfer_ids: &[u64],
   // claim_ids: &[u64],
    ts: u64,
) -> SwapReply {
    // Pay Token
   // let pay_chain = pay_token.chain().to_string();
    let pay_address = pay_token.canister_id().expect("canister id not found").to_string();
    let pay_symbol = pay_token.symbol().to_string();
    // Receive token
  //  let receive_chain = receive_token.map_or_else(|| "Receive chain not found".to_string(), |token| token.chain().to_string());
    let receive_address = receive_token.map_or_else(|| "Receive address not found".to_string(), |token| token.canister_id().expect("canister id not found").to_string());
    let receive_symbol = receive_token.map_or_else(|| "Receive symbol not found".to_string(), |token| token.symbol().to_string());
    SwapReply {
        tx_id: 0,
        pay_address,
        pay_symbol,
        pay_amount: pay_amount.clone(),
        receive_address,
        receive_symbol,
        receive_amount: nat_zero(),
        mid_price: 0_f64,
        price: 0_f64,
        slippage: 0_f64,
        txs: Vec::new(),
        transfer_ids: to_transfer_ids(transfer_ids).expect("REASON"),
        ts,
    }
}