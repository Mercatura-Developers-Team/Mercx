use candid::{CandidType, Nat};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use ic_cdk::trap;

use super::tx_id::TxId;


#[derive(CandidType, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StableTransferId(pub u64);

impl Storable for StableTransferId {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        serde_cbor::to_vec(self)
            .unwrap_or_else(|e| {
                trap(&format!("❌ Failed to serialize StableTransferId: {:?}", e));
            })
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes)
            .unwrap_or_else(|e| {
                trap(&format!("❌ Failed to deserialize StableTransferId: {:?}", e));
            })
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct StableTransfer {
    pub transfer_id: u64,
    pub is_send: bool, // from user's perspective. so if is_send is true, it means the user is sending the token
    pub amount: Nat,
    pub token_id: u32, //The ID of the token being transferred.
    pub tx_id: TxId,   //The ID of the actual blockchain-level transaction.
    pub ts: u64,
}

impl Storable for StableTransfer {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        serde_cbor::to_vec(self)
            .unwrap_or_else(|e| {
                trap(&format!("❌ Failed to serialize StableTransfer: {:?}", e));
            })
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes)
            .unwrap_or_else(|e| {
                trap(&format!("❌ Failed to deserialize StableTransfer: {:?}", e));
            })
    }

    const BOUND: Bound = Bound::Unbounded;
}
