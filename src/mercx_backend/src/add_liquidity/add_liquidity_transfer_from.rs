use candid::Nat;

use crate::helpers::math_helpers::{
    nat_add, nat_divide, nat_is_zero, nat_multiply, nat_to_decimal_precision,
};

use crate::add_liquidity::add_liquidity_args::AddLiquidityArgs;
use crate::add_liquidity::add_liquidity_reply::AddLiquidityReply;
use crate::add_liquidity::add_liquidity_reply_helpers::to_add_liquidity_reply;
use crate::ic::general::get_time;
use crate::ic::id::caller_id;
use crate::pool::add_pool_arg::{return_token, transfer_from_token};
use crate::pool::handlers;
use crate::stable_mercx_settings::mercx_settings_map;
use crate::token::handlers::exists_by_canister_id;
use crate::StablePool;
use icrc_ledger_types::icrc1::account::Account;



#[ic_cdk::update]
pub async fn add_liquidity_transfer_from(
    args: AddLiquidityArgs,
) -> Result<AddLiquidityReply, String> {
    let (pool, add_amount_0, add_amount_1) = check_arguments(&args).await?;
    let ts = get_time();

    let result = match process_add_liquidity(&pool, &add_amount_0, &add_amount_1, ts).await {
        Ok(reply) => Ok(reply),
        Err(e) => Err(e),
    };

    result
}

async fn process_add_liquidity(
    pool: &StablePool,
    add_amount_0: &Nat,
    add_amount_1: &Nat,
    ts: u64,
) -> Result<AddLiquidityReply, String> {
    // Token0
    let token_0 = pool.token_0();
    // Token1
    let token_1 = pool.token_1();

    let caller_id = caller_id();
    let mercx_backend = mercx_settings_map::get().mercx_backend;
    let mut transfer_ids = Vec::new();

    // transfer_from token_0. if this fails, nothing to return so just return the error
    transfer_from_token(
        &caller_id,
        &token_0,
        add_amount_0,
        &mercx_backend,
        &mut transfer_ids,
        ts,
    )
    .await
    .map_err(|e| format!("Token_0 transfer_from failed. {}", e))?;

    // transfer_from token_1. if this fails, return token_0 back to user
    if let Err(e) = transfer_from_token(
        &caller_id,
        &token_1,
        add_amount_1,
        &mercx_backend,
        &mut transfer_ids,
        ts,
    )
    .await
    {
        return_tokens(
            &caller_id,
            pool,
            Some(add_amount_0),
            None,
            &mut transfer_ids,
            ts,
        )
        .await;
        return Err(format!("Token_1 transfer_from failed. {}", e));
    };

    // re-calculate with latest pool state and make sure amounts are valid
    let (pool, amount_0, amount_1) = match update_liquidity_pool(pool, add_amount_0, add_amount_1) {
        Ok((pool, amount_0, amount_1)) => (pool, amount_0, amount_1),
        Err(e) => {
            // LP amounts are incorrect. return token_0 and token_1 back to user
            return_tokens(
                &caller_id,
                pool,
                Some(add_amount_0),
                Some(add_amount_1),
                &mut transfer_ids,
                ts,
            )
            .await;
            return Err(format!("failed. {}", e));
        }
    };

    Ok(to_add_liquidity_reply(
        &pool,
        &token_0,
        &token_1,
        &transfer_ids,
    ))
}

async fn check_arguments(args: &AddLiquidityArgs) -> Result<(StablePool, Nat, Nat), String> {
    if nat_is_zero(&args.amount_0) || nat_is_zero(&args.amount_1) {
        Err("Invalid zero amounts".to_string())?
    }

    // check to make sure tx_id_0 and tx_id_1 is not specified
    if args.tx_id_0.is_some() || args.tx_id_1.is_some() {
        Err("Tx_id_0 and Tx_id_1 not supported".to_string())?
    }

    // add_amount_0 and add_amount_1 are the amounts to be added to the pool with the current state
    // these are the amounts that will be transferred to the pool
    let (pool, add_amount_0, add_amount_1) =
        calculate_amounts(&args.token_0, &args.amount_0, &args.token_1, &args.amount_1)?;

    let token_0 = pool.token_0();
    let token0 = exists_by_canister_id(token_0.canister_id().expect("Missing canister_id"));
    if !token0 {
        Err("Token_0 is suspended or removed".to_string())?
    }
    if !token_0.is_icrc2() {
        Err("Token_0 must support ICRC2".to_string())?
    }

    let token_1 = pool.token_1();
    let token1 = exists_by_canister_id(token_1.canister_id().expect("Missing canister_id"));
    if !token1 {
        Err("Token_1 is suspended or removed".to_string())?
    }
    if !token_1.is_icrc2() {
        Err("Token_1 must support ICRC2".to_string())?
    }

    // make sure user is registered, if not create a new user
    // let user_id = user_map::insert(None)?;

    Ok((pool, add_amount_0, add_amount_1))
}

/// calculate the ratio of amounts (amount_0 and amount_1) to be added to the pool to maintain constant K
/// calculate the LP token amount for the user
///
/// returns (pool, amount_0, amount_1, add_lp_token_amount)
pub fn calculate_amounts(
    token_0: &str,
    amount_0: &Nat,
    token_1: &str,
    amount_1: &Nat,
) -> Result<(StablePool, Nat, Nat), String> {
    // Pool - make sure pool exists, refresh balances of the pool to make sure we have the latest state
    let pool = handlers::get_by_tokens(token_0, token_1)?;
    // Token0
    let token_0 = pool.token_0();
    // reserve_0 is the total balance of token_0 in the pool = balance_0 + lp_fee_0
    let reserve_0 = nat_add(&pool.balance_0, &pool.lp_fee_0);
    // Token1
    let token_1 = pool.token_1();
    let reserve_1 = nat_add(&pool.balance_1, &pool.lp_fee_1);
    // LP token
    // let lp_token = pool.lp_token();
    // let lp_token_id = lp_token.token_id();
    // let lp_total_supply = lp_token_map::get_total_supply(lp_token_id);

    if nat_is_zero(&reserve_0) || nat_is_zero(&reserve_1) {
        // new pool as there are no balances - take user amounts as initial ratio
        // initialize LP tokens as sqrt(amount_0 * amount_1)
        // convert the amounts to the same decimal precision as the LP token
        // let amount_0_in_lp_token_decimals = nat_to_decimal_precision(amount_0, token_0.decimals(), lp_token.decimals());
        // let amount_1_in_lp_token_decimals = nat_to_decimal_precision(amount_1, token_1.decimals(), lp_token.decimals());
        // let add_lp_token_amount = nat_sqrt(&nat_multiply(&amount_0_in_lp_token_decimals, &amount_1_in_lp_token_decimals));
        return Ok((pool, amount_0.clone(), amount_1.clone()));
    }

    // amount_0 * reserve_1 = amount_1 * reserve_0 for constant K
    let amount_0_reserve_1 = nat_multiply(amount_0, &reserve_1);
    let amount_1_reserve_0 = nat_multiply(amount_1, &reserve_0);
    // if the ratio of the user amounts is the same as the pool ratio, then the amounts are correct
    // rarely happens as there are rounding precision errors
    if amount_0_reserve_1 == amount_1_reserve_0 {
        // calculate the LP token amount for the user
        // add_lp_token_amount = lp_total_supply * amount_0 / reserve_0
        // let amount_0_in_lp_token_decimals = nat_to_decimal_precision(amount_0, token_0.decimals(), lp_token.decimals());
        // let reserve_0_in_lp_token_decimals = nat_to_decimal_precision(&reserve_0, token_0.decimals(), lp_token.decimals());
        // let numerator_in_lp_token_decimals = nat_multiply(&lp_total_supply, &amount_0_in_lp_token_decimals);
        // let add_lp_token_amount =
        //     nat_divide(&numerator_in_lp_token_decimals, &reserve_0_in_lp_token_decimals).ok_or("Invalid LP token amount")?;
        return Ok((pool, amount_0.clone(), amount_1.clone()));
    }

    // determine if the ratio of the user amounts is same or greater than the pool ratio (reserve_1 / reserve_0)
    // using amount_0 to calculate the amount_1 that should be added to the pool
    // amount_1 = amount_0 * reserve_1 / reserve_0
    // convert amount_0 and reserve_0 to token_1 decimal precision
    let amount_0_in_token_1_decimals =
        nat_to_decimal_precision(amount_0, token_0.decimals(), token_1.decimals());
    let reserve_0_in_token_1_decimals =
        nat_to_decimal_precision(&reserve_0, token_0.decimals(), token_1.decimals());
    // amount_0 * reserve_1 - do the multiplication first before divison to avoid loss of precision
    let numerator_in_token_1_decimals = nat_multiply(&amount_0_in_token_1_decimals, &reserve_1);
    let amount_1_in_token_1_decimals = nat_divide(
        &numerator_in_token_1_decimals,
        &reserve_0_in_token_1_decimals,
    )
    .ok_or("Invalid amount_1")?;
    // if amount_1 is equal or greater than calculated by the pool ratio, then use amount_0 and amount_1
    if *amount_1 >= amount_1_in_token_1_decimals {
        // calculate the LP token amount for the user
        // add_lp_token_amount = lp_total_supply * amount_0 / reserve_0
        // let amount_0_in_lp_token_decimals = nat_to_decimal_precision(amount_0, token_0.decimals(), lp_token.decimals());
        // let reserve_0_in_lp_token_decimals = nat_to_decimal_precision(&reserve_0, token_0.decimals(), lp_token.decimals());
        // let numerator_in_lp_token_decimals = nat_multiply(&lp_total_supply, &amount_0_in_lp_token_decimals);
        // let add_lp_token_amount =
        //     nat_divide(&numerator_in_lp_token_decimals, &reserve_0_in_lp_token_decimals).ok_or("Invalid LP token amount")?;
        return Ok((pool, amount_0.clone(), amount_1_in_token_1_decimals));
    }

    // using amount_1 to calculate the amount_0 that should be added to the pool
    // amount_0 = amount_1 * reserve_0 / reserve_1
    let amount_1_in_token_0_decimals =
        nat_to_decimal_precision(amount_1, token_1.decimals(), token_0.decimals());
    let reserve_1_in_token_0_decimals =
        nat_to_decimal_precision(&reserve_1, token_1.decimals(), token_0.decimals());
    let numerator_in_token_0_decimals = nat_multiply(&amount_1_in_token_0_decimals, &reserve_0);
    let amount_0_in_token_0_decimals = nat_divide(
        &numerator_in_token_0_decimals,
        &reserve_1_in_token_0_decimals,
    )
    .ok_or("Invalid amount_0")?;
    if *amount_0 >= amount_0_in_token_0_decimals {
        // let amount_1_in_lp_token_decimals = nat_to_decimal_precision(amount_1, token_1.decimals(), lp_token.decimals());
        // let reserve_1_in_lp_token_decimals = nat_to_decimal_precision(&reserve_1, token_1.decimals(), lp_token.decimals());
        // let numerator_in_lp_token_decimals = nat_multiply(&lp_total_supply, &amount_1_in_lp_token_decimals);
        // let add_lp_token_amount =
        //     nat_divide(&numerator_in_lp_token_decimals, &reserve_1_in_lp_token_decimals).ok_or("Invalid LP token amount")?;
        return Ok((pool, amount_0_in_token_0_decimals, amount_1.clone()));
    }

    // pool ratio must have changed from initial calculation and amount_0 and amount_1 are not enough now
    Err("Incorrect ratio of amount_0 and amount_1".to_string())
}

/// update the liquidity pool with the new liquidity amounts
/// ensure we have the latest state of the pool before adding the new amounts
/// //update balance
pub fn update_liquidity_pool(
    pool: &StablePool,
    add_amount_0: &Nat,
    add_amount_1: &Nat,
) -> Result<(StablePool, Nat, Nat), String> {
    let token_0 = pool
        .token_0()
        .canister_id()
        .expect("Missing canister_id")
        .to_string();
    let token_1 = pool
        .token_1()
        .canister_id()
        .expect("Missing canister_id")
        .to_string();
    // re-calculate the amounts to be added to the pool with new state (after token_0 and token_1 transfers)
    // add_amount_0 and add_amount_1 are the transferred amounts from the initial calculations
    // amount_0, amount_1 and add_lp_token_amount will be the actual amounts to be added to the pool
    match calculate_amounts(&token_0, add_amount_0, &token_1, add_amount_1) {
        Ok((mut pool, amount_0, amount_1)) => {
            pool.balance_0 = nat_add(&pool.balance_0, &amount_0);
            pool.balance_1 = nat_add(&pool.balance_1, &amount_1);
            handlers::update(&pool);

            // update user's LP token amount
            // update_lp_token(request_id, user_id, pool.lp_token_id, &add_lp_token_amount, ts);

            Ok((pool, amount_0, amount_1))
        }
        Err(e) => Err(e),
    }
}

async fn return_tokens(
    to_principal_id: &Account,
    pool: &StablePool,
    amount_0: Option<&Nat>,
    amount_1: Option<&Nat>,
    transfer_ids: &mut Vec<u64>,
    ts: u64,
) {
    //  let mut claim_ids = Vec::new();

    if let Some(amount_0) = amount_0 {
        let token_0 = pool.token_0();
        return_token(to_principal_id, &token_0, amount_0, transfer_ids, ts).await;
    }

    if let Some(amount_1) = amount_1 {
        let token_1 = pool.token_1();
        return_token(to_principal_id, &token_1, amount_1, transfer_ids, ts).await;
    }

    //let reply = to_add_liquidity_reply_failed(pool.pool_id, request_id, transfer_ids, ts);
}
