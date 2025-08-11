use candid::CandidType;
use serde::{Deserialize, Serialize};
use ic_stable_structures::{storable::Bound, Storable};


use crate::pool::handlers;
use crate::StableToken;
use crate::StablePool;
use crate::token::stable_token::symbol;


#[derive(CandidType, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StableLpMetadataId(pub u32);

impl Storable for StableLpMetadataId {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

pub const LP_DECIMALS: u8 = 8; // LP token decimal

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct LPToken {
    pub token_id: u32,
    pub symbol: String,
 //   pub address: String, // unique identifier for the token
    pub decimals: u8,
}

impl LPToken {
    pub fn new(token_0: &StableToken, token_1: &StableToken) -> Self {

        let symbol = symbol(token_0, token_1);
        // LP token's address is the combination of token_0's token_id and token_1's token_id
        // which is unique making it a unique identifier for the LP token
       // let address = token::address(token_0, token_1);
        Self {
            token_id: 0,
            symbol,
           // address,
            decimals: LP_DECIMALS,
        }
    }

    pub fn name(&self) -> String {
        format!("{} LP Token", self.symbol)
    }

   pub fn token_id(&self) -> u32 {
           self.token_id
   }

    /// Pool that the LP token belongs to
    pub fn pool_of(&self) -> Option<StablePool> {
        handlers::get_by_lp_token_id(self.token_id)
    }
}


impl Storable for LPToken {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}