use super::add_liquidity_reply::AddLiquidityReply;
use crate::helpers::math_helpers::nat_zero;
use crate::transfers::transfer_reply_helpers::to_transfer_ids;
use crate::token::stable_token::symbol;
use crate::StablePool;
use crate::StableToken;



pub fn to_add_liquidity_reply(pool: &StablePool,token0:&StableToken,token1:&StableToken, transfer_ids: &[u64],) -> AddLiquidityReply {

    AddLiquidityReply {
        pool_id: pool.pool_id,
        name:pool.name(),
        symbol:symbol(token0,token1),
        address_0:pool.canister_id_0(),
        symbol_0:pool.symbol_0(),
        amount_0: pool.balance_0.clone(),
        address_1:pool.canister_id_1(),
        symbol_1:pool.symbol_1(),
        amount_1: pool.balance_1.clone(),
        //lazem neghayrha
        lp_fee_bps:0, 
        lp_token_symbol: "LP token not added".to_string(),
        lp_token_amount: nat_zero(),
        transfer_ids: to_transfer_ids(transfer_ids),
      }
  }

  pub fn to_add_liquidity_reply_failed(
    pool: &StablePool,
    address_0: &str,
    symbol_0: &str,
    address_1: &str,
    symbol_1: &str,
    transfer_ids: &[u64],
 
) -> AddLiquidityReply {
    AddLiquidityReply {
        pool_id: pool.pool_id,
        name: "Pool not added".to_string(),
        symbol: "Pool not added".to_string(),
        address_0: address_0.to_string(),
        symbol_0: symbol_0.to_string(),
        amount_0: nat_zero(),
        address_1: address_1.to_string(),
        symbol_1: symbol_1.to_string(),
        amount_1: nat_zero(),
        lp_fee_bps: 0,
        lp_token_symbol: "LP token not added".to_string(),
        lp_token_amount: nat_zero(),
        transfer_ids: to_transfer_ids(transfer_ids),
    }
}
