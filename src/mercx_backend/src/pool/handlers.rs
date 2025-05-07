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



// #[ic_cdk::update]
// fn add_pool(args: AddPoolArgs) -> AddPoolReply {
//     POOLS.with(|pools| {
//         let mut pools = pools.borrow_mut();

//         let pool_id: u32 = pools.len() as u32 + 1;

//         // For now, set token_id_0 and token_id_1 based on fixed mappings or future token registry
//         let token_id_0: u32 = 1; // FXMX (example)
//         let token_id_1: u32 = 2; // ckUSDT (example)

//        // let lp_fee_bps = args.lp_fee_bps.unwrap_or(30); // Default to 0.3%
//        let lp_fee_bps = 30;
//         let kong_fee_bps = 5; // Fixed MercX fee in BPS (can be made dynamic later)
//         let lp_token_id = 1000 + pool_id; // Example LP token id generation

//         let pool = StablePool {
//             pool_id,
//             token_id_0,
//             balance_0: args.amount_0.clone(),
//             lp_fee_0: nat_zero(),
//             mercx_fee_0: nat_zero(),
//             token_id_1,
//             balance_1: args.amount_1.clone(),
//             lp_fee_1: nat_zero(),
//             mercx_fee_1: nat_zero(),
//             lp_fee_bps,
//             kong_fee_bps,
//             lp_token_id,
//         };

//         pools.insert(StablePoolId(pool_id), pool.clone());

//         AddPoolReply {
//             pool_id,
//             symbol: format!("{}_{}", args.token_0, args.token_1),
//             name: format!("{}_{} Liquidity Pool", args.token_0, args.token_1),
//             symbol_0: args.token_0.clone(),
//             address_0: format!("canister://{}", args.token_0), // placeholder address format
//             amount_0: args.amount_0,
//             symbol_1: args.token_1.clone(),
//             address_1: format!("canister://{}", args.token_1), // placeholder
//             amount_1: args.amount_1,
//             lp_fee_bps,
//             lp_token_symbol: format!("{}_{}_LP", args.token_0, args.token_1),
//             lp_token_amount: Nat::from(1_000_000_u64), // Placeholder LP minting logic
//             tx_id: pool_id as u64,
//             status: "Success".to_string(),
//             ts: ic_cdk::api::time(), // Replace with real-time from `ic_cdk::api::time()` or `time::OffsetDateTime`
//         }
//     })
// }

#[ic_cdk::query]
fn get_all_pools() -> Vec<AddPoolReply> {
    POOLS.with(|pools| {
        pools.borrow()
            .iter()
            .map(|(_, pool)| {
                AddPoolReply {
                    pool_id: pool.pool_id,
                    symbol: format!("{}_{}", pool.token_id_0, pool.token_id_1), // You might need mapping later
                    name: format!("{}_{} Liquidity Pool", pool.token_id_0, pool.token_id_1),
                    symbol_0: pool.token_id_0.to_string(), // temporary placeholder
                    address_0: format!("canister://{}", pool.canister_id_0()),
                    amount_0: pool.balance_0.clone(),
                    symbol_1: pool.token_id_1.to_string(), // temporary placeholder
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