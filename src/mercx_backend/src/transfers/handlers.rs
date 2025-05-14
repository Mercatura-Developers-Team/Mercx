use candid::Nat;

use super::tx_id::TxId;
use crate::stable_memory::TRANSFERS;
use crate::stable_transfer::stable_transfer::{StableTransfer, StableTransferId};

pub fn get_by_transfer_id(transfer_id: u64) -> Option<StableTransfer> {
    TRANSFERS.with(|m| m.borrow().get(&StableTransferId(transfer_id)))
}


pub fn exist(token_id: u32, block_id: &Nat) -> bool {
    TRANSFERS.with(|m| {
        m.borrow()
            .iter()
            .any(|(_, v)| v.token_id == token_id && v.tx_id == TxId::BlockIndex(block_id.clone()))
    })
}

