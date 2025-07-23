use candid::{CandidType,Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use crate::stable_memory::{TOKENS,POOLS,TRANSFERS};
use icrc_ledger_types::icrc1::account::Account;
use crate::ic::canister_address::MERCX_BACKEND;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct StableMercxSettings {
    pub mercx_backend: Account,
    pub token_map_idx: u32,    // counter for TOKEN_MAP
    pub pool_map_idx: u32,     // counter for POOL_MAP
    pub default_lp_fee_bps: u8,
    pub default_mercx_fee_bps: u8,
    pub transfer_map_idx: u64, // counter for TRANSFER_MAP
    pub transfer_expiry_nanosecs: u64,
   
}

impl Default for StableMercxSettings {
    fn default() -> Self {
        let token_map_idx = TOKENS.with(|m| m.borrow().iter().map(|(k, _)| k.0).max().unwrap_or(0));
        let pool_map_idx = POOLS.with(|m| m.borrow().iter().map(|(k, _)| k.0).max().unwrap_or(0));
        let transfer_map_idx = TRANSFERS.with(|m| m.borrow().iter().map(|(k, _)| k.0).max().unwrap_or(0));
    

        Self {
            mercx_backend: Account::from(Principal::from_text(MERCX_BACKEND).unwrap()),
            token_map_idx,
            pool_map_idx,
            default_lp_fee_bps: 30,
            default_mercx_fee_bps: 0,
            transfer_map_idx,
            transfer_expiry_nanosecs: 3_600_000_000_000, // 1 hour (nano seconds)
        }
    }
}

impl Storable for StableMercxSettings {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap_or_default()
    }

    const BOUND: Bound = Bound::Unbounded;
}