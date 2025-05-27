
use crate::transfers::transfer_reply::{ICTransferReply,TransferIdReply};
use crate::transfers::tx_id::TxId;
use crate::token::handlers as token_handlers;
use crate::transfers::handlers;

pub fn to_transfer_ids(transfer_ids: &[u64]) -> Option<Vec<TransferIdReply>> {
    transfer_ids.iter().map(|&transfer_id| to_transfer_id(transfer_id)).collect()
}

pub fn to_transfer_id(transfer_id: u64) -> Option<TransferIdReply> {
    match handlers::get_by_transfer_id(transfer_id) {
        Some(transfer) => match token_handlers::get_by_token_id(transfer.token_id) {
            Some(token) => match transfer.tx_id {
                TxId::BlockIndex(block_index) => Some(TransferIdReply {
                    transfer_id,
                    transfer: ICTransferReply {
                        symbol: token.symbol,
                        is_send: transfer.is_send,
                        amount: transfer.amount,
                        canister_id: token.canister_id.to_string(),
                        block_index,
                    },
                }),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}