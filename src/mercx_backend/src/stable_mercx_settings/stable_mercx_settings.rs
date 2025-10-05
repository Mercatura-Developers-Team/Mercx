use candid::{CandidType,Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use crate::stable_memory::{TOKENS,POOLS,TRANSFERS,LPMETADATA};
use icrc_ledger_types::icrc1::account::Account;
use crate::ic::canister_address::MERCX_BACKEND;
use crate::stable_memory::LP_TOKEN_MAP;
use crate::stable_memory::ANALYTICS_DATA;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct StableMercxSettings {
    pub mercx_backend: Account,
    pub token_map_idx: u32,    // counter for TOKEN_MAP
    pub lp_metadata_map_idx: u32,    // counter for LP Metadata
    pub pool_map_idx: u32,     // counter for POOL_MAP
    pub default_lp_fee_bps: u8,
    pub default_mercx_fee_bps: u8,
    pub transfer_map_idx: u64, // counter for TRANSFER_MAP
    pub transfer_expiry_nanosecs: u64,
    pub default_max_slippage: f64,
    pub lp_token_map_idx: u64, // counter for LP_TOKEN_MAP
    pub analytics_map_idx: u32, // NEW: counter for ANALYTICS_DATA

}

impl Default for StableMercxSettings {
    fn default() -> Self {
        let token_map_idx = TOKENS.with(|m| m.borrow().iter().map(|(k, _)| k.0).max().unwrap_or(0));
        let lp_metadata_map_idx = LPMETADATA.with(|m| m.borrow().iter().map(|(k, _)| k.0).max().unwrap_or(0));
        let pool_map_idx = POOLS.with(|m| m.borrow().iter().map(|(k, _)| k.0).max().unwrap_or(0));
        let transfer_map_idx = TRANSFERS.with(|m| m.borrow().iter().map(|(k, _)| k.0).max().unwrap_or(0));
        let lp_token_map_idx = LP_TOKEN_MAP.with(|m| m.borrow().iter().map(|(k, _)| k.0).max().unwrap_or(0));
        let analytics_map_idx = ANALYTICS_DATA.with(|m| m.borrow().iter().map(|(k, _)| k.0).max().unwrap_or(0)); // NEW


        Self {
            mercx_backend: Account::from(Principal::from_text(MERCX_BACKEND).unwrap()),
            token_map_idx,
            lp_metadata_map_idx,
            pool_map_idx,
            default_lp_fee_bps: 30,
            default_mercx_fee_bps: 0,
            transfer_map_idx,
            transfer_expiry_nanosecs: 3_600_000_000_000, // 1 hour (nano seconds)
            default_max_slippage: 2.0_f64,
            lp_token_map_idx,
            analytics_map_idx,
        }
    }
}

impl Storable for StableMercxSettings {
    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<'_, [u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap_or_default()
    }

    const BOUND: Bound = Bound::Unbounded;
}