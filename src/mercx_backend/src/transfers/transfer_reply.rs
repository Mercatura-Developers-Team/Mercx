use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct TransferIdReply {
    pub transfer_id: u64,
    pub transfer: ICTransferReply, //The actual transfer info
}


//his is an enum designed to support multiple chains/types of transfers(kan fi)


#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct ICTransferReply {
    pub symbol: String,
    pub is_send: bool, // from user's perspective. so if is_send is true, it means the user is sending the token
    pub amount: Nat,
    pub canister_id: String,
    pub block_index: Nat,
}