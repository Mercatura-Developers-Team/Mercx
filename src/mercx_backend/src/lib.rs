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
use crate::swap::swap_reply::SwapReply;
use crate::swap::swap_args::SwapArgs;

pub mod swap_amounts;
use crate::swap_amounts::swap_amounts_reply::SwapAmountsReply;

pub mod transfers;

pub mod add_liquidity;
use crate::add_liquidity::add_liquidity_amounts_reply::AddLiquidityAmountsReply;
use crate::add_liquidity::add_liquidity_args::AddLiquidityArgs;
use crate::add_liquidity::add_liquidity_reply::AddLiquidityReply;

pub mod lp_metadata;

pub mod stable_lp_token;
use crate::stable_lp_token::stable_lp_token::StableLPToken;
use crate::lp_metadata::stable_lp_metadata::LPToken;

pub mod kyc;
use crate::kyc::kyc_id::User;

pub mod remove_liquidity;
use crate::remove_liquidity::remove_liquidity_args::RemoveLiquidityArgs;
use crate::remove_liquidity::remove_liquidity_reply::RemoveLiquidityReply;
use crate::stable_lp_token::lp_token_map::LpTokenInfo;

pub mod remove_liquidity_amounts;
use crate::remove_liquidity_amounts::remove_liquidity_amounts_reply::RemoveLiquidityAmountsReply;

pub mod helpers;
pub mod stable_mercx_settings;
use candid::Nat;
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::BlockIndex;
use icrc_ledger_types::icrc1::transfer::NumTokens;
ic_cdk::export_candid!();
