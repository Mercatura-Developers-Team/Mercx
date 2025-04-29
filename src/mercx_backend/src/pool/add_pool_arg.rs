use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};


#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct AddPoolArgs {
    pub token_0: String,             // e.g. FXMX
    pub amount_0: Nat,               // amount to deposit of token 0
    pub token_1: String,             // e.g. ckUSDT
    pub amount_1: Nat,               // amount to deposit of token 1
    pub lp_fee_bps: Option<u8>,      // optional fee in basis points, default = 30 //for each swap
}

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
    pub tx_id: u64,                  // tx id of creation
    pub status: String,              // e.g. "Success"
    pub is_removed: bool,
    pub ts: u64,                     // timestamp of creation
}

