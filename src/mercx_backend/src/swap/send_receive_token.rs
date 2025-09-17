use candid::Nat;

use super::swap_calc::SwapCalc;
use super::swap_reply::SwapReply;
use super::swap_reply_helpers::{to_swap_reply_failed};
use crate::transfers::transfer_reply_helpers::to_transfer_ids;
use crate::swap::swap_reply_helpers::to_txs;

use crate::ic::{
    address::Address,
    transfer::{icp_transfer, icrc1_transfer},
};
use crate::token::{stable_token::StableToken};
use crate::transfers::{stable_transfer::{StableTransfer,TransferType}, handlers, tx_id::TxId};
//use crate::stable_tx::{stable_tx::StableTx, swap_tx::SwapTx, tx_map};

pub async fn send_receive_token(
   // request_id: u64,
   // user_id: u32,
    pay_token: &StableToken,
    pay_amount: &Nat,
    receive_token: &StableToken,
    receive_amount: &Nat,
    to_address: &Address, //// Where to send the receive_token (AccountId or PrincipalId)
    transfer_ids: &mut Vec<u64>,
    mid_price: f64,
    price: f64,
    slippage: f64,
    txs: &[SwapCalc],
    ts: u64,
) -> SwapReply {
   // let pay_token_id = pay_token.token_id();
    let receive_token_id = receive_token.token_id();

  //  let mut claim_ids = Vec::new();

   // request_map::update_status(request_id, StatusCode::SendReceiveToken, None);

    // send ICP using icp_transfer or ICRC1 using icrc1_transfer
    match match to_address {
        Address::AccountId(to_account_id) => icp_transfer(receive_amount, to_account_id, receive_token, None).await,
        Address::PrincipalId(to_principal_id) => icrc1_transfer(receive_amount, to_principal_id, receive_token, None).await,
    } {
        Ok(tx_id) => {
            // insert_transfer() will use the latest state of DEPOSIT_MAP so no reentrancy issues after icp_transfer() or icrc1_transfer()
            let transfer_id = handlers::insert(&StableTransfer {
                transfer_id: 0,
             //   request_id,
                is_send: false,
                token_id: receive_token_id,
                amount: receive_amount.clone(),
                tx_id: TxId::BlockIndex(tx_id),
                transfer_type: TransferType::Swap,       // This is a SWAP, not liquidity removal
                ts,
            });
            transfer_ids.push(transfer_id);
           // request_map::update_status(request_id, StatusCode::SendReceiveTokenSuccess, None);
        }
        Err(e) => {
            println!("SendReceiveTokenFailed: {}",e)
            // let claim = StableClaim::new(
            //     user_id,
            //     receive_token_id,
            //     receive_amount,
            //     Some(request_id),
            //     Some(to_address.clone()),
            //     ts,
            // );
            // let claim_id = claim_map::insert(&claim);
            // claim_ids.push(claim_id);
            // request_map::update_status(
            //     request_id,
            //     StatusCode::SendReceiveTokenFailed,
            //     Some(&format!("Saved as claim #{}. {}", claim_id, e)),
            // );
        }
    }

    // let swap_tx = SwapTx::new_success(
    //     user_id,
    //   //  request_id,
    //     pay_token_id,
    //     pay_amount,
    //     receive_token_id,
    //     receive_amount,
    //     mid_price,
    //     price,
    //     slippage,
    //     txs,
    //     transfer_ids,
    //  //   &claim_ids,
    //     ts,
    // );
  //  let tx_id = tx_map::insert(&StableTx::Swap(swap_tx.clone()));
  let reply = match (pay_token.canister_id(), receive_token.canister_id()) {
    (Some(pay_addr), Some(receive_addr)) => SwapReply {
      //  tx_id: 0, // or any dummy/default value if you don’t store transactions
    //    request_id,
     //   status: "success".to_string(),
     //   pay_chain: pay_token.chain().unwrap_or("unknown".to_string()),
        pay_address: pay_addr.to_string(),
        pay_symbol: pay_token.symbol().to_string(),
        pay_amount: pay_amount.clone(),
      //  receive_chain: receive_token.chain().unwrap_or("unknown".to_string()),
        receive_address: receive_addr.to_string(),
        receive_symbol: receive_token.symbol().to_string(),
        receive_amount: receive_amount.clone(),
        mid_price,
        price,
        slippage,
        txs: to_txs(txs, ts),
        transfer_ids: to_transfer_ids(transfer_ids).expect("REASON"),
        //  claim_ids: claim_ids.clone(),
        ts,
    },
    _ => to_swap_reply_failed(
     //   request_id,
        pay_token,
        pay_amount,
        Some(receive_token),
        transfer_ids,
     //   &claim_ids,
        ts,
    ),
};

   // request_map::update_reply(request_id, Reply::Swap(reply.clone()));

    reply
}