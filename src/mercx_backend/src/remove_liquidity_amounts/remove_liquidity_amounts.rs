use crate::pool::handlers;
use crate::remove_liquidity_amounts::remove_liquidity_amounts_reply::RemoveLiquidityAmountsReply;
use crate::remove_liquidity::remove_liquidity::calculate_amounts;
use candid::Nat;
use ic_cdk::query;

#[query]
fn remove_liquidity_amounts(token_0: String, token_1: String, remove_lp_token_amount: Nat) -> Result<RemoveLiquidityAmountsReply, String> {
    // Pool
    let pool = handlers::get_by_tokens(token_0, token_1)?;
    let symbol = pool.name();
    // Token0
    let token_0 = pool.token_0();
    let address_0 = pool
    .token_0()
    .canister_id()
    .map(|p| p.to_text())
    .ok_or_else(|| "Token0 has no canister_id".to_string())?;
    let symbol_0 = token_0.symbol();
    // Token1
    let token_1 = pool.token_1();
    let address_1 = pool
    .token_1()
    .canister_id()
    .map(|p| p.to_text())
    .ok_or_else(|| "Token1 has no canister_id".to_string())?;    let symbol_1 = token_1.symbol();

    let (amount_0, lp_fee_0, amount_1, lp_fee_1) = calculate_amounts(&pool, &remove_lp_token_amount)?;

    Ok(RemoveLiquidityAmountsReply {
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
    })
}