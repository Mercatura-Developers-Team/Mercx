use candid::Nat;
use icrc_ledger_types::icrc1::account::Account;


use crate::helpers::math_helpers::{nat_subtract, nat_zero};
use crate::ic::{transfer::icrc1_transfer};

use crate::token::{stable_token::StableToken};
use crate::transfers::{stable_transfer::StableTransfer, handlers, tx_id::TxId};

//to return the pay_token to the user in a token swap flow — typically when the swap has failed and the user needs to get their funds back. 
pub async fn return_pay_token(
    // request_id: u64,
    // user_id: u32,
    to_principal_id: &Account,
    pay_token: &StableToken,
    pay_amount: &Nat,
   // receive_token: Option<&StableToken>,
    transfer_ids: &mut Vec<u64>,
    ts: u64,
) {
    let token_id = pay_token.token_id();
    let fee = pay_token.fee();

 //   let mut claim_ids = Vec::new();

   // request_map::update_status(request_id, StatusCode::ReturnPayToken, None);

    let pay_amount_with_gas = nat_subtract(pay_amount, &fee).unwrap_or(nat_zero());
    match icrc1_transfer(&pay_amount_with_gas, to_principal_id, pay_token, None).await {
        Ok(tx_id) => {
            let transfer_id = handlers::insert(&StableTransfer {
                transfer_id: 0,
               // request_id,
                is_send: false,
                amount: pay_amount_with_gas,
                token_id,
                tx_id: TxId::BlockIndex(tx_id),
                ts,
            });
            transfer_ids.push(transfer_id);
           // request_map::update_status(request_id, StatusCode::ReturnPayTokenSuccess, None);
        }
        Err(e) => {
            // let claim = StableClaim::new(
            //     user_id,
            //     token_id,
            //     pay_amount,
            //     Some(request_id),
            //     Some(Address::PrincipalId(*to_principal_id)),
            //     ts,
            // );
            // let claim_id = claim_map::insert(&claim);
            // claim_ids.push(claim_id);
            // request_map::update_status(
            //     request_id,
            //     StatusCode::ReturnPayTokenFailed,
            //     Some(&format!("Saved as claim #{}. {}", claim_id, e)),
            // );
            println!("ReturnPayTokenFailed{}",e);
        }
    };

   // let reply = to_swap_reply_failed(request_id, pay_token, pay_amount, receive_token, transfer_ids);
   // request_map::update_reply(request_id, Reply::Swap(reply));
}