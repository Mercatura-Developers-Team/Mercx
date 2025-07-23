use candid::Nat;
use crate::stable_memory::POOLS;
use crate::pool::stable_pool::StablePoolId;
use crate::pool::stable_pool::StablePool;

// Utility functions
pub fn nat_mul(a: &Nat, b: &Nat) -> Nat {
    Nat::from(a.0.clone() * b.0.clone())
}

pub fn nat_div(a: &Nat, b: &Nat) -> Nat {
    Nat::from(a.0.clone() / b.0.clone())
}

// Fee calculation
#[ic_cdk::update]
pub fn calculate_fees(
    amount_in: Nat,
    lp_fee_bps: u8,
    kong_fee_bps: u8,
) -> (Nat, Nat, Nat) {
    let bps_divisor = Nat::from(10_000_u32);

    let fee_total = nat_div(&nat_mul(&amount_in, &Nat::from(lp_fee_bps)), &bps_divisor);
    let mercx_fee = nat_div(&nat_mul(&amount_in, &Nat::from(kong_fee_bps)), &bps_divisor);
    let lp_fee = fee_total.clone() - mercx_fee.clone();

    (fee_total, lp_fee, mercx_fee)
}

// ✅ Main logic to apply fees
pub fn apply_swap_fee_to_pool(
    amount_in: Nat,
    lp_fee_bps: u8,
    kong_fee_bps: u8,
    pool_lp_fee: &mut Nat,
    pool_mercx_fee: &mut Nat,
    pool_balance: &mut Nat,
) {
    let (fee_total, lp_fee, mercx_fee) = calculate_fees(amount_in.clone(), lp_fee_bps, kong_fee_bps);

// net amount that enters the pool reserves
    let amount_after_fee = amount_in.clone() - fee_total.clone();  // ✅ Corrected
// update pool fee trackers

    *pool_lp_fee += lp_fee;
    *pool_mercx_fee += mercx_fee;
    *pool_balance += amount_after_fee;
}


#[ic_cdk::update]
pub fn swap_example(amount_in: Nat) -> String {
   

    let pool_id = StablePoolId(1); // Example pool ID

    // Step 1: Get a cloned pool object
    let mut pool: StablePool = POOLS.with(|pools| {
        pools
            .borrow()
            .get(&pool_id)
            .expect("Pool not found")
            .clone()
    });
    
    // Step 2: Apply swap fees (mutates pool in-memory)
    apply_swap_fee_to_pool(
        amount_in,
        pool.lp_fee_bps,
        pool.kong_fee_bps,
        &mut pool.lp_fee_0,
        &mut pool.mercx_fee_0,
        &mut pool.balance_0,
    );

    // Step 3: Store the updated pool back
    POOLS.with(|pools| {
        pools.borrow_mut().insert(pool_id, pool);
    });

    "Swap applied successfully.".to_string()
}