use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::transfers::transfer_reply::TransferIdReply;

//This struct represents the details of a single internal swap transaction between two tokens inside a liquidity pool.
#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct SwapTxReply {
    pub pool_symbol: String,  //The name of the liquidity pool used for the swap, e.g., "FXMX_ckUSDT".
   // pub pay_chain: String,
    #[serde(default = "empty_string")]
    pub pay_address: String, //Optional. The address from which payment was made.
    pub pay_symbol: String, //The symbol of the token being paid, e.g., "ckUSDT".
    pub pay_amount: Nat, //How much of the pay token was provided.
  //  pub receive_chain: String,
    #[serde(default = "empty_string")]
    pub receive_address: String, //The destination address receiving the tokens. 
    pub receive_symbol: String, //The symbol of the token received in the swap.
    pub receive_amount: Nat, // including fees
    pub price: f64, //The actual swap price 
    pub lp_fee: Nat,  // will be in receive_symbol //The liquidity provider fee, taken in the receive_symbol
    pub gas_fee: Nat, // will be in receive_symbol //The gas fee charged (if applicable), also in receive_symbol.
    pub ts: u64,
}


//This struct wraps up the entire swap operation, including metadata and a list of sub-transactions
#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct SwapReply {
   // pub tx_id: u64, //	Unique transaction ID of the swap.
   // pub request_id: u64,
  //  pub status: String,
   // pub pay_chain: String,
    #[serde(default = "empty_string")]
    pub pay_address: String, //Address of the user who paid 
    pub pay_symbol: String, //The symbol of the token sent.
    pub pay_amount: Nat, //Amount sent by the user.
   // pub receive_chain: String,
    #[serde(default = "empty_string")]
    pub receive_address: String, //Where the received token was sent to.
    pub receive_symbol: String,
    pub receive_amount: Nat, //Total amount received (after slippage and fees).
    pub mid_price: f64, //The theoretical mid-market price (used for slippage calculation).
    pub price: f64, //The actual execution price of the swap.
    pub slippage: f64, //The price difference in percentage from the expected mid-price.
    pub txs: Vec<SwapTxReply>, //All the internal steps/sub-swaps (e.g. if using multiple pools).
    pub transfer_ids: Vec<TransferIdReply>, //	Transfer IDs (from ICP ledger) for tracking movement of tokens.
   // pub claim_ids: Vec<u64>,
    pub ts: u64,
}

fn empty_string() -> String {
    String::new()
}