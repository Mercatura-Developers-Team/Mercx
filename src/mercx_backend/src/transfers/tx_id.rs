use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};
#[derive(Hash)]

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxId {
    BlockIndex(Nat), // Represents the index of the block
    TransactionHash(String), //An external blockchain like Ethereum or Solana, where transactions are identified by hash.
}