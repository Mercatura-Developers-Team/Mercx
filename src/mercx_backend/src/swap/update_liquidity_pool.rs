use candid::Nat;

use super::calculate_amounts::calculate_amounts;
use super::swap_calc::SwapCalc;

use crate::helpers::math_helpers::{nat_add, nat_divide, nat_multiply, nat_subtract, nat_zero};
use crate::pool::handlers;
use crate::StableToken;

pub fn update_liquidity_pool(
    pay_token: &StableToken,
    pay_amount: &Nat,
    receive_token: &StableToken,
    receive_amount: Option<&Nat>, //// user’s min expected receive
    max_slippage: f64,            //// user’s max allowed slippage %
) -> Result<(Nat, f64, f64, f64, Vec<SwapCalc>), String> {
    // request_map::update_status(request_id, StatusCode::CalculatePoolAmounts, None);

    //If successful, it proceeds to actually update the pool balances.
    match calculate_amounts(
        pay_token,
        pay_amount,
        receive_token,
        receive_amount,
        max_slippage,
    ) {
        Ok((receive_amount_with_fees_and_gas, price, mid_price, slippage, swaps)) => {
            // request_map::update_status(request_id, StatusCode::CalculatePoolAmountsSuccess, None);

            // update the pool, in some cases there could be multiple pools
            //Loop over the swaps and update each pool involved
            // request_map::update_status(request_id, StatusCode::UpdatePoolAmounts, None);
            for swap in &swaps {
                // refresh pool with the latest state
                let mut pool = match handlers::get_by_pool_id(swap.pool_id) {
                    Some(pool) => pool,
                    None => continue, // should not get here
                };

                //Means the direction is: token_0 → token_1
                if swap.receive_token_id == pool.token_id_1 {
                    // user pays token_0 and receives token_1
                    //Increase balance of token_0 (user gave this).
                    pool.balance_0 = nat_add(&pool.balance_0, &swap.pay_amount); // pay_amount is in token_0
                                                                                 //Decrease balance of token_1 (user received this).
                    pool.balance_1 =
                        nat_subtract(&pool.balance_1, &swap.receive_amount).unwrap_or(nat_zero()); // receive_amount is in token_1
                                                                                                   // fees are in token_1. take out Kong's fee
                                                                                                   // mercx_fee_1 = lp_fee * kong_fee_bps / lp_fee_bps
                                                                                                   // kong_fee_bps: How much of the total LP fee goes to MercX (e.g., 500 bps = 5%)
                                                                                                   // lp_fee_bps: Total fee collected from swap (e.g., 3000 bps = 30%)                                                                                        // lp_fee_1 = lp_fee - mercx_fee_1
                    let numerator = nat_multiply(&swap.lp_fee, &Nat::from(pool.kong_fee_bps)); //swap.lp_fee is in token_1
                    let mercx_fee_1 =
                        nat_divide(&numerator, &Nat::from(pool.lp_fee_bps)).unwrap_or(nat_zero());
                    let lp_fee_1 = nat_subtract(&swap.lp_fee, &mercx_fee_1).unwrap_or(nat_zero());
                    pool.lp_fee_1 = nat_add(&pool.lp_fee_1, &lp_fee_1);
                    pool.mercx_fee_1 = nat_add(&pool.mercx_fee_1, &mercx_fee_1);
                } else {
                    // user pays token_1 and receives token_0
                    pool.balance_1 = nat_add(&pool.balance_1, &swap.pay_amount); // pay_amount is in token_1
                    pool.balance_0 =
                        nat_subtract(&pool.balance_0, &swap.receive_amount).unwrap_or(nat_zero()); // receive_amount is in token_0
                                                                                                   // fees are in token_0. take out Kong's fee
                                                                                                   // mercx_fee_0 = lp_fee * kong_fee_bps / lp_fee_bps
                                                                                                   // lp_fee_0 = lp_fee - mercx_fee_0
                    let numerator = nat_multiply(&swap.lp_fee, &Nat::from(pool.kong_fee_bps)); //swap.lp_fee is in token_0
                    let mercx_fee_0 =
                        nat_divide(&numerator, &Nat::from(pool.lp_fee_bps)).unwrap_or(nat_zero());
                    let lp_fee_0 = nat_subtract(&swap.lp_fee, &mercx_fee_0).unwrap_or(nat_zero());
                    pool.lp_fee_0 = nat_add(&pool.lp_fee_0, &lp_fee_0);
                    pool.mercx_fee_0 = nat_add(&pool.mercx_fee_0, &mercx_fee_0);
                }
                handlers::update(&pool);
            }

            //  request_map::update_status(request_id, StatusCode::UpdatePoolAmountsSuccess, None);

            Ok((
                receive_amount_with_fees_and_gas,
                mid_price,
                price,
                slippage,
                swaps,
            ))
        }
        Err(e) => {
            //   request_map::update_status(request_id, StatusCode::CalculatePoolAmountsFailed, Some(&e));
            Err(e)
        }
    }
}



// User swaps 1000 FXMX (token_0) to receive ckUSDT (token_1)

// Total swap fee is 30 bps = 0.3% = 3 ckUSDT

// MercX fee = 5 bps = 0.05% = 0.5 ckUSDT

// Then:

// mercx_fee_1 = 3 * 5 / 30 = 0.5

// lp_fee_1 = 3 - 0.5 = 2.5

// Pool updates:

// balance_0 += 1000 FXMX

// balance_1 -= amount received by user

// lp_fee_1 += 2.5

// mercx_fee_1 += 0.5