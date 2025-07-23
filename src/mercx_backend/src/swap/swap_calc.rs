use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};


//The SwapCalc struct is a calculation result structure used to pre-calculate a swap before executing it.
//It tells you how much you'll receive, and how much you'll pay in fees — without executing the actual transaction.
#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct SwapCalc {
    pub pool_id: u32,
    // pay and receive are from the point of view of the user
    pub pay_token_id: u32, //3 //// FXMX
    pub pay_amount: Nat,
    pub receive_token_id: u32,
    pub receive_amount: Nat, // does not include any fees. used to keep a constant K with pay amount
    pub lp_fee: Nat,         // will be in receive_token //// 0.5% LP fee
    pub gas_fee: Nat,        // will be in receive_token //// fixed or % based gas fee
}