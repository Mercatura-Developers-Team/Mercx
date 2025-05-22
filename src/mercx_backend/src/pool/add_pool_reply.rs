use candid::{CandidType, Nat,Principal};
use serde::{Deserialize, Serialize};

 use crate::helpers::math_helpers::nat_zero;
//for frontend API
#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct AddPoolReply {
    pub pool_id: u32,                // unique pool identifier
    pub symbol: String,              // FXMX_ckUSDT
    pub name: String,                // FXMX_ckUSDT Liquidity Pool
    pub symbol_0: String,            // FXMX
    pub address_0: String,           // token 0 address
    pub amount_0: Nat,               // deposited
    pub symbol_1: String,            // ckUSDT
    pub address_1: String,           // token 1 address
    pub amount_1: Nat,               // deposited
    pub lp_fee_bps: u8,              // confirmed LP fee
    pub lp_token_symbol: String,     // FXMX_ckUSDT_LP
    pub lp_token_amount: Nat,        // amount of LP tokens minted
    pub transfer_ids: Vec<u64>,
    pub ts: u64,                     // timestamp of creation
}

pub fn to_add_pool_reply_failed(
    address_0: &str,
    symbol_0: &str,
    address_1: &str,
    symbol_1: &str,
    transfer_ids: &[u64],
    ts: u64,
) -> AddPoolReply {
    AddPoolReply {
        pool_id: 0,
        name: "Pool not added".to_string(),
        symbol: "Pool not added".to_string(),
        address_0: address_0.to_string(),
        symbol_0: symbol_0.to_string(),
        amount_0: nat_zero(),
        address_1: address_1.to_string(),
        symbol_1: symbol_1.to_string(),
        amount_1: nat_zero(),
        lp_fee_bps: 0,
        lp_token_symbol: "LP token not added".to_string(),
        lp_token_amount: nat_zero(),
        transfer_ids: transfer_ids.clone(),
        ts,
    }
}