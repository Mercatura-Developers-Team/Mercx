use crate::helpers::math_helpers::{nat_add, nat_is_zero, nat_subtract, nat_zero};
use crate::ic::id::caller_id;
use crate::ic::transfer::icrc1_transfer;
use crate::ic::transfer::icrc2_transfer_from;
use crate::ic::verify_transfer::verify_transfer;
use crate::pool::add_pool_reply::{to_add_pool_reply_failed, AddPoolReply,to_add_pool_reply};
//use crate::transfers::transfer_reply_helpers::to_transfer_ids;
use crate::pool::handlers;
use crate::stable_mercx_settings::mercx_settings_map;
use crate::token::add_token::add_token;
use crate::token::handlers::get_by_token;
use crate::transfers::handlers as transfer_handlers;
use crate::transfers::stable_transfer::StableTransfer;
use crate::transfers::tx_id::TxId;
use crate::StablePool;
use crate::StableToken;
use candid::{CandidType, Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct AddPoolArgs {
    pub token_0: String, // e.g. FXMX
    pub amount_0: Nat,   // amount to deposit of token 0
    pub tx_id_0: Option<TxId>,
    pub token_1: String, // e.g. ckUSDT
    pub amount_1: Nat,   // amount to deposit of token 1
    pub tx_id_1: Option<TxId>,
    pub lp_fee_bps: Option<u8>, // optional fee in basis points, default = 30 //for each swap
}

fn add_new_pool(
    token_id_0: u32,
    token_id_1: u32,
    lp_fee_bps: u8,
    mercx_fee_bps: u8,
    lp_token_id: u32,
) -> Result<StablePool, String> {
    let pool = StablePool::new(
        token_id_0,
        token_id_1,
        lp_fee_bps,
        mercx_fee_bps,
        lp_token_id,
    );
    let pool_id = handlers::insert(&pool)?;
    // Retrieves the inserted pool by its pool_id
    handlers::get_by_pool_id(pool_id).ok_or_else(|| "Failed to add pool".to_string())
}

async fn process_add_pool(
    token_0: &StableToken,
    amount_0: &Nat,
    tx_id_0: Option<&Nat>,
    token_1: &StableToken,
    amount_1: &Nat,
    tx_id_1: Option<&Nat>,
    lp_fee_bps: u8,
    mercx_fee_bps: u8,
    ts: u64,
) -> Result<AddPoolReply, String> {
    let caller_id = caller_id(); // Uncomment if you need caller_id later
    let mercx_backed = mercx_settings_map::get().mercx_backend;
    let mut transfer_ids = Vec::new();
    let transfer_0 = match tx_id_0 {
        Some(block_id) => {
            verify_transfer_token(token_0, block_id, amount_0, &mut transfer_ids, ts).await
        }
        None => {
            transfer_from_token(
                &caller_id,
                token_0,
                amount_0,
                &mercx_backed,
                &mut transfer_ids,
                ts,
            )
            .await
           
        }
         
    };

    let transfer_1 = match tx_id_1 {
        Some(block_id) => {
            verify_transfer_token(token_1, block_id, amount_1, &mut transfer_ids, ts).await
        }
        None => {
            //  if transfer_token_0 failed, no need to icrc2_transfer_from token_1
            if transfer_0.is_err() {
                Err("Token_0 transfer failed".to_string())
            } else {
                transfer_from_token(
                    &caller_id,
                    token_1,
                    amount_1,
                    &mercx_backed,
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
            &transfer_0,
            token_0,
            amount_0,
            &transfer_1,
            token_1,
            amount_1,
            &mut transfer_ids,
            ts,
        )
        .await;
        if transfer_0.is_err() {
            return Err(format!("failed. {}", transfer_0.unwrap_err()));
        } else {
            return Err(format!("failed. {}", transfer_1.unwrap_err()));
        };
    }

    //add pool
    let pool = match add_new_pool(
        token_0.token_id(),
        token_1.token_id(),
        lp_fee_bps,
        mercx_fee_bps,
        0, // ← TEMP: You must pass a valid `lp_token_id` here!
    ) {
        Ok(pool) => pool,
        Err(e) => {
            return_tokens(
                &caller_id,
                &transfer_0,
                token_0,
                amount_0,
                &transfer_1,
                token_1,
                amount_1,
                &mut transfer_ids,
                ts,
            )
            .await;
        return Ok(to_add_pool_reply_failed(       
             &token_0.canister_id().expect("Missing canister_id").to_string(),
        &token_0.symbol(),
        &token_1.canister_id().expect("Missing canister_id").to_string(),
        &token_1.symbol(),
        &transfer_ids));
            },
    };

    // update pool with new balances
    update_liquidity_pool(&pool, amount_0, amount_1);

    // TODO: Return actual AddPoolReply here, depending on your logic
    Ok(to_add_pool_reply(   &pool, token_0, token_1))
}

//update balance
fn update_liquidity_pool(
    // user_id: u32,
    pool: &StablePool,
    amount_0: &Nat,
    amount_1: &Nat,
    // add_lp_token_amount: &Nat,
    // ts: u64,
) {
    let update_pool = StablePool {
        balance_0: nat_add(&pool.balance_0, amount_0),
        balance_1: nat_add(&pool.balance_1, amount_1),
        ..pool.clone()
    };
    handlers::update(&update_pool);

    // update user's LP token amount
    // update_lp_token(request_id, user_id, pool.lp_token_id, add_lp_token_amount, ts);
}

async fn check_arguments(
    args: &AddPoolArgs,
) -> Result<
    (
        StableToken,
        Nat,
        Option<Nat>,
        StableToken,
        Nat,
        Option<Nat>,
        u8,
        u8,
    ),
    String,
> {
    if nat_is_zero(&args.amount_0) || nat_is_zero(&args.amount_1) {
        Err("Invalid zero amounts".to_string())?
    }

    let lp_fee_bps = match args.lp_fee_bps {
        Some(lp_fee_bps) => lp_fee_bps,
        None => mercx_settings_map::get().default_lp_fee_bps,
    };

    let default_mercx_fee_bps = mercx_settings_map::get().default_mercx_fee_bps;
    let mercx_fee_bps = default_mercx_fee_bps;
    if lp_fee_bps < mercx_fee_bps {
        Err(format!(
            "LP fee cannot be less than Mercx fee of {}",
            mercx_fee_bps
        ))?
    }

    // check tx_id_0 and tx_id_1 are valid block index Nat
    let tx_id_0 = match &args.tx_id_0 {
        Some(tx_id_0) => match tx_id_0 {
            TxId::BlockIndex(block_id) => Some(block_id).cloned(),
            _ => Err("Unsupported tx_id_0".to_string())?,
        },
        None => None,
    };
    let tx_id_1 = match &args.tx_id_1 {
        Some(tx_id_1) => match tx_id_1 {
            TxId::BlockIndex(block_id) => Some(block_id).cloned(),
            _ => Err("Unsupported tx_id_1".to_string())?,
        },
        None => None,
    };

    // // make sure token_1 is ckUSDT or ICP
    // let token_1 = match args.token_1.as_str() {
    //     token if is_ckusdt(token) => token_map::get_ckusdt()?,
    //     token if is_icp(token) => token_map::get_icp()?,
    //     _ => Err(format!(
    //         "Token_1 must be {} or {}",
    //         kong_settings_map::get().ckusdt_symbol,
    //         kong_settings_map::get().icp_symbol
    //     ))?,
    // };

    // token_0, check if it exists already or needs to be added
    // leave token_0 check latest as possible as token will be added to the system
    let token_0 = match get_by_token(&args.token_0) {
        Ok(token) => token, // token_0 exists already
        Err(_) => {
            let principal: Principal = Principal::from_text(&args.token_0)
                .map_err(|e| format!("Invalid canister id '{}': {}", args.token_0, e))?;
            add_token(principal).await?
        }
    };

    let token_1 = match get_by_token(&args.token_1) {
        Ok(token) => token, // token_0 exists already
        Err(_) => {
            let principal: Principal = Principal::from_text(&args.token_1)
                .map_err(|e| format!("Invalid canister id '{}': {}", args.token_1, e))?;
            add_token(principal).await?
        }
    };

    // make sure LP token does not already exist
    // let lp_token_address = token::address(&token_0, &token_1);
    // if token_map::exists(&lp_token_address) {
    //     Err(format!("LP token {} already exists", token::symbol(&token_0, &token_1)))?
    // }

    // make sure pool does not already exist
    if handlers::exists(&token_0, &token_1) {
        Err(format!(
            "Pool {} already exists",
            handlers::symbol(&token_0, &token_1)
        ))?
    }

    //  let (add_amount_0, add_amount_1, add_lp_token_amount) = calculate_amounts(&token_0, &args.amount_0, &token_1, &args.amount_1)?;

    // make sure user is registered, if not create a new user
    //let user_id = user_map::insert(None)?;

    Ok((
        //  user_id,
        token_0,
        args.amount_0.clone(),
        tx_id_0,
        token_1,
        args.amount_1.clone(),
        tx_id_1,
        lp_fee_bps,
        mercx_fee_bps,
        // add_lp_token_amount,
    ))
}

#[ic_cdk::update]
pub async fn add_pool(args: AddPoolArgs) -> Result<AddPoolReply, String> {
    let (token_0, add_amount_0, tx_id_0, token_1, add_amount_1, tx_id_1, lp_fee_bps, kong_fee_bps) =
        check_arguments(&args).await?;
    let ts = ic_cdk::api::time();
    //  let request_id = request_map::insert(&StableRequest::new(user_id, &Request::AddPool(args), ts));

    let result = match process_add_pool(
        &token_0,
        &add_amount_0,
        tx_id_0.as_ref(),
        &token_1,
        &add_amount_1,
        tx_id_1.as_ref(),
        lp_fee_bps,
        kong_fee_bps,
        ts,
    )
    .await
    {
        Ok(reply) => Ok(reply),
        Err(e) => Err(e),
    };

    result
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
        Ok(block_id) => {
            // insert_transfer() will use the latest state of TRANSFER_MAP so no reentrancy issues after icrc2_transfer_from()
            // as icrc2_transfer_from() does a new transfer so block_id should be new
            let transfer_id = transfer_handlers::insert(&StableTransfer {
                transfer_id: 0,
                is_send: true,
                amount: amount.clone(),
                token_id,
                tx_id: TxId::BlockIndex(block_id),
                ts,
            });
            transfer_ids.push(transfer_id);
ic_cdk::println!("💬 Transfer_0 result: {:?}", transfer_ids);

            Ok(())
        }
        Err(e) => Err(e),
    }
}

//This function is used after a user has manually sent tokens, and you're verifying their claim.
async fn verify_transfer_token(
    token: &StableToken,
    tx_id: &Nat,
    amount: &Nat,
    transfer_ids: &mut Vec<u64>,
    ts: u64,
) -> Result<(), String> {
    let token_id = token.token_id();

    match verify_transfer(token, tx_id, amount).await {
        Ok(_) => {
            // insert_transfer() will use the latest state of TRANSFER_MAP so no reentrancy issues after verify_transfer()
            if transfer_handlers::exist(token_id, tx_id) {
                let e = format!("Duplicate block id: #{}", tx_id);
                return Err(e);
            }
            let transfer_id = transfer_handlers::insert(&StableTransfer {
                transfer_id: 0,
                is_send: true,
                amount: amount.clone(),
                token_id,
                tx_id: TxId::BlockIndex(tx_id.clone()),
                ts,
            });
            transfer_ids.push(transfer_id);

            Ok(())
        }
        Err(e) => Err(e),
    }
}

async fn return_tokens(
    to_principal_id: &Account,
    transfer_from_token_0: &Result<(), String>,
    token_0: &StableToken,
    amount_0: &Nat,
    transfer_from_token_1: &Result<(), String>,
    token_1: &StableToken,
    amount_1: &Nat,
    transfer_ids: &mut Vec<u64>,
    ts: u64,
) {
    if transfer_from_token_0.is_ok() {
        return_token(to_principal_id, token_0, amount_0, transfer_ids, ts).await;
    }

    if transfer_from_token_1.is_ok() {
        return_token(to_principal_id, token_1, amount_1, transfer_ids, ts).await;
    }

    // let reply = to_add_pool_reply_failed(
    //     &token_0.canister_id().expect("Missing canister_id").to_string(),
    //     &token_0.symbol(),
    //     &token_1.canister_id().expect("Missing canister_id").to_string(),
    //     &token_1.symbol(),
    //     transfer_ids,
    //     ts,
    // );
}

async fn return_token(
    to_principal_id: &Account,
    token: &StableToken,
    amount: &Nat,
    transfer_ids: &mut Vec<u64>,
    //claim
    ts: u64,
) {
    let amount_0_with_gas = nat_subtract(amount, &token.fee()).unwrap_or(nat_zero());
    match icrc1_transfer(&amount_0_with_gas, to_principal_id, token, None).await {
        Ok(block_id) => {
            let transfer_id = transfer_handlers::insert(&StableTransfer {
                transfer_id: 0,
                is_send: false,
                amount: amount_0_with_gas,
                token_id: token.token_id(),
                tx_id: TxId::BlockIndex(block_id),
                ts,
            });
            transfer_ids.push(transfer_id);
        }
        Err(_) => {
            // claim

            //  let err_msg = format!(" Failed to return tokens back to user: {}", e);
           //   Err("");
        }
    }
}
