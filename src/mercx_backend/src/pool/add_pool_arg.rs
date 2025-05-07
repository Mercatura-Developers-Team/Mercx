use candid::{CandidType, Nat,Principal};
use serde::{Deserialize, Serialize};
use crate::StablePool;
use crate::pool::handlers;
use crate::StableToken;
use ic_cdk::api;
use crate::stable_mercx_settings::mercx_settings_map;
use crate::token::handlers::get_by_token;
use crate::token::add_token::add_token;
use crate::helpers::math_helpers::{nat_add,nat_is_zero};

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct AddPoolArgs {
    pub token_0: String,             // e.g. FXMX
    pub amount_0: Nat,               // amount to deposit of token 0
    pub token_1: String,             // e.g. ckUSDT
    pub amount_1: Nat,               // amount to deposit of token 1
  pub lp_fee_bps: Option<u8>,      // optional fee in basis points, default = 30 //for each swap
}

//for frontend API
#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct AddPoolReply {
    pub pool_id: u32,                // unique pool identifier
    pub symbol: String,              // FXMX_ckUSDT
    pub name: String,                // FXMX_ckUSDT Liquidity Pool
    pub symbol_0: String,            // FXMX
    pub address_0: String,           // token 0 address
    pub amount_0: Nat,               // deposited
    pub symbol_1: String,            // ckUSDT
    pub address_1: String,           // token 1 address
    pub amount_1: Nat,               // deposited
    pub lp_fee_bps: u8,              // confirmed LP fee
    pub lp_token_symbol: String,     // FXMX_ckUSDT_LP
    pub lp_token_amount: Nat,        // amount of LP tokens minted
    pub ts: u64,                     // timestamp of creation
}
#[ic_cdk::update]
// add_pool() taken
 fn add_new_pool(token_id_0: u32, token_id_1: u32, lp_fee_bps: u8, kong_fee_bps: u8, lp_token_id: u32) -> Result<StablePool, String> {
    let pool = StablePool::new(token_id_0, token_id_1, lp_fee_bps, kong_fee_bps, lp_token_id);
    let pool_id = handlers::insert(&pool)?;
    // Retrieves the inserted pool by its pool_id
    handlers::get_by_pool_id(pool_id).ok_or_else(|| "Failed to add pool".to_string())
}

async fn process_add_pool(
    token_0: &StableToken,
    amount_0: &Nat,
    token_1: &StableToken,
    amount_1: &Nat,
    lp_fee_bps: u8,
    kong_fee_bps: u8,
) -> Result<AddPoolReply, String> {
    // let caller_id = caller_id();  // Uncomment if you need caller_id later

    let pool = match add_new_pool(
        token_0.token_id(),
        token_1.token_id(),
        lp_fee_bps,
        kong_fee_bps,
        0, // ← TEMP: You must pass a valid `lp_token_id` here!
    ) {
        Ok(pool) => pool,
        Err(e) => return Err(format!("Pool creation failed: {}", e)),
    };

        // update pool with new balances
        update_liquidity_pool( &pool, amount_0, amount_1);

    // TODO: Return actual AddPoolReply here, depending on your logic
    Ok(AddPoolReply {
        // Fill this based on your actual struct
        pool_id: pool.pool_id,
        symbol: format!("{}_{}", token_0.symbol(), token_1.symbol()),
        name: format!("{}_{} Liquidity Pool", token_0.symbol(), token_1.symbol()),
        symbol_0: token_0.symbol(),
        address_0: pool.canister_id_0(),
        amount_0: amount_0.clone(),
        symbol_1: token_1.symbol(),
        address_1: pool.canister_id_1(),
        amount_1: amount_1.clone(),
        lp_fee_bps,
        lp_token_symbol: format!("{}_{}_LP", token_0.symbol(), token_1.symbol()),
        lp_token_amount: Nat::from(0_u32),  // Replace with actual LP token mint amount later
        ts: api::time() / 1_000_000_000,                      // Use your imported `get_time()` helper
    })
    
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
) -> Result<( StableToken, Nat, StableToken, Nat, u8, u8), String> {
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
        Err(format!("LP fee cannot be less than Mercx fee of {}", mercx_fee_bps))?
    }

    // // check tx_id_0 and tx_id_1 are valid block index Nat
    // let tx_id_0 = match &args.tx_id_0 {
    //     Some(tx_id_0) => match tx_id_0 {
    //         TxId::BlockIndex(block_id) => Some(block_id).cloned(),
    //         _ => Err("Unsupported tx_id_0".to_string())?,
    //     },
    //     None => None,
    // };
    // let tx_id_1 = match &args.tx_id_1 {
    //     Some(tx_id_1) => match tx_id_1 {
    //         TxId::BlockIndex(block_id) => Some(block_id).cloned(),
    //         _ => Err("Unsupported tx_id_1".to_string())?,
    //     },
    //     None => None,
    // };

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
        Err(format!("Pool {} already exists", handlers::symbol(&token_0, &token_1)))?
    }

  //  let (add_amount_0, add_amount_1, add_lp_token_amount) = calculate_amounts(&token_0, &args.amount_0, &token_1, &args.amount_1)?;

    // make sure user is registered, if not create a new user
    //let user_id = user_map::insert(None)?;

    Ok((
        // user_id,
        token_0,
        args.amount_0.clone(),
        // tx_id_0,
        token_1,
        args.amount_1.clone(),
        // tx_id_1,
        lp_fee_bps,
        mercx_fee_bps,
        // add_lp_token_amount,
    ))
}

#[ic_cdk::update]
pub async fn add_pool(args: AddPoolArgs) -> Result<AddPoolReply, String> {
    let ( token_0, add_amount_0, token_1, add_amount_1, lp_fee_bps, kong_fee_bps) =
        check_arguments(&args).await?;
   // let ts = get_time();
  //  let request_id = request_map::insert(&StableRequest::new(user_id, &Request::AddPool(args), ts));

    let result = match process_add_pool(
        &token_0,
        &add_amount_0,
        &token_1,
        &add_amount_1,
        lp_fee_bps,
        kong_fee_bps,
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