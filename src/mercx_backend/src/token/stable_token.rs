use crate::ic::general::get_supported_standards;
use crate::ic::general::{get_decimals, get_fee, get_name, get_symbol};
use candid::{CandidType, Decode, Encode, Nat, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
#[derive(CandidType, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StableTokenId(pub u32);

impl Storable for StableTokenId {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let val = Decode!(bytes.as_ref(), u32)
            .unwrap_or_else(|e| ic_cdk::trap(&format!("❌ Failed to decode StableTokenId: {}", e)));
        StableTokenId(val)
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 20,
        is_fixed_size: false,
    };
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct StableToken {
    pub token_id: u32,
    pub name: String,
    pub symbol: String,
    pub canister_id: Principal,
    pub decimals: u8,
    pub fee: Nat,
    pub icrc1: bool,
    pub icrc2: bool,
    pub icrc3: bool,
}

impl StableToken {
    pub async fn new(canister_id: Principal) -> Result<Self, String> {
        let name = get_name(canister_id)
            .await
            .map_err(|e| format!("get_name failed: {}", e))?;
        let symbol = get_symbol(canister_id)
            .await
            .map_err(|e| format!("get_symbol failed: {}", e))?;
        let decimals = get_decimals(canister_id)
            .await
            .map_err(|e| format!("get_decimals failed: {}", e))?;
        let fee = get_fee(canister_id)
            .await
            .map_err(|e| format!("get_fee failed: {}", e))?;
        let (icrc1, icrc2, icrc3) = match get_supported_standards(canister_id).await {
            Ok(supported_standards) => {
                let icrc1 = supported_standards
                    .iter()
                    .any(|standard| standard.name == "ICRC-1");
                let icrc2 = supported_standards
                    .iter()
                    .any(|standard| standard.name == "ICRC-2");
                let icrc3 = supported_standards
                    .iter()
                    .any(|standard| standard.name == "ICRC-3");
                (icrc1, icrc2, icrc3)
            }
            Err(_) => (true, false, false), // should at least support ICRC-1 if it made it this far
        };
        Ok(Self {
            token_id: 0,
            name,
            symbol,
            canister_id,
            decimals,
            fee,
            icrc1,
            icrc2,
            icrc3,
        })
    }

    pub fn token_id(&self) -> u32 {
        self.token_id
    }

    // fn name(&self) -> String {
    //     self.name.to_string()
    // }

    pub fn symbol(&self) -> String {
        self.symbol.to_string()
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn canister_id(&self) -> Option<&Principal> {
        Some(&self.canister_id)
    }

  pub fn is_icrc1(&self) -> bool {
        self.icrc1
    }

    pub fn is_icrc2(&self) -> bool {
        self.icrc2
    }

    pub fn fee(&self) -> Nat {
    
        self.fee.clone()
 }

}

pub fn symbol(token_0: &StableToken, token_1: &StableToken) -> String {
    format!("{}_{}", token_0.symbol(), token_1.symbol())
}


impl Storable for StableToken {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), StableToken)
            .unwrap_or_else(|e| ic_cdk::trap(&format!("❌ Failed to decode StableToken: {}", e)))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 2048, // adjust depending on your expected size
        is_fixed_size: false,
    };
}
