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
