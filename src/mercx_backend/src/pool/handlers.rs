use candid::Nat;
use crate::pool::stable_pool::StablePool;
use crate::stable_memory::POOLS;
use crate::pool::stable_pool::StablePoolId;
use crate::AddPoolReply;
use crate::AddPoolArgs;

pub fn nat_zero() -> Nat {
    Nat::from(0_u128)
}


#[ic_cdk::update]
fn add_pool(args: AddPoolArgs) -> AddPoolReply {
    POOLS.with(|pools| {
        let mut pools = pools.borrow_mut();

        let pool_id: u32 = pools.len() as u32 + 1;

        // For now, set token_id_0 and token_id_1 based on fixed mappings or future token registry
        let token_id_0: u32 = 1; // FXMX (example)
        let token_id_1: u32 = 2; // ckUSDT (example)

        let lp_fee_bps = args.lp_fee_bps.unwrap_or(30); // Default to 0.3%
        let kong_fee_bps = 5; // Fixed MercX fee in BPS (can be made dynamic later)
        let lp_token_id = 1000 + pool_id; // Example LP token id generation

        let pool = StablePool {
            pool_id,
            token_id_0,
            balance_0: args.amount_0.clone(),
            lp_fee_0: nat_zero(),
            mercx_fee_0: nat_zero(),
            token_id_1,
            balance_1: args.amount_1.clone(),
            lp_fee_1: nat_zero(),
            mercx_fee_1: nat_zero(),
            lp_fee_bps,
            kong_fee_bps,
            lp_token_id,
            is_removed: false,
        };

        pools.insert(StablePoolId(pool_id), pool.clone());

        AddPoolReply {
            pool_id,
            symbol: format!("{}_{}", args.token_0, args.token_1),
            name: format!("{}_{} Liquidity Pool", args.token_0, args.token_1),
            symbol_0: args.token_0.clone(),
            address_0: format!("canister://{}", args.token_0), // placeholder address format
            amount_0: args.amount_0,
            symbol_1: args.token_1.clone(),
            address_1: format!("canister://{}", args.token_1), // placeholder
            amount_1: args.amount_1,
            lp_fee_bps,
            lp_token_symbol: format!("{}_{}_LP", args.token_0, args.token_1),
            lp_token_amount: Nat::from(1_000_000_u64), // Placeholder LP minting logic
            tx_id: pool_id as u64,
            status: "Success".to_string(),
            is_removed: false,
            ts: ic_cdk::api::time(), // Replace with real-time from `ic_cdk::api::time()` or `time::OffsetDateTime`
        }
    })
}

#[ic_cdk::query]
fn get_all_pools() -> Vec<StablePool> {
    POOLS.with(|pools| {
        pools.borrow()
            .iter()
            .map(|(_, pool)| pool.clone())
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