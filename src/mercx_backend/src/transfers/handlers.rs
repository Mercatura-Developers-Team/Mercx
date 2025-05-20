use candid::Nat;

use super::tx_id::TxId;
use crate::stable_memory::TRANSFERS;
use crate::transfers::stable_transfer::{StableTransfer, StableTransferId};
use crate::stable_mercx_settings::mercx_settings_map;

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

pub fn insert(transfer: &StableTransfer) -> u64 {
    TRANSFERS.with(|m| {
        let mut map = m.borrow_mut();
        let transfer_id = mercx_settings_map::inc_transfer_map_idx();
        let insert_transfer = StableTransfer {
            transfer_id,
            ..transfer.clone()
        };
        map.insert(StableTransferId(transfer_id), insert_transfer);
        transfer_id
    })
}