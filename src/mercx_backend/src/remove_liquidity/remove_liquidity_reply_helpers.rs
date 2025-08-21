use super::remove_liquidity_reply::RemoveLiquidityReply;
use crate::helpers::math_helpers::nat_zero;
use candid::Nat;
use crate::pool::handlers;
use crate::transfers::transfer_reply_helpers::to_transfer_ids;

fn get_pool_info(pool_id: u32) -> (String, String, String, String, String) {
    handlers::get_by_pool_id(pool_id).map_or_else(
        || {
            (
                "Pool symbol not found".to_string(),
                "Pool address_0 not found".to_string(),
                "Pool symbol_0 not found".to_string(),
                "Pool address_1 not found".to_string(),
                "Pool symbol_1 not found".to_string(),
            )
        },
        |pool| {
            (
                pool.name(),
                pool.canister_id_0(),
                pool.symbol_0(),
                pool.canister_id_0(),
                pool.symbol_1(),
            )
        },
    )
}

pub fn to_remove_liquidity_reply(
    
    pool_id: u32,
    amount_0: Nat,
    lp_fee_0: Nat,
    amount_1: Nat,
    lp_fee_1: Nat,
    remove_lp_token_amount: Nat,
    transfer_ids: Vec<u64>,
    ts: u64,
) -> RemoveLiquidityReply {
    let (symbol, symbol_0, address_0, address_1, symbol_1) = get_pool_info(pool_id);

    RemoveLiquidityReply {
 
        symbol,
        address_0,
        symbol_0,
        amount_0,
        lp_fee_0,
        address_1,
        symbol_1,
        amount_1,
        lp_fee_1,
        remove_lp_token_amount,
        transfer_ids: to_transfer_ids(&transfer_ids).expect("error in transfer_ids"),
        ts,
    }
}


pub fn to_remove_liquidity_reply_failed(pool_id: u32,ts: u64) -> RemoveLiquidityReply {
    let (symbol, symbol_0, address_0, address_1, symbol_1) = get_pool_info(pool_id);
    RemoveLiquidityReply {
     //   tx_id: 0,
     //   request_id,
       // status: StatusTx::Failed.to_string(),
        symbol,
      //  chain_0,
        address_0,
        symbol_0,
        amount_0: nat_zero(),
        lp_fee_0: nat_zero(),
       // chain_1,
        address_1,
        symbol_1,
        amount_1: nat_zero(),
        lp_fee_1: nat_zero(),
        remove_lp_token_amount: nat_zero(),
        transfer_ids: Vec::new(), // if failed, transfer_ids is empty as no tokens are returned
      //  claim_ids: Vec::new(),    // if failed, claims_ids is empty as no LP tokens are returned
        ts,
    }
}