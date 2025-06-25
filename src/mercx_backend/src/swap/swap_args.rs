use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::transfers::tx_id::TxId;

/// Data structure for the arguments of the `swap` function.
/// Used in StableRequest
#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct SwapArgs {
    pub pay_token: String,  //The symbol or identifier of the token the user wants to sell/pay with (e.g., "ckUSDT").
    pub pay_amount: Nat, //	The amount of pay_token the user wants to swap.
    pub pay_tx_id: Option<TxId>,
    pub receive_token: String, //The symbol of the token the user wants to receive (e.g., "FXMX").
    pub receive_amount: Option<Nat>, //	(Optional) Minimum amount of receive_token the user expects to get — used for limit orders or protecting against slippage.
    pub receive_address: Option<String>, //(Optional) If provided, the tokens will be sent to this address instead of the caller's default account.
    pub max_slippage: Option<f64>, //	(Optional) Allowed price deviation (e.g., 0.01 = 1% max slippage). Protects users from unfavorable price changes.
   // pub referred_by: Option<String>,
}


//So, if you allow 1% slippage and the price shifts 3%, the swap fails.