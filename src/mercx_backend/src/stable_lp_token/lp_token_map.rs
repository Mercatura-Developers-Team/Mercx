use candid::Nat;
use super::stable_lp_token::{StableLPToken};
use crate::stable_memory::LP_TOKEN_MAP;
use crate::kyc::kyc_id::get_user_by_caller;
use crate::helpers::math_helpers::{nat_add,nat_zero};
use crate::stable_lp_token::stable_lp_token::StableLPTokenId;
use crate::stable_mercx_settings::mercx_settings_map;

/// get lp_token of the caller
 #[ic_cdk::query]
pub async fn get_by_token_id(token_id: u32) -> Option<StableLPToken> {
    let user = get_user_by_caller().await.ok().flatten()?;
    let user_id: u32 = user.user_id;
    get_by_token_id_by_user_id(token_id, user_id)
}

/// get lp_token for specific user and token
pub fn get_by_token_id_by_user_id(token_id: u32, user_id: u32) -> Option<StableLPToken> {
    LP_TOKEN_MAP.with(|m| {
        m.borrow().iter().find_map(|(_, v)| {
            if v.user_id == user_id && v.token_id == token_id {
                return Some(v);
            }
            None
        })
    })
} 

/// get lp_token for specific user
pub fn get_by_user_id(user_id: u32) -> Vec<StableLPToken> {
    LP_TOKEN_MAP.with(|m| {
        m.borrow()
            .iter()
            .filter_map(|(_, v)| if v.user_id == user_id { Some(v) } else { None })
            .collect()
    })
}

pub fn get_total_supply(token_id: u32) -> Nat {
    LP_TOKEN_MAP.with(|m| {
        m.borrow()
            .iter()
            .filter_map(|(_, v)| if v.token_id == token_id { Some(v.amount) } else { None })
            .fold(nat_zero(), |acc, x| nat_add(&acc, &x))
    })
}   


pub fn insert(lp_token: &StableLPToken) -> Result<u64, String> {
    let insert_lp_token = LP_TOKEN_MAP.with(|m| {
        let mut map = m.borrow_mut();
        let lp_token_id = mercx_settings_map::inc_lp_token_map_idx();
        let insert_lp_token = StableLPToken {
            lp_token_id,
            ..lp_token.clone()
        };
        map.insert(StableLPTokenId(lp_token_id), insert_lp_token.clone());
        insert_lp_token
    });

    Ok(insert_lp_token.lp_token_id)
}


pub fn update(lp_token: &StableLPToken) {
    LP_TOKEN_MAP.with(|m| m.borrow_mut().insert(StableLPTokenId(lp_token.lp_token_id), lp_token.clone()));

}