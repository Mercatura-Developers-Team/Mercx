use crate::swap::calculate_amounts::calculate_amounts;
//use std::time::Duration;
use crate::ic::general::get_time;
use crate::helpers::math_helpers::nat_is_zero;
use crate::ic::id::caller_id;
use crate::ic::address::Address;
use crate::StableToken;
use candid::Nat;
use crate::swap::swap_args::SwapArgs;
use crate::stable_mercx_settings::mercx_settings_map;
use crate::token::handlers; 
use crate::ic::address_helpers::get_address;
use crate::transfers::tx_id::TxId;
use crate::transfers::stable_transfer::StableTransfer;
use crate::ic::transfer::icrc2_transfer_from;
use icrc_ledger_types::icrc1::account::Account;
use crate::transfers::handlers as transfer_handlers;
use crate::swap::update_liquidity_pool::update_liquidity_pool;
use crate::swap::return_pay_token::return_pay_token; 
use crate::swap::swap_calc::SwapCalc;
use crate::swap::send_receive_token::send_receive_token;
use crate::swap::swap_reply::SwapReply; 

pub async fn swap_transfer_from(args: SwapArgs) -> Result<SwapReply, String> {
    let ( pay_token, pay_amount, receive_token, max_slippage, to_address) = check_arguments(&args).await?;
    let ts = get_time();
    let receive_amount = args.receive_amount.clone();
  //  let request_id = request_map::insert(&StableRequest::new(user_id, &Request::Swap(args), ts));
    let mut transfer_ids = Vec::new();

    let (receive_amount_with_fees_and_gas, mid_price, price, slippage, swaps) = process_swap(
       // request_id,

        &pay_token,
        &pay_amount,
        &receive_token,
        receive_amount.as_ref(),
        max_slippage,
        &mut transfer_ids,
        ts,
    )
    .await
    .inspect_err(|_| {
      //  request_map::update_status(request_id, StatusCode::Failed, None);
      //  increase_consecutive_error(user_id);
     //   let _ = archive_to_kong_data(request_id);
    })?;

    let result = send_receive_token(
        &pay_token,
        &pay_amount,
        &receive_token,
        &receive_amount_with_fees_and_gas,
        &to_address,
        &mut transfer_ids,
        mid_price,
        price,
        slippage,
        &swaps,
        ts,
    )
    .await;

    // request_map::update_status(request_id, StatusCode::Success, None);
    // reset_consecutive_error(user_id);
    // let _ = archive_to_kong_data(request_id);

    Ok(result)
}


async fn check_arguments(args: &SwapArgs) -> Result<( StableToken, Nat, StableToken, f64, Address), String> {
    let pay_token = handlers::get_by_token(&args.pay_token)?;


    let pay_amount = args.pay_amount.clone();

    let receive_token = handlers::get_by_token(&args.receive_token)?;


    // use specified max slippage or use default
    let max_slippage = args.max_slippage.unwrap_or(mercx_settings_map::get().default_max_slippage);
    // use specified address or default to caller's principal id
    let to_address = match args.receive_address {
        Some(ref address) => get_address(&receive_token, address)?,
        None => Address::PrincipalId(caller_id()),
    };
    if nat_is_zero(&pay_amount) {
        Err("Pay amount is zero".to_string())?;
    }

    // check to make sure pay_tx_id is not specified
    if args.pay_tx_id.is_some() {
        Err("Pay tx_id not supported".to_string())?;
    }

    if !pay_token.is_icrc2() {
        Err("Pay token must support ICRC2".to_string())?;
    }

    // make sure user is registered, if not create a new user with referred_by if specified
    // let user_id = user_map::insert(args.referred_by.as_deref())?;
    // // check if user is banned
    // if let Some(banned_until) = is_banned_user(user_id) {
    //     let now = get_time();
    //     if banned_until > now {
    //         let duration_ns = Duration::from_nanos(banned_until - now);
    //         let duration_min = duration_ns.as_secs() / 60;
    //         Err(format!("Too many consecutive errors. User is banned for {} minutes", duration_min))?;
    //     }
    // }

    // calculate receive_amount and swaps. do after user_id is created as it will be needed to calculate the receive_amount (user fee level)
    // no needs to store the return values as it'll be called again in process_swap
    calculate_amounts(&pay_token, &pay_amount, &receive_token, args.receive_amount.as_ref(), max_slippage)?;

    Ok(( pay_token, pay_amount, receive_token, max_slippage, to_address))
}

async fn transfer_from_token(
    from_principal_id: &Account,
    token: &StableToken,
    amount: &Nat,
    to_principal_id: &Account,
    transfer_ids: &mut Vec<u64>,
    ts: u64,
) -> Result<(), String> {
    let token_id = token.token_id();


    match icrc2_transfer_from(token, amount, from_principal_id, to_principal_id).await {
        Ok(tx_id) => {
            // insert_transfer() will use the latest state of TRANSFER_MAP to prevent reentrancy issues after icrc2_transfer_from()
            // as icrc2_transfer_from() does a new transfer so tx_id will be new
            let transfer_id = transfer_handlers::insert(&StableTransfer {
                transfer_id: 0,
                is_send: true,
                amount: amount.clone(),
                token_id,
                tx_id: TxId::BlockIndex(tx_id),
                ts,
            });
            transfer_ids.push(transfer_id);
            //request_map::update_status(request_id, StatusCode::SendPayTokenSuccess, None);
            Ok(())
        }
        Err(e) => {
           // request_map::update_status(request_id, StatusCode::SendPayTokenFailed, Some(&e));
            Err(e)
        }
    }
}

// swaps needs to be passed in to get the pool of the pay token which is needed to determine if the
// pay_tx_id is a double spend
#[allow(clippy::too_many_arguments)]
async fn process_swap(

    pay_token: &StableToken,
    pay_amount: &Nat,
    receive_token: &StableToken,
    receive_amount: Option<&Nat>,
    max_slippage: f64,
    transfer_ids: &mut Vec<u64>,
    ts: u64,
) -> Result<(Nat, f64, f64, f64, Vec<SwapCalc>), String> {
    let caller_id = caller_id();
    let mercx_backend = mercx_settings_map::get().mercx_backend;

    //request_map::update_status(request_id, StatusCode::Start, None);
//from user to backend caniste
    transfer_from_token(&caller_id, pay_token, pay_amount, &mercx_backend, transfer_ids, ts)
        .await
        .map_err(|e| format!("Pay token transfer_from failed. {}", e))?;

    // re-calculate receive_amount and swaps with the latest pool state
    let (receive_amount_with_fees_and_gas, mid_price, price, slippage, swaps) =
        match update_liquidity_pool( pay_token, pay_amount, receive_token, receive_amount, max_slippage) {
            Ok((receive_amount, mid_price, price, slippage, swaps)) => (receive_amount, mid_price, price, slippage, swaps),
            Err(e) => {
                // return pay token back to user
                return_pay_token(
                  //  user_id,
                    &caller_id,
                    pay_token,
                    pay_amount,
                  //  Some(receive_token),
                    transfer_ids,
                    ts,
                )
                .await;
                Err(format!(" failed. {}",  e))?
            }
        };


    Ok((receive_amount_with_fees_and_gas, mid_price, price, slippage, swaps))
}