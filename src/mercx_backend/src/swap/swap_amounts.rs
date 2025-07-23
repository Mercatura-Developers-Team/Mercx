use candid::Nat;
use num::rational::BigRational;
use num::{FromPrimitive, One, Zero};
use num::ToPrimitive; 

use crate::pool::handlers;
use crate::swap::swap_calc::SwapCalc;
use crate::helpers::math_helpers::{nat_to_decimal_precision,nat_divide,nat_multiply,nat_subtract,nat_add,nat_zero,nat_is_zero,round_f64,price_rounded,nat_multiply_f64};
use crate::StablePool;
use crate::StableToken;

/// calculate the receive_amount of a swap using mid price
/// returns the receive_amount
pub fn swap_mid_amounts(pay_token: &StableToken, pay_amount: &Nat, receive_token: &StableToken) -> Result<Nat, String> {
    let mid_price = swap_mid_price(pay_token, receive_token)?;
    let receive_amount_pay_token_decimal = nat_multiply_f64(pay_amount, mid_price).ok_or("Failed to mid price")?;
    let receive_amount = nat_to_decimal_precision(&receive_amount_pay_token_decimal, pay_token.decimals(), receive_token.decimals());
    Ok(receive_amount)
}

//Passing None means:
// You don't want to perform a swap.
// You only want to know the mid-market price — the price based on pool reserves, not factoring slippage or gas fees.
pub fn swap_mid_price(pay_token: &StableToken, receive_token: &StableToken) -> Result<f64, String> {
    let (_, _, mid_price, _, _) = swap_amounts(pay_token, None, receive_token)?;
    Ok(mid_price)
}

/// calculate the receive_amount of a swap using pool price (bid/offer, fee and gas included)
/// returns the receive_amount, price, mid_price, slippage and the pools used
///
/// pay_token - pay token
/// pay_amount - amount of pay token. pay_amount is None if only mid price is requested
/// receive_token - receive token
pub fn swap_amounts(
    pay_token: &StableToken,
    pay_amount: Option<&Nat>,
    receive_token: &StableToken,
) -> Result<(Nat, f64, f64, f64, Vec<SwapCalc>), String> {
    let pay_token_id = pay_token.token_id();
    let receive_token_id = receive_token.token_id();

    // if tokens are the same return the same amount
    if pay_token_id == receive_token_id {
        // if pay_amount is None, set receive_amount = 0 and return 1.0
        let receive_amount = pay_amount.unwrap_or(&nat_zero()).clone();
        return Ok((receive_amount, 1.0, 1.0, 0.0, Vec::new()));
    }

    // if pay_amount is None, user_fee_level is None as only mid_price is needed
   // let user_fee_level = pay_amount.map(|_| user_map::get_by_caller().ok().flatten().unwrap_or_default().fee_level);
   let user_fee_level = None;

    // swaps stores all the swap permutations
    let mut swaps: Vec<(Nat, f64, f64, f64, Vec<SwapCalc>)> = Vec::new();

    // 1-step swap
    one_step_swaps(pay_token_id, pay_amount, receive_token_id, user_fee_level, &mut swaps)?;

    // 2-step swap
    // two_step_swaps(pay_token_id, pay_amount, receive_token_id, user_fee_level, &mut swaps)?;

    // // 3-step swap
    // three_step_swaps(pay_token_id, pay_amount, receive_token_id, user_fee_level, &mut swaps)?;

    let max_swap = if pay_amount.is_none() {
        // return the swap with the highest mid_price
        swaps
            .into_iter()
            .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
            .ok_or("Invalid swap")?
    } else {
        // return the swap with the highest receive amount
        swaps.into_iter().max_by(|a, b| a.0.cmp(&b.0)).ok_or("Invalid swap")?
    };

    Ok(max_swap)
}


/// returns (receive_amount_with_gas_and_fees, price, mid_price, slippage, swap)
fn one_step_swaps(
    pay_token_id: u32,
    pay_amount: Option<&Nat>,
    receive_token_id: u32,
    user_fee_level: Option<u8>,
    swaps: &mut Vec<(Nat, f64, f64, f64, Vec<SwapCalc>)>,
) -> Result<(), String> {
    let swap = if let Some(pool) = handlers::get_by_token_ids(pay_token_id, receive_token_id) {
        swap_amount_0(&pool, pay_amount, user_fee_level, None, None)?
    } else if let Some(pool) = handlers::get_by_token_ids(receive_token_id, pay_token_id) {
        swap_amount_1(&pool, pay_amount, user_fee_level, None, None)?
    } else {
        return Ok(());
    };
//for just showing price 
    if pay_amount.is_none() {
        // if pay_amount is None, return the mid price
        let mid_price = swap.get_mid_price().unwrap_or(BigRational::zero());
        let mid_price_f64 = price_rounded(&mid_price).ok_or("Invalid mid price")?;
        swaps.push((nat_zero(), mid_price_f64, mid_price_f64, 0.0, vec![swap]));
    } else { //Computes actual amount received (after LP and gas fees)
        let receive_amount = swap.receive_amount_with_fees_and_gas();
        let price = swap.get_price().unwrap_or(BigRational::zero());
        let price_f64 = price_rounded(&price).ok_or("Invalid price")?;
        let mid_price = swap.get_mid_price().unwrap_or(BigRational::zero());
        let mid_price_f64 = price_rounded(&mid_price).ok_or("Invalid mid price")?;
        let slippage_f64 = get_slippage(&price, &mid_price).unwrap_or(0_f64);
        swaps.push((receive_amount, price_f64, mid_price_f64, slippage_f64, vec![swap]));
    }

    Ok(())


}


// 📦 Example Scenario
// User wants to swap 100 FXMX for ckUSDT.

// Pool holds: 10,000 FXMX and 5,000 ckUSDT.

// LP Fee: 0.3% (30 bps).

// User has a 50% discount (user_fee_level = 50 → pays only 50% of the fee).

// Result:

// They receive ~49.7 ckUSDT.

// LP fee = 0.075 ckUSDT.

// Gas fee = 0.025 ckUSDT.
/// Swap amount 0 of a given pool
/// use_lp_fee and use_gas_fee are used to overwrite the default LP and gas fees, if None, then use the pool's default
/// User provides Token 0, and the function calculates how much Token 1 they’ll receive.
fn swap_amount_0(
    pool: &StablePool,
    amount_0: Option<&Nat>,
    user_fee_level: Option<u8>, // user specific fee level, 0 = 100% fee (no discount), 100 = 0% fee (max discount)
    use_lp_fee: Option<u8>,     // overwrite for LP fee in case of 2-legged synthetic swaps
    use_gas_fee: Option<&Nat>,  // overwrite for gas fee in case of synethetic swaps
) -> Result<SwapCalc, String> {
    // Token 0
    let token_0 = pool.token_0();
    let token_id_0 = token_0.token_id();
    // Token 1
    let token_1 = pool.token_1();
    let token_id_1 = token_1.token_id();

    let reserve_0 = nat_add(&pool.balance_0, &pool.lp_fee_0);
    let reserve_1 = nat_add(&pool.balance_1, &pool.lp_fee_1);

    //No liquidity 
    if nat_is_zero(&reserve_0) || nat_is_zero(&reserve_1) {
        return Ok(SwapCalc {
            pool_id: pool.pool_id,
            pay_token_id: token_id_0,
            pay_amount: nat_zero(),
            receive_token_id: token_id_1,
            receive_amount: nat_zero(),
            lp_fee: nat_zero(),
            gas_fee: nat_zero(),
        });
    }

    //If the user just wants the price 
    // Frontend will then call .get_price() on this object.
    let amount_0 = match amount_0 {
        None => {
            // return "mid" swap price if amount_0 is none
            return Ok(SwapCalc {
                pool_id: pool.pool_id,
                pay_token_id: token_id_0,
                pay_amount: nat_zero(),
                receive_token_id: token_id_1,
                receive_amount: nat_zero(),
                lp_fee: nat_zero(),
                gas_fee: nat_zero(),
            });
        }
        Some(amount) => amount,
    };

    // convert amount_0 and pool balances to the max_decimals precision
    let max_decimals = std::cmp::max(token_0.decimals(), token_1.decimals());
    let reserve_0_in_max_decimals = nat_to_decimal_precision(&reserve_0, token_0.decimals(), max_decimals);
    let reserve_1_in_max_decimals = nat_to_decimal_precision(&reserve_1, token_1.decimals(), max_decimals);
    let amount_0_in_max_decimals = nat_to_decimal_precision(amount_0, token_0.decimals(), max_decimals);

    // amount_1 = (amount_0 * reserve_1) / (reserve_0 + amount_0)
    let numerator_in_max_decimals = nat_multiply(&amount_0_in_max_decimals, &reserve_1_in_max_decimals);
    let denominator_in_max_decimals = nat_add(&reserve_0_in_max_decimals, &amount_0_in_max_decimals);
    let amount_1_in_max_decimals = nat_divide(&numerator_in_max_decimals, &denominator_in_max_decimals).ok_or("Invalid amount_1")?;

    // calculate the LP fees
    // any user fee discount. user.fee_level is 0 = 100% fee (no discount), 100 = 0% fee (max discount)
    // user_lp_fee_pct = 100 - user.fee_level
    let user_lp_fee_pct = nat_subtract(&Nat::from(100_u8), &Nat::from(user_fee_level.unwrap_or(0_u8))).unwrap_or(Nat::from(100_u8));
    // user_lp_fee_bps = (user_lp_fee * user_lp_fee_pct) / 100 - user's fee level in bps with discount
    let user_lp_fee_bps = nat_divide(
        &nat_multiply(&user_lp_fee_pct, &Nat::from(use_lp_fee.unwrap_or(pool.lp_fee_bps))),
        &Nat::from(100_u8),
    )
    .ok_or("Invalid LP fee")?;
    // lp_fee_1 = (amount_1 * user_lp_fee_bps) / 10_000
    let numerator_in_max_decimals = nat_multiply(&amount_1_in_max_decimals, &user_lp_fee_bps);
    let lp_fee_1_in_max_decimals = nat_divide(&numerator_in_max_decimals, &Nat::from(10_000_u128)).ok_or("Invalid LP fee")?;

    // convert amount_1 and lp_fee_1 from max_decimals to token_1 precision
    let amount_1 = nat_to_decimal_precision(&amount_1_in_max_decimals, max_decimals, token_1.decimals());
    let lp_fee = nat_to_decimal_precision(&lp_fee_1_in_max_decimals, max_decimals, token_1.decimals());
    let gas_fee = use_gas_fee.map_or_else(|| token_1.fee(), |fee| fee.clone());

    if amount_1 > reserve_1 {
        Err(format!("Insufficient {} in pool", token_1.symbol()))?
    }

    Ok(SwapCalc {
        pool_id: pool.pool_id,
        pay_token_id: token_id_0,
        pay_amount: amount_0.clone(),
        receive_token_id: token_id_1,
        receive_amount: amount_1,
        lp_fee,
        gas_fee,
    })
}


/// Swap amount 1 of a given pool
fn swap_amount_1(
    pool: &StablePool,
    amount_1: Option<&Nat>,
    user_fee_level: Option<u8>,
    use_lp_fee: Option<u8>,
    use_gas_fee: Option<&Nat>,
) -> Result<SwapCalc, String> {
    // Token 0
    let token_0 = pool.token_0();
    let token_id_0 = token_0.token_id();
    // Token 1
    let token_1 = pool.token_1();
    let token_id_1 = token_1.token_id();

    let reserve_0 = nat_add(&pool.balance_0, &pool.lp_fee_0);
    let reserve_1 = nat_add(&pool.balance_1, &pool.lp_fee_1);

    if nat_is_zero(&reserve_0) || nat_is_zero(&reserve_1) {
        return Ok(SwapCalc {
            pool_id: pool.pool_id,
            pay_token_id: token_id_1,
            pay_amount: nat_zero(),
            receive_token_id: token_id_0,
            receive_amount: nat_zero(),
            lp_fee: nat_zero(),
            gas_fee: nat_zero(),
        });
    }

    let amount_1 = match amount_1 {
        None => {
            // return "mid" swap price if amount_1 is none
            return Ok(SwapCalc {
                pool_id: pool.pool_id,
                pay_token_id: token_id_1,
                pay_amount: nat_zero(),
                receive_token_id: token_id_0,
                receive_amount: nat_zero(),
                lp_fee: nat_zero(),
                gas_fee: nat_zero(),
            });
        }
        Some(amount) => amount,
    };

    // convert amount_1 and pool balances to the max_decimals precision
    let max_decimals = std::cmp::max(token_0.decimals(), token_1.decimals());
    let reserve_0_in_max_decimals = nat_to_decimal_precision(&reserve_0, token_0.decimals(), max_decimals);
    let reserve_1_in_max_decimals = nat_to_decimal_precision(&reserve_1, token_1.decimals(), max_decimals);
    let amount_1_in_max_decimals = nat_to_decimal_precision(amount_1, token_1.decimals(), max_decimals);

    // amount_0 = (amount_1 * reserve_0) / (reserve_1 + amount_1)
    let numerator_in_max_decimals = nat_multiply(&amount_1_in_max_decimals, &reserve_0_in_max_decimals);
    let denominator_in_max_decimals = nat_add(&reserve_1_in_max_decimals, &amount_1_in_max_decimals);
    let amount_0_in_max_decimals = nat_divide(&numerator_in_max_decimals, &denominator_in_max_decimals).ok_or("Invalid amount_0")?;

    // calculate the LP fees
    // user_lp_fee_pct = 100 - user.fee_level
    let user_lp_fee_pct = nat_subtract(&Nat::from(100_u8), &Nat::from(user_fee_level.unwrap_or(0_u8))).unwrap_or(Nat::from(100_u8));
    let user_lp_fee_bps = nat_divide(
        &nat_multiply(&user_lp_fee_pct, &Nat::from(use_lp_fee.unwrap_or(pool.lp_fee_bps))),
        &Nat::from(100_u8),
    )
    .ok_or("Invalid LP fee")?;
    let numerator_in_max_decimals = nat_multiply(&amount_0_in_max_decimals, &user_lp_fee_bps);
    let lp_fee_0_in_max_decimals = nat_divide(&numerator_in_max_decimals, &Nat::from(10_000_u128)).ok_or("Invalid LP fee")?;

    // convert amount_0 and lp_fee_0 to token_0 precision
    let amount_0 = nat_to_decimal_precision(&amount_0_in_max_decimals, max_decimals, token_0.decimals());
    let lp_fee = nat_to_decimal_precision(&lp_fee_0_in_max_decimals, max_decimals, token_0.decimals());
    let gas_fee = use_gas_fee.map_or_else(|| token_0.fee(), |fee| fee.clone());

    if amount_0 > reserve_0 {
        Err(format!("Insufficient {} in pool", token_0.symbol()))?
    }

    Ok(SwapCalc {
        pool_id: pool.pool_id,
        pay_token_id: token_id_1,
        pay_amount: amount_1.clone(),
        receive_token_id: token_id_0,
        receive_amount: amount_0,
        lp_fee,
        gas_fee,
    })
}


//calculates how much worse the actual swap price is compared to the expected price
//Slippagd is an optional parameter in swap logic to cancel trades if slippage exceeds a certain threshold
//Helps you detect if your pool needs more liquidity or better fee logic.
//price_achieved: the actual price you got in the swap.
//price_expected: the price you hoped or estimated you'd get.
fn get_slippage(price_achieved: &BigRational, price_expected: &BigRational) -> Option<f64> {
    //If the achieved price is better than expected
    if price_achieved > price_expected {
        return Some(0.0); // if price is greater than expected, slippage is 0
    }
    //If price_expected = 0, the logic is invalid → return None.
    if price_expected.is_zero() {
        None?;
    }

    //Calculate slippage when price is worse
    // slippage = 100 * (price_achieved / price_expected - 1)
    let raw_slippage = (BigRational::from_i32(100)? * (price_achieved / price_expected - BigRational::one()))
        .to_f64()?
        .abs();
    Some(round_f64(raw_slippage, 2)) // 2 decimals 
}