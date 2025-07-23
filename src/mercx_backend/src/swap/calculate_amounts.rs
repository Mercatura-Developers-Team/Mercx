use candid::Nat;

use super::swap_amounts::swap_amounts;
use super::swap_calc::SwapCalc;

use crate::helpers::math_helpers::{nat_is_zero, nat_to_decimals_f64};
use crate::token::{stable_token::StableToken};

//only betet2aked en slippage and min is within user range and if yes return paramters 
//This function calculate_amounts is the final gatekeeper before performing a token swap. It computes the expected output (amount of tokens received)
pub fn calculate_amounts(
    pay_token: &StableToken,
    pay_amount: &Nat,
    receive_token: &StableToken,
    user_receive_amount: Option<&Nat>, //// Optional: Minimum user expects to receive
    user_max_slippage: f64,   //  // Optional: Max % slippage user is willing to tolerate
) -> Result<(Nat, f64, f64, f64, Vec<SwapCalc>), String> {
    let (receive_amount_with_fees_and_gas, price, mid_price, slippage, txs) = swap_amounts(pay_token, Some(pay_amount), receive_token)?;

    // make sure receive_amount is not zero
    if nat_is_zero(&receive_amount_with_fees_and_gas) {
        Err("Receive amount is zero".to_string())?;
    }

    // check if receive_amount_with_fees_and_gas is within user's specified
    //If the user specified a minimum acceptable amount and the result is below that, the function rejects the swap.
    if let Some(user_receive_amount) = user_receive_amount {
        // receive_amount_with_fees_and_gas < min user expects to receive
        if receive_amount_with_fees_and_gas < *user_receive_amount {
            let decimals = receive_token.decimals();
            let receive_amount_with_fees_and_gas_f64 = nat_to_decimals_f64(decimals, &receive_amount_with_fees_and_gas).unwrap_or(0_f64);
            Err(format!(
                "Insufficient receive amount. Can only receive {} {} with {}% slippage",
                receive_amount_with_fees_and_gas_f64,
                receive_token.symbol(),
                slippage
            ))?
        }
    }

    // check if slippage is within user's specified
    if slippage > user_max_slippage {
        let decimals = receive_token.decimals();
        let receive_amount_with_fees_and_gas_f64 = nat_to_decimals_f64(decimals, &receive_amount_with_fees_and_gas).unwrap_or(0_f64);
        Err(format!(
            "Slippage exceeded. Can only receive {} {} with {}% slippage",
            receive_amount_with_fees_and_gas_f64,
            receive_token.symbol(),
            slippage
        ))?
    }

    Ok((receive_amount_with_fees_and_gas, mid_price, price, slippage, txs))
}