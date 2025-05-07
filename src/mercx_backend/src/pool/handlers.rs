use candid::Nat;
use crate::pool::stable_pool::{StablePool,StablePoolId};
use crate::stable_memory::POOLS;
use crate::AddPoolReply;
use crate::StableToken;
use crate::token::handlers;
use crate::stable_mercx_settings;


pub fn symbol(token_0: &StableToken, token_1: &StableToken) -> String {
    format!("{}_{}", token_0.symbol(), token_1.symbol())
}

pub fn get_by_pool_id(pool_id: u32) -> Option<StablePool> {
    POOLS.with(|m| m.borrow().get(&StablePoolId(pool_id)))
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

pub fn get_by_tokens(token_0: &str, token_1: &str) -> Result<StablePool, String> {
    let token_0: StableToken = handlers::get_by_symbol(token_0)?;
    let token_1 = handlers::get_by_symbol(token_1)?;
    get_by_token_ids(token_0.token_id(), token_1.token_id()).ok_or_else(|| format!("Pool {} not found", symbol(&token_0, &token_1)))
}

pub fn nat_zero() -> Nat {
    Nat::from(0_u128)
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

#[ic_cdk::query]
fn get_all_pools() -> Vec<AddPoolReply> {
    POOLS.with(|pools| {
        pools.borrow()
            .iter()
            .map(|(_, pool)| {
                AddPoolReply {
                    pool_id: pool.pool_id,
                    symbol: format!("{}_{}", pool.symbol_0(), pool.symbol_1()),
                    name: format!("{} Liquidity Pool", pool.name()),
                    symbol_0: pool.symbol_0(), // temporary placeholder
                    address_0: format!("canister://{}", pool.canister_id_0()),
                    amount_0: pool.balance_0.clone(),
                    symbol_1: pool.symbol_1(), // temporary placeholder
                    address_1: format!("canister://{}", pool.canister_id_1()),
                    amount_1: pool.balance_1.clone(),
                    lp_fee_bps: pool.lp_fee_bps,
                    lp_token_symbol: format!("{}_{}_LP", pool.token_id_0, pool.token_id_1),
                    lp_token_amount: Nat::from(1_000_000_u64), // This should be real LP minted later
                    ts: ic_cdk::api::time(), // Or if you save ts at creation inside StablePool later, use that
                }
            })
            .collect()
    })
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

//It stores the updated version of a liquidity pool into the global state
pub fn update(pool: &StablePool) {
    POOLS.with(|m| m.borrow_mut().insert(StablePoolId(pool.pool_id), pool.clone()));
}