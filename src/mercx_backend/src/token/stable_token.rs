use candid::{CandidType, Nat, Principal,Decode,Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{get_decimals, get_fee, get_name, get_symbol};
#[derive(CandidType, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StableTokenId(pub u32);

impl Storable for StableTokenId {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

// impl Storable for StableTokenId {
//     fn to_bytes(&self) -> Cow<[u8]> {
//         Cow::Owned(Encode!(&self.0).unwrap())
//     }

//     fn from_bytes(bytes: Cow<[u8]>) -> Self {
//         Self(Decode!(bytes.as_ref(), u32).unwrap()) // 👈 decode into u32 not Principal
//     }

//     const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
//         max_size: 29,
//         is_fixed_size: false,
//     };
// }

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct StableToken {
    pub token_id: u32,
    pub name: String,
    pub symbol: String,
    pub canister_id: Principal,
    pub decimals: u8,
    pub fee: Nat,
    #[serde(default = "false_bool")]
    pub is_removed: bool,
}

fn false_bool() -> bool {
    false
}

impl StableToken {
    pub async fn new(canister_id: Principal) -> Result<Self, String> {
        let name = get_name(canister_id).await?;
        let symbol = get_symbol(canister_id).await?;
        let decimals = get_decimals(canister_id).await?;
        let fee = get_fee(canister_id).await?;
        Ok(Self {
            token_id: 0,
            name,
            symbol,
            canister_id,
            decimals,
            fee,
            is_removed: false,
        })
    }

    fn token_id(&self) -> u32 {
        self.token_id
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn symbol(&self) -> String {
        self.symbol.to_string()
    }

    fn canister_id(&self) -> Option<&Principal> {
        Some(&self.canister_id)
    }
}

pub fn symbol(token_0: &StableToken, token_1: &StableToken) -> String {
    format!("{}_{}", token_0.symbol(), token_1.symbol())
}

impl Storable for StableToken {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
