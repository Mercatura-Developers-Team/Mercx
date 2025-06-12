use candid::{CandidType, Nat};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use crate::token::stable_token::StableToken;
use crate::token::handlers;
use crate::helpers::math_helpers::{price_rounded,nat_to_bigint,nat_to_decimal_precision,nat_is_zero,nat_add,nat_zero};
use num::BigRational;

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
        }
    }
    //from stable token
    pub fn token_0(&self) -> StableToken {
       handlers::get_by_token_id(self.token_id_0).unwrap()
    }

    pub fn token_1(&self) -> StableToken {
        handlers::get_by_token_id(self.token_id_1).unwrap()
     }

    pub fn symbol_0(&self) -> String {
        self.token_0().symbol().to_string()
    }

    pub fn symbol_1(&self) -> String {
        self.token_1().symbol().to_string()
    }
    pub fn name(&self) -> String {
        format!("{}_{} Liquidity Pool", self.symbol_0(), self.symbol_1())
    }
    pub fn canister_id_0(&self) -> String {
        self.token_0().canister_id().expect("Canister ID missing").to_string()    }

   pub fn canister_id_1(&self) -> String {
            self.token_1().canister_id().expect("Canister ID missing").to_string()    }

    pub fn get_price(&self) -> Option<BigRational> {
        let reserve_0 = nat_add(&self.balance_0, &self.lp_fee_0);
        let reserve_1 = nat_add(&self.balance_1, &self.lp_fee_1);
        if nat_is_zero(&reserve_0) {
            None?
        }

        let token_0 = self.token_0();
        let token_1 = self.token_1();
        let max_decimals = std::cmp::max(token_0.decimals(), token_1.decimals());
        let reserve_0 = nat_to_bigint(&nat_to_decimal_precision(&reserve_0, token_0.decimals(), max_decimals));
        let reserve_1 = nat_to_bigint(&nat_to_decimal_precision(&reserve_1, token_1.decimals(), max_decimals));

        Some(BigRational::new(reserve_1, reserve_0))
    }


    //#[ic_cdk::query]
    pub fn get_price_as_f64(&self) -> Option<f64> {
        price_rounded(&self.get_price()?)
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