mod stable_memory;
pub mod general;
pub use general::*;

pub mod xrc_mock;
pub use xrc_mock::get_icp_rate;

pub mod pool;
use crate::pool::add_pool_arg::{AddPoolArgs, AddPoolReply};
use crate::pool::stable_pool::StablePool;

pub mod swap;


use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::NumTokens;
use icrc_ledger_types::icrc1::transfer::BlockIndex;
use candid::Nat;

ic_cdk::export_candid!();
