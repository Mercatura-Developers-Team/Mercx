use candid::Principal;
use candid::Nat;
use super::stable_lp_token::{StableLPToken};
use crate::stable_memory::LP_TOKEN_MAP;
use crate::kyc::kyc_id::get_user_by_caller;
use crate::helpers::math_helpers::{nat_add,nat_zero};
use crate::stable_lp_token::stable_lp_token::StableLPTokenId;
use crate::stable_mercx_settings::mercx_settings_map;
use crate::stable_mercx_settings::mercx_settings_map::reset_lp_map_idx;
use crate::stable_memory::LPMETADATA;
use crate::lp_metadata::stable_lp_metadata::StableLpMetadataId;
use candid::CandidType;use serde::Serialize; use candid::Deserialize;

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
#[ic_cdk::query]
pub fn get_by_token_id_by_principal(token_id: u32, principal: Principal) -> Option<StableLPToken> {
    LP_TOKEN_MAP.with(|m| {
        m.borrow().iter().find_map(|(_, v)| {
            if v.principal == principal && v.token_id == token_id {
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

#[ic_cdk::query]
pub fn get_by_principal(principal: Principal) -> Vec<StableLPToken> {
    LP_TOKEN_MAP.with(|m| {
        m.borrow()
            .iter()
            .filter_map(|(_, v)| if v.principal == principal { Some(v) } else { None })
            .collect()
    })
}

// #[ic_cdk::query]
// pub fn get_lp_symbols_by_principal(principal: Principal) -> Vec<String> {
//     // First collect the lp token_ids for this principal
//     let ids: Vec<u32> = LP_TOKEN_MAP.with(|m| {
//         m.borrow()
//             .iter()
//             .filter_map(|(_, v)| {
//                 if v.principal == principal {
//                     Some(v.token_id) // this is a u32
//                 } else {
//                     None
//                 }
//             })
//             .collect()
//     });

//     // Then map those ids to symbols via LPMETADATA, whose key is StableLpMetadataId(u32)
//     let mut set = HashSet::new();
//     LPMETADATA.with(|meta| {
//         let meta = meta.borrow();
//         for id in ids {
//             if let Some(t) = meta.get(&StableLpMetadataId(id)) { // <-- wrap id in key type
//                 set.insert(t.symbol.clone());
//             }
//         }
//     });

//     // Return a stable order if you like
//     let mut out: Vec<String> = set.into_iter().collect();
//     out.sort();
//     out
// }

#[derive(CandidType, Serialize, Deserialize)]
pub struct LpTokenInfo {
    pub symbol: String,
    pub amount: Nat,
}

#[ic_cdk::query]
pub fn get_lp_tokens_by_principal(principal: Principal) -> Vec<LpTokenInfo> {
    let mut results: Vec<LpTokenInfo> = vec![];

    // Step 1: Collect all LP tokens owned by the principal
    let tokens = LP_TOKEN_MAP.with(|map| {
        map.borrow()
            .iter()
            .filter_map(|(_, token)| {
                if token.principal == principal {
                    Some((token.token_id, token.amount.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<(u32, Nat)>>()
    });

    // Step 2: Lookup symbols from metadata and pair with amount
    LPMETADATA.with(|meta| {
        let map = meta.borrow();
        for (token_id, amount) in tokens {
            if let Some(metadata) = map.get(&StableLpMetadataId(token_id)) {
                results.push(LpTokenInfo {
                    symbol: metadata.symbol.clone(),
                    amount,
                });
            }
        }
    });

    results
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

#[cfg(not(feature = "prod"))]
#[ic_cdk::update]
fn reset_lp() -> Result<String, String> {
    LP_TOKEN_MAP.with(|tokens| {
        tokens.borrow_mut().clear_new(); // `clear_new()` btmsh kolo remove law hanmsh haga specific
    });

    reset_lp_map_idx();

    Ok("✅ lp memory cleared".to_string())
}