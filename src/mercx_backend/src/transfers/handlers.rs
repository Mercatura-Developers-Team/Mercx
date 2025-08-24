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
    ic_cdk::println!("🔍 TRANSFERS accessed here");

    // 👇 Move ID generation *outside* to avoid double borrow
    let transfer_id = mercx_settings_map::inc_transfer_map_idx();

    ic_cdk::println!("🔍 TRANSFERS accessed here");
    TRANSFERS.with(|m| {
        let mut map = m.borrow_mut();
        let insert_transfer = StableTransfer {
            transfer_id,
            ..transfer.clone()
        };
        map.insert(StableTransferId(transfer_id), insert_transfer);
        transfer_id
    })
}


pub fn get_by_token_ids(token_id_0: u32, token_id_1: u32) -> Vec<u64> {
    TRANSFERS.with(|m| {
        m.borrow()
            .iter()
            .filter_map(|(id, tx)| {
                if tx.token_id == token_id_0 || tx.token_id == token_id_1 {
                    Some(id.0)
                } else {
                    None
                }
            })
            .collect()
    })
}