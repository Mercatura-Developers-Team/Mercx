use num::{BigRational,BigInt,Zero};
use num::bigint::Sign;
use core::cmp::Ordering;
use num::ToPrimitive;
use candid::Nat;
use num::{BigUint,pow};

// format price based on the amount
pub fn price_rounded(price: &BigRational) -> Option<f64> {
    let price_f64 = price.to_f64()?;
    if price_f64 <= 0.0001 {
        Some(round_f64(price_f64, 12)) // 12 decimals
    } else if price_f64 <= 0.1 {
        Some(round_f64(price_f64, 10)) // 10 decimals
    } else if price_f64 <= 20.0 {
        Some(round_f64(price_f64, 8)) // 8 decimals
    } else if price_f64 <= 100.0 {
        Some(round_f64(price_f64, 6)) // 6 decimals
    } else if price_f64 <= 500.0 {
        Some(round_f64(price_f64, 5)) // 5 decimals
    } else if price_f64 <= 5000.00 {
        Some(round_f64(price_f64, 4)) // 4 decimals
    } else if price_f64 <= 50000.00 {
        Some(round_f64(price_f64, 3)) // 3 decimals
    } else if price_f64 <= 100000.00 {
        Some(round_f64(price_f64, 2)) // 2 decimals
    } else {
        Some(round_f64(price_f64, 0)) // 0 decimals
    }
}

pub fn round_f64(f: f64, decimals: u8) -> f64 {
    let decimals_pow = 10_u64.pow(decimals.into()) as f64;
    let numerator = (f * decimals_pow).round();
    numerator / decimals_pow
}

// both Nat must have the same decimal precision
pub fn nat_add(n1: &Nat, n2: &Nat) -> Nat {
    n1.clone() + n2.clone()
}

pub fn nat_subtract(n1: &Nat, n2: &Nat) -> Option<Nat> {
    if n1 < n2 {
        None?
    }
    Some(n1.clone() - n2.clone())
}
pub fn nat_is_zero(n: &Nat) -> bool {
    n.0.is_zero()
}

pub fn nat_to_bigint(n: &Nat) -> BigInt {
    BigInt::from_bytes_be(Sign::Plus, &n.0.to_bytes_be())
}


pub fn nat_to_u64(n: &Nat) -> Option<u64> {
    n.0.to_u64()
}

pub fn nat_zero() -> Nat {
    Nat::from(0_u128)
}

// convert Nat from one decimal precision to another
// to convert from BTC (8 digit precision) to ETH (18 digit precision), call nat_to_decimals(n, 8, 18)
pub fn nat_to_decimal_precision(n: &Nat, from_decimal_precision: u8, to_decimal_precision: u8) -> Nat {
    match from_decimal_precision.cmp(&to_decimal_precision) {
        Ordering::Equal => n.clone(),
        Ordering::Less => {
            let decimal_diff = to_decimal_precision - from_decimal_precision;
            n.clone() * 10_u128.pow(decimal_diff as u32)
        }
        Ordering::Greater => {
            let decimal_diff = from_decimal_precision - to_decimal_precision;
            n.clone() / 10_u128.pow(decimal_diff as u32)
        }
    }
}

pub fn nat_divide(numerator: &Nat, denominator: &Nat) -> Option<Nat> {
    if nat_is_zero(numerator) {
        return Some(nat_zero());
    }
    if nat_is_zero(denominator) {
        None?
    }
    Some(numerator.clone() / denominator.clone())
}

pub fn nat_multiply(n1: &Nat, n2: &Nat) -> Nat {
    n1.clone() * n2.clone()
}

pub fn nat_sqrt(n: &Nat) -> Nat {
    Nat::from(n.0.sqrt())
}


pub fn nat_multiply_rational(n1: &Nat, n2: &BigRational) -> Option<Nat> {
    let numerator = nat_multiply(n1, &Nat::from(n2.numer().to_biguint()?));
    nat_divide(&numerator, &Nat::from(n2.denom().to_biguint()?))
}

pub fn nat_multiply_f64(n1: &Nat, n2: f64) -> Option<Nat> {
    let n2 = BigRational::from_float(n2)?;
    nat_multiply_rational(n1, &n2)
}

/// Convert Nat to f64 with decimals
pub fn nat_to_decimals_f64(decimals: u8, amount: &Nat) -> Option<f64> {
    let numerator = nat_to_biguint(amount);
    let denominator = pow(BigInt::from(10), decimals.into());
    let real_amount = BigRational::new(numerator.into(), denominator).to_f64()?;
    Some(round_f64(real_amount, decimals))
}

pub fn nat_to_biguint(n: &Nat) -> BigUint {
    BigUint::from_bytes_be(&n.0.to_bytes_be())
}