use candid::{CandidType, Nat};
use ic_stable_structures::{storable::Bound, Storable};
//use num::BigRational;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StablePoolId(pub u32);

impl Storable for StablePoolId {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct StablePool {
    pub pool_id: u32,
    pub token_id_0: u32,
    pub balance_0: Nat, //Amount of token 0 in the pool
    pub lp_fee_0: Nat, //Fees collected from swaps for each token, to distribute to LPs //Token amount
    pub mercx_fee_0: Nat, // MercX's share of the LP fee //Token amount
    pub token_id_1: u32,
    pub balance_1: Nat,
    pub lp_fee_1: Nat,
    pub mercx_fee_1: Nat,  // Mercx's share of the LP fee
    pub lp_fee_bps: u8,   // LP's fee in basis points //distributed to the liquidity providers //Percentage (%)(felswap)
    pub kong_fee_bps: u8, // Kong's fee in basis points //Percentage (%) (fel swap)
    pub lp_token_id: u32, // token id of the LP token
    #[serde(default = "false_bool")]
    pub is_removed: bool,
}

fn false_bool() -> bool {
    false
}

pub fn nat_zero() -> Nat {
    Nat::from(0_u128)
}

impl StablePool {
    pub fn new(token_id_0: u32, token_id_1: u32, lp_fee_bps: u8, kong_fee_bps: u8, lp_token_id: u32) -> Self {
        Self {
            pool_id: 0,
            token_id_0,
            balance_0: Nat::from(0_u128),
            lp_fee_0: nat_zero(),
            mercx_fee_0: nat_zero(),
            token_id_1,
            balance_1: nat_zero(),
            lp_fee_1: nat_zero(),
            mercx_fee_1: nat_zero(),
            lp_fee_bps,
            kong_fee_bps,
            lp_token_id,
            is_removed: false,
        }
    }


}

impl Storable for StablePool {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}