use crate::pool::stable_pool::{StablePool,StablePoolId};
use crate::stable_memory::POOLS;
use crate::pool::add_pool_reply::AddPoolReply;
use crate::StableToken;
use crate::token::handlers;
use crate::stable_mercx_settings;
use crate::token::handlers::get_by_token;
use crate::transfers::handlers as transfer_handlers;
use crate::pool::add_pool_reply::to_add_pool_reply;
use crate::token::handlers::get_by_token_id;
use crate::stable_mercx_settings::mercx_settings_map::reset_pool_map_idx;
use crate::helpers::math_helpers::nat_zero;
use candid::Nat;
use candid::Principal;
use crate::stable_lp_token::lp_token_map::{get_by_token_id_by_principal};
use crate::xrc_mock::get_rate_vs_usd;
use std::borrow::Cow;
pub fn symbol(token_0: &StableToken, token_1: &StableToken) -> String {
    format!("{}_{}", token_0.symbol(), token_1.symbol())
}

pub fn get_by_pool_id(pool_id: u32) -> Option<StablePool> {
    POOLS.with(|m| {
        m.borrow().get(&StablePoolId(pool_id))
    })
}

pub fn get_by_symbol(symbol: &str) -> Result<StablePool, String> {
    POOLS.with(|pools| {
        pools
            .borrow()
            .iter()
            .find_map(|(_, pool)| {
                if pool.name() == symbol {
                    Some(pool.clone())
                } else {
                    None
                }
            })
    })
    .ok_or_else(|| format!("Pool with symbol '{}' not found", symbol))
}

pub fn get_by_token_ids(token_id_0: u32, token_id_1: u32) -> Option<StablePool> {
    POOLS.with(|m| {
        m.borrow().iter().find_map(|(_, v)| {
            if v.token_id_0 == token_id_0 && v.token_id_1 == token_id_1 {
                return Some(v);
            }
            
            None
        })
    })
}

#[ic_cdk::query]
pub fn get_by_tokens(token_0:String, token_1: String) -> Result<StablePool, String> {
    let token_0: StableToken = handlers::get_by_token(&token_0)?;
    let token_1 = handlers::get_by_token(&token_1)?;
 

         get_by_token_ids(token_0.token_id(), token_1.token_id()).ok_or_else(|| format!("Pool {} not found", symbol(&token_0, &token_1)))

}



pub fn exists(token_0: &StableToken, token_1: &StableToken) -> bool {
    POOLS.with(|m| {
        m.borrow().iter().any(|(_, v)| {
            v.token_id_0 == token_0.token_id() && v.token_id_1 == token_1.token_id()
                || v.token_id_0 == token_1.token_id() && v.token_id_1 == token_0.token_id()
        })
    })
}

pub fn insert(pool: &StablePool) -> Result<u32, String> {
    if exists(&pool.token_0(), &pool.token_1()) {
        Err(format!("Pool {} already exists", pool.name()))?
    }

    let insert_pool = POOLS.with(|m| {
        let mut map = m.borrow_mut();
        let pool_id = stable_mercx_settings::mercx_settings_map::inc_pool_map_idx();
        let insert_pool = StablePool { pool_id, ..pool.clone() };
        map.insert(StablePoolId(pool_id), insert_pool.clone());
        insert_pool
    });
    Ok(insert_pool.pool_id)
}



// (optional) tiny helper: LP amount for this pool & principal
fn lp_amount_for_pool_and_principal(pool: &StablePool, principal: Principal) -> Nat {
    get_by_token_id_by_principal(pool.lp_token_id, principal)
        .map(|row| row.amount)
        .unwrap_or_else(nat_zero)
}

#[ic_cdk::query] // update, not query
pub async fn get_all_pools() -> Result<Vec<AddPoolReply>, String> {
    
    let caller_principal = ic_cdk::api::caller();


    let list = POOLS.with(|pools| {
        pools.borrow().iter().map(|(_, pool)| {
            // let transfer_ids =
            //     transfer_handlers::get_by_token_ids(pool.token_id_0, pool.token_id_1);

            let token0 = get_by_token_id(pool.token_id_0).unwrap();
            let token1 = get_by_token_id(pool.token_id_1).unwrap();

            // If you really want the caller’s LP amount here, compute it. Otherwise use nat_zero().
         //   let _lp_amount = get_user_lp_amount_for_pool(&pool, user_id);
         // actual LP amount of the caller for this pool
         let lp_amount = lp_amount_for_pool_and_principal(&pool, caller_principal);

            to_add_pool_reply(&pool, &token0, &token1, lp_amount, &[])
        }).collect::<Vec<_>>()
    });

    Ok(list)
}


#[ic_cdk::query]
fn get_pool_price(token_0: String, token_1: String) -> Result<f64, String> {
    if let Ok(pool) = get_by_tokens(token_0.clone(), token_1.clone())
        .or_else(|_| get_by_tokens(token_1, token_0))
    {
        pool.get_price_as_f64().ok_or("Price unavailable".to_string())
    } else {
        Err("Pool not found".to_string())
    }
}


#[ic_cdk::update]
fn delete_pool(pool_id: u32) -> Result<String, String> {
    POOLS.with(|pools| {
        let mut pools = pools.borrow_mut();

        if pools.remove(&StablePoolId(pool_id)).is_some() {
            Ok(format!("Pool with id {} has been permanently deleted.", pool_id))
        } else {
            Err(format!("Pool with id {} not found.", pool_id))
        }
    })
}

//cargo build --release --features prod
#[cfg(not(feature = "prod"))]
#[ic_cdk::update]
fn reset_pools() -> Result<String, String> {
    POOLS.with(|pools| {
        pools.borrow_mut().clear_new(); // `clear_new()` btmsh kolo remove law hanmsh haga specific
    });

    reset_pool_map_idx();

    Ok("✅ Tokens memory cleared".to_string())
}

//It stores the updated version of a liquidity pool into the global state
pub fn update(pool: &StablePool) {
    POOLS.with(|m| m.borrow_mut().insert(StablePoolId(pool.pool_id), pool.clone()));
}

#[ic_cdk::query]
pub fn pool_exists(token_0: String, token_1: String) -> bool {
    let token_0 = match get_by_token(&token_0) {
        Ok(token) => token,
        Err(_) => return false,
    };

    let token_1 = match get_by_token(&token_1) {
        Ok(token) => token,
        Err(_) => return false,
    };

    exists(&token_0, &token_1)
}

/// Get pool by LP token's id.
pub fn get_by_lp_token_id(lp_token_id: u32) -> Option<StablePool> {
    POOLS.with(|m| {
        m.borrow()
            .iter()
            .find_map(|(_, v)| if v.lp_token_id == lp_token_id { Some(v) } else { None })
    })
}


// =============================
// USD pricing via pools (route ≤ 2 hops)
// =============================



// ---------- Anchors & helpers ----------
const ANCHORS: [&str; 5] = ["USDT", "USDC", "ICP", "ETH", "BTC"];

fn is_stable(sym: &str) -> bool {
    matches!(sym, "USDT" | "USDC" | "CKUSDT" | "CKUSDC")
}

fn normalize<'a>(sym: &'a str) -> Cow<'a, str> {
    let s = sym.trim().to_uppercase();
    let s = s.strip_suffix(".E").unwrap_or(&s).to_string();
    match s.as_str() {
        "CKBTC"  => Cow::from("BTC"),
        "CKETH"  => Cow::from("ETH"),
        "CKUSDT" => Cow::from("USDT"),
        "CKUSDC" => Cow::from("USDC"),
        _ => Cow::from(s),
    }
}

// POOL PRICE 
fn pool_price_a_in_b(a: &str, b: &str) -> Result<f64, String> {
    if let Ok(pool) = get_by_tokens(a.to_string(), b.to_string())
        .or_else(|_| get_by_tokens(b.to_string(), a.to_string()))
    {
        if pool.symbol_0() == a && pool.symbol_1() == b {
            pool.get_price_as_f64().ok_or("Price unavailable".to_string())
        } else if pool.symbol_0() == b && pool.symbol_1() == a {
            pool.get_price_as_f64()
                .map(|p| if p > 0.0 { 1.0 / p } else { p })
                .ok_or("Price unavailable".to_string())
        } else {
            Err("Pool symbols mismatch".to_string())
        }
    } else {
        Err("Pool not found".to_string())
    }
}


// ---------- Core: fetch USD price via pools ----------
#[ic_cdk::update] // make it query if you never await inter-canister calls inside
pub async fn get_usd_price_from_pools(raw_symbol: String) -> Result<f64, String> {
    let token = normalize(&raw_symbol);

    // 0) Trivial cases [USDT,USDC,CKUSDT,CKUSDC =1 ]
    if token.as_ref() == "USD" { return Ok(1.0); }
    if is_stable(token.as_ref()) { return Ok(1.0); }

    // 1) Direct USD-pegged pairs first: TKN/USDT or TKN/USDC (ck* covered by normalize)
    for usd_sym in ["USDT", "USDC"] {
        if let Ok(p) = pool_price_a_in_b(token.as_ref(), usd_sym) {
            // Price of 1 TKN in USDT/USDC ≈ USD
            return Ok(p);
        }
        if let Ok(p) = pool_price_a_in_b(usd_sym, token.as_ref()) {
            // If the stored mid price is 1 USD in TKN, invert
            if p > 0.0 { return Ok(1.0 / p); }
        }
    }

    // 2) Single hop via crypto anchors {ICP, ETH, BTC}
    for anchor in ["ICP", "ETH", "BTC"] {
        if let Ok(p_tkn_in_anchor) = pool_price_a_in_b(token.as_ref(), anchor) {
            // TKN/USD = (TKN/ANCHOR) * (ANCHOR/USD)
            let anchor_usd = get_rate_vs_usd(anchor.to_string()).await
                .map_err(|e| format!("Failed to get {anchor}/USD: {e}"))?;
            return Ok(p_tkn_in_anchor * anchor_usd);
        }
        if let Ok(p_anchor_in_tkn) = pool_price_a_in_b(anchor, token.as_ref()) {
            if p_anchor_in_tkn > 0.0 {
                let anchor_usd = get_rate_vs_usd(anchor.to_string()).await
                    .map_err(|e| format!("Failed to get {anchor}/USD: {e}"))?;
                // If price was ANCHOR/TKN, invert to get TKN/ANCHOR
                return Ok((1.0 / p_anchor_in_tkn) * anchor_usd);
            }
        }
    }



    // 3) Two-hop route: TKN -> REF -> USD, where REF ∈ ANCHORS.
    // Try REF in order of most reliable peg/coverage.
    for ref_sym in ANCHORS {
        // First leg: TKN <-> REF from your pools
        let tkn_in_ref = match pool_price_a_in_b(token.as_ref(), ref_sym)
            .or_else(|_| {
                pool_price_a_in_b(ref_sym, token.as_ref())
                    .map(|p| if p > 0.0 { 1.0 / p } else { f64::NAN })
            }) {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => continue,
        };

        // Second leg: REF/USD
        let ref_usd = if is_stable(ref_sym) || ref_sym == "USD" {
            1.0
        } else {
            get_rate_vs_usd(ref_sym.to_string()).await
                .map_err(|e| format!("Failed {ref_sym}/USD: {e}"))?
        };

        return Ok(tkn_in_ref * ref_usd);
    }

    Err(format!(
        "No USD route found for {} (no pools to anchors: USDT/USDC/ICP/ETH/BTC)",
        token
    ))
}
