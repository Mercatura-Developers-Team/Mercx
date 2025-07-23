use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::transfers::transfer_reply::TransferIdReply;



#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct AddLiquidityReply {
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
    pub transfer_ids: Option<Vec<TransferIdReply>>,
 
}
