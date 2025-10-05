use candid::{Nat,Principal};
use crate::helpers::math_helpers::{
    nat_add, nat_divide, nat_is_zero, nat_multiply, nat_to_decimal_precision,nat_sqrt
};
use crate::add_liquidity::add_liquidity_args::AddLiquidityArgs;
use crate::add_liquidity::add_liquidity_reply::AddLiquidityReply;
use crate::add_liquidity::add_liquidity_reply_helpers::{to_add_liquidity_reply,to_add_liquidity_reply_failed};
use crate::ic::general::get_time;
use crate::ic::id::caller_id;
use crate::pool::add_pool_arg::{return_token, transfer_from_token};
use crate::pool::handlers;
use crate::stable_mercx_settings::mercx_settings_map;
use crate::token::handlers::exists_by_canister_id;
use crate::StablePool;
use icrc_ledger_types::icrc1::account::Account;
use crate::transfers::tx_id::TxId;
use crate::StableToken;
use crate::transfers::stable_transfer::{StableTransfer,TransferType};
use crate::ic::verify_transfer::verify_transfer;
use crate::transfers::handlers::{insert,exist};
use crate::kyc::kyc_id::get_user_by_caller;
 use crate::stable_lp_token::lp_token_map;
 use crate::StableLPToken;
 use crate::lp_metadata::stable_lp_metadata::LP_DECIMALS;
 use crate::pool_analytics::analytics_storage::record_pool_snapshot2;
#[ic_cdk::update]
pub async fn add_liquidity_transfer_from(
    args: AddLiquidityArgs,
) -> Result<AddLiquidityReply, String> {
    let (user_id, pool, add_amount_0, add_amount_1,token_0, tx_id_0, token_1, tx_id_1) = check_arguments(&args).await?;
    let ts = get_time();

    let result = match process_add_liquidity(user_id,&pool, &add_amount_0, &add_amount_1,&token_0, tx_id_0.as_ref(), &token_1 ,tx_id_1.as_ref(),ts).await {
        Ok(reply) => Ok({
            let _ = record_pool_snapshot2(reply.pool_id).await;
            reply}),
        Err(e) => Err(e),
    };

    result
}

async fn process_add_liquidity(
     user_id: u32,
    pool: &StablePool,
    add_amount_0: &Nat,
    add_amount_1: &Nat,
    token_0: &StableToken,
    tx_id_0: Option<&Nat>,
    token_1: &StableToken,
    tx_id_1: Option<&Nat>,
    ts: u64,
) -> Result<AddLiquidityReply, String> {
    // Token0
   // let token_0 = pool.token_0();
    // Token1
    //let token_1 = pool.token_1();

    let caller_id = caller_id();
    let mercx_backend = mercx_settings_map::get().mercx_backend;
    let mut transfer_ids = Vec::new();

    let transfer_0 = match tx_id_0 {
        Some(block_id) => {
            verify_transfer_token(token_0, block_id, add_amount_0, &mut transfer_ids, ts).await
        }
        None => {
            transfer_from_token(
                &caller_id,
                token_0,
                add_amount_0,
                &mercx_backend,
                &mut transfer_ids,
                ts,
            )
            .await
           
        }
         
    };

    let transfer_1 = match tx_id_1 {
        Some(block_id) => {
            verify_transfer_token(token_1, block_id, add_amount_1, &mut transfer_ids, ts).await
        }
        None => {
            //  if transfer_token_0 failed, no need to icrc2_transfer_from token_1
            if transfer_0.is_err() {
                Err("Token_0 transfer failed".to_string())
            } else {
                transfer_from_token(
                    &caller_id,
                    token_1,
                    add_amount_1,
                    &mercx_backend,
                    &mut transfer_ids,
                    ts,
                )
                .await
            }
        }
    };

     // both transfers must be successful
     if transfer_0.is_err() || transfer_1.is_err() {
        return_tokens(
            &caller_id,
    pool,
            add_amount_0,
            add_amount_1,
            &mut transfer_ids,
            ts,
            &transfer_0,
            &transfer_1,    
        )
        .await;
        if transfer_0.is_err() {
            return Err(format!("failed. {}", transfer_0.unwrap_err()));
        } else {
            return Err(format!("failed. {}", transfer_1.unwrap_err()));
        };
    }


    // re-calculate with latest pool state and make sure amounts are valid
    let (pool, _, _,add_lp_token_amount) = match update_liquidity_pool(user_id,pool, add_amount_0, add_amount_1 ,ts).await {
        Ok((pool, amount_0, amount_1,add_lp_token_amount)) => (pool, amount_0, amount_1,add_lp_token_amount),
        Err(err) => {

             //"Token with symbol 'by6od-j4aaa-aaaaa-qaadq-cai' not found"
            ic_cdk::println!("❌ update_liquidity_pool failed: {:?}", err);
            // LP amounts are incorrect. return token_0 and token_1 back to user
            return_tokens(
                &caller_id,
                pool,
                add_amount_0,
                add_amount_1,
                &mut transfer_ids,
                ts,
                &transfer_0,
                &transfer_1,  
            )
           .await;
            return Ok(to_add_liquidity_reply_failed(       
                pool,
                &token_0.canister_id().expect("Missing canister_id").to_string(),
           &token_0.symbol(),
           &token_1.canister_id().expect("Missing canister_id").to_string(),
           &token_1.symbol(),
           &transfer_ids));
        }
    };

    Ok(to_add_liquidity_reply(
        &pool,
        &token_0,
        &token_1,
       add_lp_token_amount,
        &transfer_ids,
    ))
}


async fn check_arguments(args: &AddLiquidityArgs) -> Result<(u32,StablePool, Nat, Nat,StableToken, Option<Nat>,StableToken, Option<Nat>), String> {

    if nat_is_zero(&args.amount_0) || nat_is_zero(&args.amount_1) {
        Err("Invalid zero amounts".to_string())?
    }

    // check to make sure tx_id_0 and tx_id_1 is not specified
    if args.tx_id_0.is_some() || args.tx_id_1.is_some() {
        Err("Tx_id_0 and Tx_id_1 not supported".to_string())?
    }

    // add_amount_0 and add_amount_1 are the amounts to be added to the pool with the current state
    // these are the amounts that will be transferred to the pool
    let (pool, add_amount_0, add_amount_1, _) =
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

    //new
    // check tx_id_0 is valid block index Nat
    let tx_id_0 = match &args.tx_id_0 {
        Some(TxId::BlockIndex(tx_id)) => Some(tx_id.clone()),
        _ => None,
    };

    let tx_id_1 = match &args.tx_id_1 {
        Some(TxId::BlockIndex(tx_id)) => Some(tx_id.clone()),
        _ => None,
    };

    // either tx_id_0 or tx_id_1 must be valid
    // if tx_id_0.is_none() && tx_id_1.is_none() {
    //     Err("Tx_id_0 or Tx_id_1 is required".to_string())?
    // }



    // make sure user is registered, if not create a new user
    // let user_id = user_map::insert(None)?;
 let user_id = get_user_by_caller()
        .await
        .map_err(|e| format!("KYC lookup failed: {}", e))?
        .ok_or("User not found. Please sign up in KYC first.")?
        .user_id;
    Ok((user_id , pool, add_amount_0, add_amount_1,token_0, tx_id_0, token_1, tx_id_1))
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
) -> Result<(StablePool, Nat, Nat ,Nat), String> {
    // Pool - make sure pool exists, refresh balances of the pool to make sure we have the latest state
    let pool = handlers::get_by_tokens(token_0.to_string(), token_1.to_string())?;
    // Token0
    let token_0 = pool.token_0();
    // reserve_0 is the total balance of token_0 in the pool = balance_0 + lp_fee_0
    let reserve_0 = nat_add(&pool.balance_0, &pool.lp_fee_0);
    // Token1
    let token_1 = pool.token_1();
    let reserve_1 = nat_add(&pool.balance_1, &pool.lp_fee_1);
    // LP token
    let lp_token = pool.lp_token();
    let lp_token_id = lp_token.token_id();
    let lp_total_supply = lp_token_map::get_total_supply(lp_token_id);

    if nat_is_zero(&reserve_0) || nat_is_zero(&reserve_1) {
        // new pool as there are no balances - take user amounts as initial ratio
        // initialize LP tokens as sqrt(amount_0 * amount_1)
        // convert the amounts to the same decimal precision as the LP token
        let amount_0_in_lp_token_decimals = nat_to_decimal_precision(amount_0, token_0.decimals(), LP_DECIMALS);
        let amount_1_in_lp_token_decimals = nat_to_decimal_precision(amount_1, token_1.decimals(), LP_DECIMALS);
        let add_lp_token_amount = nat_sqrt(&nat_multiply(&amount_0_in_lp_token_decimals, &amount_1_in_lp_token_decimals));
        return Ok((pool, amount_0.clone(), amount_1.clone(),add_lp_token_amount));
    }

    // amount_0 * reserve_1 = amount_1 * reserve_0 for constant K
    let amount_0_reserve_1 = nat_multiply(amount_0, &reserve_1);
    let amount_1_reserve_0 = nat_multiply(amount_1, &reserve_0);
    // if the ratio of the user amounts is the same as the pool ratio, then the amounts are correct
    // rarely happens as there are rounding precision errors
    if amount_0_reserve_1 == amount_1_reserve_0 {
        // calculate the LP token amount for the user
       // add_lp_token_amount = lp_total_supply * amount_0 / reserve_0
        let amount_0_in_lp_token_decimals = nat_to_decimal_precision(amount_0, token_0.decimals(), LP_DECIMALS);
        let reserve_0_in_lp_token_decimals = nat_to_decimal_precision(&reserve_0, token_0.decimals(), LP_DECIMALS);
        let numerator_in_lp_token_decimals = nat_multiply(&lp_total_supply, &amount_0_in_lp_token_decimals);
        let add_lp_token_amount =
            nat_divide(&numerator_in_lp_token_decimals, &reserve_0_in_lp_token_decimals).ok_or("Invalid LP token amount")?;
        return Ok((pool, amount_0.clone(), amount_1.clone(),add_lp_token_amount));
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
        let amount_0_in_lp_token_decimals = nat_to_decimal_precision(amount_0, token_0.decimals(), LP_DECIMALS);
        let reserve_0_in_lp_token_decimals = nat_to_decimal_precision(&reserve_0, token_0.decimals(), LP_DECIMALS);
        let numerator_in_lp_token_decimals = nat_multiply(&lp_total_supply, &amount_0_in_lp_token_decimals);
        let add_lp_token_amount =
            nat_divide(&numerator_in_lp_token_decimals, &reserve_0_in_lp_token_decimals).ok_or("Invalid LP token amount")?;
        return Ok((pool, amount_0.clone(), amount_1_in_token_1_decimals, add_lp_token_amount));
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
        let amount_1_in_lp_token_decimals = nat_to_decimal_precision(amount_1, token_1.decimals(), LP_DECIMALS);
        let reserve_1_in_lp_token_decimals = nat_to_decimal_precision(&reserve_1, token_1.decimals(), LP_DECIMALS);
        let numerator_in_lp_token_decimals = nat_multiply(&lp_total_supply, &amount_1_in_lp_token_decimals);
        let add_lp_token_amount =
            nat_divide(&numerator_in_lp_token_decimals, &reserve_1_in_lp_token_decimals).ok_or("Invalid LP token amount")?;
        return Ok((pool, amount_0_in_token_0_decimals, amount_1.clone(),add_lp_token_amount));
    }

    // pool ratio must have changed from initial calculation and amount_0 and amount_1 are not enough now
    Err("Incorrect ratio of amount_0 and amount_1".to_string())
}

/// update the liquidity pool with the new liquidity amounts
/// ensure we have the latest state of the pool before adding the new amounts
/// //update balance
pub async fn update_liquidity_pool(
      user_id: u32,
    pool: &StablePool,
    add_amount_0: &Nat,
    add_amount_1: &Nat,
        ts: u64,
) -> Result<(StablePool, Nat, Nat ,Nat), String> {
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
        Ok((mut pool, amount_0, amount_1,add_lp_token_amount)) => {
            pool.balance_0 = nat_add(&pool.balance_0, &amount_0);
            pool.balance_1 = nat_add(&pool.balance_1, &amount_1);
            handlers::update(&pool);

            // update user's LP token amount
            update_lp_token(user_id, pool.lp_token_id, &add_lp_token_amount, ts).await;

            Ok((pool, amount_0, amount_1,add_lp_token_amount))
        }
        Err(e) => Err(e),
    }
}
/// update the user's LP token amount
/// ensure we have the latest state of the LP token before adding the new amounts
async fn update_lp_token(user_id: u32, lp_token_id: u32, add_lp_token_amount: &Nat, ts: u64) {
    // if you want the current caller’s principal here
    let principal: Principal = ic_cdk::api::caller();
    ic_cdk::println!("principal {}",principal);
    // refresh with the latest state if the entry exists
    match lp_token_map::get_by_token_id(lp_token_id).await {
        Some(lp_token) => {
            // update adding the new deposit amount
            let new_user_lp_token = StableLPToken {
                amount: nat_add(&lp_token.amount, add_lp_token_amount),
                ts,
                ..lp_token.clone()
            };
            lp_token_map::update(&new_user_lp_token);
          
        }
        None => {
            // new entry
            let new_user_lp_token = StableLPToken::new(user_id,principal, lp_token_id, add_lp_token_amount.clone(), ts);
            match lp_token_map::insert(&new_user_lp_token) {
                 Ok(_) => {
                    ic_cdk::println!("✅ LP token inserted successfully");
                }
              Err(e) => {
                    ic_cdk::println!("❌ Failed to insert LP token: {}", e);
                }
            };
        }
    }
}

async fn return_tokens(
    to_principal_id: &Account,
    pool: &StablePool,
    amount_0: &Nat,
    amount_1: &Nat,
    transfer_ids: &mut Vec<u64>,
    ts: u64,
    transfer_0: &Result<(), String>,
    transfer_1: &Result<(), String>,

) {
    //  let mut claim_ids = Vec::new();
    if transfer_0.is_ok() {
        let token_0 = pool.token_0();
        return_token(to_principal_id, &token_0, amount_0, transfer_ids, ts).await;
    }

    if transfer_1.is_ok(){
        let token_1 = pool.token_1();
        return_token(to_principal_id, &token_1, amount_1, transfer_ids, ts).await;
    }

    //let reply = to_add_liquidity_reply_failed(pool.pool_id, request_id, transfer_ids, ts);
}

async fn verify_transfer_token(
    token: &StableToken,
    tx_id: &Nat,
    amount: &Nat,
    transfer_ids: &mut Vec<u64>,
    ts: u64,
) -> Result<(), String> {
    let token_id: u32 = token.token_id();

   
    match verify_transfer(token, tx_id, amount).await {
        Ok(_) => {
            // contain() will use the latest state of TRANSFER_MAP to prevent reentrancy issues after verify_transfer()
            if exist(token_id, tx_id) {
                let e = format!("Duplicate block id #{}", tx_id);
           
                return Err(e);
            }
            let transfer_id = insert(&StableTransfer {
                transfer_id: 0,
                is_send: true,
                amount: amount.clone(),
                token_id,
                tx_id: TxId::BlockIndex(tx_id.clone()),
                transfer_type: TransferType::LiquidityAdd,      
                ts,
            });
            transfer_ids.push(transfer_id);
            Ok(())
        }
        Err(e) => {
         
            Err(e)
        }
    }
}
