pub mod ic;
mod stable_memory;
use crate::ic::general::{GetTransactions, TransactionResponse, TransferArgs};
pub mod xrc_mock;
pub use xrc_mock::get_icp_rate;
pub mod pool;
use crate::pool::add_pool_arg::AddPoolArgs;
use crate::pool::add_pool_reply::AddPoolReply;
use crate::pool::stable_pool::StablePool;

pub mod token;
use crate::token::stable_token::StableToken;

pub mod swap;

pub mod transfers;

pub mod add_liquidity;
use crate::add_liquidity::add_liquidity_amounts_reply::AddLiquidityAmountsReply;
use crate::add_liquidity::add_liquidity_args::AddLiquidityArgs;
use crate::add_liquidity::add_liquidity_reply::AddLiquidityReply;

pub mod helpers;
pub mod stable_mercx_settings;
use candid::Nat;
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::BlockIndex;
use icrc_ledger_types::icrc1::transfer::NumTokens;
ic_cdk::export_candid!();
