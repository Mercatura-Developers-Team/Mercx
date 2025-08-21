use crate::helpers::math_helpers::{nat_divide, nat_is_zero, nat_multiply, nat_subtract, nat_zero,nat_add};
use crate::kyc::kyc_id::get_user_by_caller;
use crate::pool::handlers;
use crate::remove_liquidity::remove_liquidity_args::RemoveLiquidityArgs;
use crate::stable_lp_token::lp_token_map;
use crate::transfers::handlers as transfer_handlers;
use crate::{StableLPToken, StablePool, StableToken,LPToken};
use crate::transfers::tx_id::TxId;
use crate::transfers::stable_transfer::StableTransfer;
use crate::ic::transfer::icrc1_transfer;
use crate::ic::id::caller_id;
use icrc_ledger_types::icrc1::account::Account;
use crate::remove_liquidity::remove_liquidity_reply_helpers::{to_remove_liquidity_reply_failed,to_remove_liquidity_reply};
use crate::remove_liquidity::remove_liquidity_reply::RemoveLiquidityReply;
use crate::ic::general::get_time;
use candid::Nat;

//to calculate how many tokens in the pool and lp_fees the user will recieve
pub fn calculate_amounts(
    pool: &StablePool,
    remove_lp_token_amount: &Nat,
) -> Result<(Nat, Nat, Nat, Nat), String> {
    // Token0
    let balance_0 = &pool.balance_0;
    let lp_fee_0 = &pool.lp_fee_0;
    // Token1
    let balance_1 = &pool.balance_1;
    let lp_fee_1 = &pool.lp_fee_1;
    // LP token
    let lp_token = pool.lp_token();
    let lp_token_id = lp_token.token_id();
    let lp_total_supply = lp_token_map::get_total_supply(lp_token_id);
    // Imagine a pool holds:

    // 1,000 BELLA in liquidity

    // 30 BELLA in fees

    // You have 10% of the LP tokens

    // Then:

    // payout_amount_0 = 10% of 1,000 = 100 BELLA

    // payout_lp_fee_0 = 10% of 30 = 3 BELLA

    // You should receive:
    // 100 + 3 = 103 BELLA when removing liquidity.

    // calculate user's payout in token_0
    // we split the calculations for balance and fees
    // amount_0 = balance_0 * remove_lp_token_amount / lp_total_supply
    let numerator = nat_multiply(balance_0, remove_lp_token_amount);
    let payout_amount_0 =
        nat_divide(&numerator, &lp_total_supply).ok_or("Invalid LP token amount_0")?;
    // payout_lp_fee_0 = lp_fee_0 * remove_lp_token_amount / lp_total_supply
    let numerator = nat_multiply(lp_fee_0, remove_lp_token_amount);
    let payout_lp_fee_0 = nat_divide(&numerator, &lp_total_supply).ok_or("Invalid LP lp_fee_0")?;

    // calculate user's payout in token_1
    // amount_1 = balance_1 * remove_lp_token_amount / lp_total_supply
    let numerator = nat_multiply(balance_1, remove_lp_token_amount);
    let payout_amount_1 =
        nat_divide(&numerator, &lp_total_supply).ok_or("Invalid LP token amount_1")?;
    // payout_lp_fee_1 = lp_fee_1 * remove_lp_token_amount / lp_total_supply
    let numerator = nat_multiply(lp_fee_1, remove_lp_token_amount);
    let payout_lp_fee_1 = nat_divide(&numerator, &lp_total_supply).ok_or("Invalid LP lp_fee_1")?;

    Ok((
        payout_amount_0,
        payout_lp_fee_0,
        payout_amount_1,
        payout_lp_fee_1,
    ))
}

//check that user wants to remove valid amount of lp tokens
async fn check_arguments_with_user(
    args: &RemoveLiquidityArgs,
    user_id: u32,
) -> Result<(StablePool, Nat, Nat, Nat, Nat, Nat), String> {
    // Pool
    let pool = handlers::get_by_tokens(args.token_0.clone(), args.token_1.clone())?;
    // Token0
    let balance_0 = &pool.balance_0;
    // Token1
    let balance_1 = &pool.balance_1;
    // LP token
    let lp_token = pool.lp_token();
    let lp_token_id = lp_token.token_id();

    if nat_is_zero(balance_0) && nat_is_zero(balance_1) {
        Err("Zero balances in pool".to_string())?
    }

    // Check the user has enough LP tokens
    let user_lp_token_amount = lp_token_map::get_by_token_id_by_user_id(lp_token_id, user_id)
        .map_or_else(nat_zero, |lp_token| lp_token.amount);
    //no LP tokens, or
    //trying to remove more LP tokens than they own
    let remove_lp_token_amount = if user_lp_token_amount == nat_zero()
        || args.remove_lp_token_amount > user_lp_token_amount
    {
        Err("User has insufficient LP balance".to_string())?
    } else {
        args.remove_lp_token_amount.clone()
    };

    // calculate the payout amounts.
    let (payout_amount_0, payout_lp_fee_0, payout_amount_1, payout_lp_fee_1) =
        calculate_amounts(&pool, &args.remove_lp_token_amount)?;

    Ok((
        pool,
        remove_lp_token_amount,
        payout_amount_0,
        payout_lp_fee_0,
        payout_amount_1,
        payout_lp_fee_1,
    ))
}

//get user id and send it to calculate
async fn check_arguments(
    args: &RemoveLiquidityArgs,
) -> Result<(u32, StablePool, Nat, Nat, Nat, Nat, Nat), String> {
    // make sure user is not anonymous and exists
    let user_id = get_user_by_caller()
        .await?
        .ok_or("Insufficient LP balance")?
        .user_id;
    let (
        pool,
        remove_lp_token_amount,
        payout_amount_0,
        payout_lp_fee_0,
        payout_amount_1,
        payout_lp_fee_1,
    ) = check_arguments_with_user(args, user_id).await?;

    Ok((
        user_id,
        pool,
        remove_lp_token_amount,
        payout_amount_0,
        payout_lp_fee_0,
        payout_amount_1,
        payout_lp_fee_1,
    ))
}

fn update_liquidity_pool(
    pool: &StablePool,
    amount_0: &Nat,
    lp_fee_0: &Nat,
    amount_1: &Nat,
    lp_fee_1: &Nat,
) {
    //  request_map::update_status(request_id, StatusCode::UpdatePoolAmounts, None);

    let update_pool = StablePool {
        balance_0: nat_subtract(&pool.balance_0, amount_0).unwrap_or(nat_zero()),
        lp_fee_0: nat_subtract(&pool.lp_fee_0, lp_fee_0).unwrap_or(nat_zero()),
        balance_1: nat_subtract(&pool.balance_1, amount_1).unwrap_or(nat_zero()),
        lp_fee_1: nat_subtract(&pool.lp_fee_1, lp_fee_1).unwrap_or(nat_zero()),
        ..pool.clone()
    };
    handlers::update(&update_pool);
    //  request_map::update_status(request_id, StatusCode::UpdatePoolAmountsSuccess, None);
}

//check if user has enough lp tokens to remove and if so update the lp_stable
fn remove_lp_token(
    user_id: u32,
    lp_token: &LPToken,
    remove_lp_token_amount: &Nat,
    ts: u64,
) -> Result<(), String> {
    // LP token
    let lp_token_id = lp_token.token_id();

    // request_map::update_status(request_id, StatusCode::UpdateUserLPTokenAmount, None);

    // make sure user has LP token in ledger and that has enough to remove
    match lp_token_map::get_by_token_id_by_user_id(lp_token_id, user_id) {
        Some(lp_token) => {
            //lp_token.amount → the current LP token balance the user has.

            // remove_lp_token_amount → the amount they want to burn/remove.
            let amount = match nat_subtract(&lp_token.amount, remove_lp_token_amount) {
                Some(amount) => amount,
                None => {
                    let message = format!(
                        "Insufficient LP tokens. {} available, {} required",
                        lp_token.amount, remove_lp_token_amount
                    );
                    //  request_map::update_status(request_id, StatusCode::UpdateUserLPTokenAmountFailed, Some(&message));
                    Err(message)?
                }
            };
            let new_user_lp_token = StableLPToken {
                amount,
                ts,
                ..lp_token.clone()
            };
            lp_token_map::update(&new_user_lp_token);
            //  request_map::update_status(request_id, StatusCode::UpdateUserLPTokenAmountSuccess, None);
            Ok(())
        }
        None => {
            let message = format!(
                "Insufficient LP tokens. 0 available, {} required",
                remove_lp_token_amount
            );
            // request_map::update_status(request_id, StatusCode::UpdateUserLPTokenAmountFailed, Some(&message));
            Err(message)?
        }
    }
}

//take args and transfer the token back to users(lp) and update the stable transfer if it didn't fail
async fn transfer_token(
    user_id: u32,
    to_principal_id: &Account,
    token: &StableToken,
    payout_amount: &Nat, //user’s share of reserves
    payout_lp_fee: &Nat,//// Extra payout (e.g., their accrued LP fees)
    transfer_ids: &mut Vec<u64>,
    ts: u64,
) {
    let token_id = token.token_id();

    // total payout = amount + lp_fee - gas fee
    let amount = nat_add(payout_amount, payout_lp_fee);
    let amount_with_gas = nat_subtract(&amount, &token.fee()).unwrap_or(nat_zero());

    match icrc1_transfer(&amount_with_gas, to_principal_id, token, None).await {
        Ok(block_id) => {
            let transfer_id = transfer_handlers::insert(&StableTransfer {
                transfer_id: 0,
                is_send: false,
                amount: amount_with_gas,
                token_id,
                tx_id: TxId::BlockIndex(block_id),
                ts,
            });
            transfer_ids.push(transfer_id);
     
        }
        Err(e) => {
            ic_cdk::println!(
                "❌ transfer_token: ledger error | user_id={}, token_id={}, err={}",
                user_id, token_id, e
            );

        }
    }
}


// send payout tokens to user and final balance integrity checks
// - send payout token_0 and token_1 to user
// - check the actual balances of the canister vs. expected balances in stable memory
async fn send_payout_tokens(
    user_id: u32,
    to_principal_id: &Account,
    pool: &StablePool,
    payout_amount_0: &Nat,
    payout_lp_fee_0: &Nat,
    payout_amount_1: &Nat,
    payout_lp_fee_1: &Nat,
    remove_lp_token_amount: &Nat, //what user wants
    ts: u64,
) -> Result<RemoveLiquidityReply, String> {
    // Token0
    let token_0 = pool.token_0();
    // Token1
    let token_1 = pool.token_1();

    let mut transfer_ids = Vec::new();

    // send payout token_0 to the user
    transfer_token(
        user_id,
        to_principal_id,
        &token_0,
        payout_amount_0,
        payout_lp_fee_0,
        &mut transfer_ids,
        ts,
    )
    .await;

    // send payout token_1 to the user
    transfer_token(
        user_id,
        to_principal_id,
        &token_1,
        payout_amount_1,
        payout_lp_fee_1,
        &mut transfer_ids,
        ts,
    )
    .await;

    let reply = if !transfer_ids.is_empty() {
        to_remove_liquidity_reply(
            pool.pool_id,
            payout_amount_0.clone(),
            payout_lp_fee_0.clone(),
            payout_amount_1.clone(),
            payout_lp_fee_1.clone(),
            remove_lp_token_amount.clone(),
            transfer_ids.clone(),
            ts,
        )
    } else {
        to_remove_liquidity_reply_failed(pool.pool_id, ts)
    };
    
    Ok(reply)
}


//This return_lp_token function is responsible for returning LP tokens to a user — typically when a liquidity removal fails and you want to give the LP tokens back to the user.
//it calculates and update stable lp 
fn return_lp_token(user_id: u32, lp_token: &LPToken, remove_lp_token_amount: &Nat, ts: u64) -> Result<(), String> {
    // LP token
    let lp_token_id = lp_token.token_id();

    match lp_token_map::get_by_token_id_by_user_id(lp_token_id, user_id) {
        Some(lp_token) => {
            let new_user_lp_token = StableLPToken {
                amount: nat_add(&lp_token.amount, remove_lp_token_amount),
                ts,
                ..lp_token.clone()
            };
            lp_token_map::update(&new_user_lp_token);
            Ok(())
        }
        None => Err("Unable to find LP tokens balance".to_string())?,
    }
}

//return lp to users back 
fn return_tokens(
    user_id: u32,
    pool: &StablePool,
    transfer_lp_token: &Result<(), String>,
    remove_lp_token_amount: &Nat,
    ts: u64,
) {
    // LP token
    let lp_token = pool.lp_token();

    // if transfer_lp_token was successful, then we need to return the LP token back to the user
    if transfer_lp_token.is_ok() {
        //request_map::update_status(request_id, StatusCode::ReturnUserLPTokenAmount, None);
        match return_lp_token(user_id, &lp_token, remove_lp_token_amount, ts) {
            Ok(()) => {
              //  request_map::update_status(request_id, StatusCode::ReturnUserLPTokenAmountSuccess, None);
            }
            Err(_) => {
                to_remove_liquidity_reply_failed(pool.pool_id, ts);
               // request_map::update_status(request_id, StatusCode::ReturnUserLPTokenAmountFailed, Some(&e));
            }
        }
    }

   // let reply = to_remove_liquidity_reply_failed(pool.pool_id, ts);
  //  request_map::update_reply(request_id, Reply::RemoveLiquidity(reply));
}

async fn process_remove_liquidity(
    user_id: u32,
    to_principal_id: &Account,
    pool: &StablePool,
    remove_lp_token_amount: &Nat,
    payout_amount_0: &Nat,
    payout_lp_fee_0: &Nat,
    payout_amount_1: &Nat,
    payout_lp_fee_1: &Nat,
    ts: u64,
) -> Result<RemoveLiquidityReply, String> {
    // LP token
    let lp_token = pool.lp_token();

   // request_map::update_status(request_id, StatusCode::Start, None);

    // remove LP tokens from user's ledger
    let transfer_lp_token = remove_lp_token( user_id, &lp_token, remove_lp_token_amount, ts);
    if transfer_lp_token.is_err() {
        return_tokens( user_id, pool, &transfer_lp_token, remove_lp_token_amount, ts);
       // Err(format!("Req #{} failed. {}", request_id, transfer_lp_token.unwrap_err()))?
    }

    // update liquidity pool with new removed amounts
    update_liquidity_pool(pool, payout_amount_0, payout_lp_fee_0, payout_amount_1, payout_lp_fee_1);

    // successful, add tx and update request with reply
    send_payout_tokens(
        user_id,
        to_principal_id,
        pool,
        payout_amount_0,
        payout_lp_fee_0,
        payout_amount_1,
        payout_lp_fee_1,
        remove_lp_token_amount,
        ts,
    )
    .await
}

/// remove liquidity from a pool
/// - before calling remove_liquidity(), the user must create an icrc2_approve_transaction for the LP token to
///   allow the backend canister to icrc2_transfer_from. Note, the approve transaction will incur
///   gas fees - which is 1 for LP tokens. However, the icrc2_transfer_from to the backend canister is considered
///   a burn and does not incur gas fees.
///
/// Notes regarding gas:
///   - payout_amount_0, payout_lp_fee_0, payout_amount_1, payout_lp_fee_1 does not include gas fees
#[ic_cdk::update]
pub async fn remove_liquidity(args: RemoveLiquidityArgs) -> Result<RemoveLiquidityReply, String> {
    let (user_id, pool, remove_lp_token_amount, payout_amount_0, payout_lp_fee_0, payout_amount_1, payout_lp_fee_1) =
        check_arguments(&args).await?;
    let ts = get_time();
    //let request_id = request_map::insert(&StableRequest::new(user_id, &Request::RemoveLiquidity(args), ts));
    let caller_id = caller_id();

    let result = match process_remove_liquidity(
        user_id,
        &caller_id,
        &pool,
        &remove_lp_token_amount,
        &payout_amount_0,
        &payout_lp_fee_0,
        &payout_amount_1,
        &payout_lp_fee_1,
        ts,
    )
    .await
    {
        Ok(reply) => {
            Ok(reply)
        }
        Err(e) => {
            Err(e)
        }
    };


    result
}

/// used by remove_lp_positions() to remove_liquidity for user_id and tokens returned to to_principal_id
pub async fn remove_liquidity_from_pool(
    args: RemoveLiquidityArgs,
    user_id: u32,
    to_principal_id: &Account,
) -> Result<RemoveLiquidityReply, String> {
    let (pool, remove_lp_token_amount, payout_amount_0, payout_lp_fee_0, payout_amount_1, payout_lp_fee_1) =
        check_arguments_with_user(&args, user_id).await?;
    let ts = get_time();

    let result = match process_remove_liquidity(
        user_id,
        to_principal_id,
        &pool,
        &remove_lp_token_amount,
        &payout_amount_0,
        &payout_lp_fee_0,
        &payout_amount_1,
        &payout_lp_fee_1,
        ts,
    )
    .await
    {
        Ok(reply) => {
            Ok(reply)
        }
        Err(e) => {
            Err(e)
        }
    };

    result
}