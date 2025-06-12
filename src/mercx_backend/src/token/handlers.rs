use crate::token::stable_token::{StableToken, StableTokenId};
use crate::stable_memory::TOKENS;
use candid::Principal;
use crate::stable_mercx_settings::mercx_settings_map::reset_token_map_idx;
use crate::stable_memory::POOLS;
pub fn get_by_token_id(token_id: u32) -> Option<StableToken> {
    TOKENS.with(|m| m.borrow().get(&StableTokenId(token_id)))
}

//return all tokens
#[ic_cdk::query]
pub fn get_all_tokens() -> Vec<StableToken> {
    TOKENS.with(|m| {
        m.borrow()
            .iter()
            .map(|(_, v)| v.clone()) 
            .collect()
    })
}

pub fn exists_by_canister_id(canister_id: &Principal) -> bool {
    TOKENS.with(|m| {
        m.borrow()
            .iter()
            .any(|(_, token)| token.canister_id == *canister_id)
    })
}

pub fn get_by_symbol(symbol: &str) -> Result<StableToken, String> {
    let query = symbol.to_lowercase();

    TOKENS.with(|tokens| {
        tokens
            .borrow()
            .iter()
            .find(|(_, token)| token.symbol.to_lowercase() == query)
            .map(|(_, token)| token.clone())
    })
    .ok_or_else(|| format!("Token with symbol '{}' not found", symbol))
}


pub fn get_by_canister_id(canister_id: &Principal) -> Option<StableToken> {
    TOKENS.with(|tokens| {
        tokens
            .borrow()
            .iter()
            .find_map(|(_, token)| {
                if &token.canister_id == canister_id {
                    Some(token.clone())
                } else {
                    None
                }
            })
    })
}


pub fn get_by_token(token: &str) -> Result<StableToken, String> {
    // Try by symbol
    if let Ok(token) = get_by_symbol(token) {
        return Ok(token);
    }

    // Try by canister ID string
    if let Ok(principal) = Principal::from_text(token) {
        if let Some(token) = get_by_canister_id(&principal) {
            return Ok(token);
        }
    }

    Err(format!(
        "Token '{}' not found symbols/canisters exist",
        token
    ))
}

#[cfg(not(feature = "prod"))]
#[ic_cdk::update]
fn reset_tokens() -> Result<String, String> {
    TOKENS.with(|tokens| {
        tokens.borrow_mut().clear_new(); // `clear_new()` btmsh kolo remove law hanmsh haga specific
    });

    reset_token_map_idx();

    Ok("✅ Tokens memory cleared".to_string())
}

//#[ic_cdk::update]
// fn delete_token_by_id(canister_id: Principal) -> Result<String, String> {
//     let token_id = StableTokenId(id);
//     let removed = TOKENS.with(|tokens| {
//         tokens.borrow_mut().remove(&token_id)
//     });

//     match removed {
//         Some(_) => Ok(format!("✅ Token with ID {} deleted.", id)),
//         None => Err(format!("❌ Token with ID {} not found.", id)),
//     }
// }


#[ic_cdk::update]
fn delete_token_by_canister_id(canister_id: Principal) -> Result<String, String> {
    // Step 1: Find token ID from TOKENS map
    let maybe_token_id = TOKENS.with(|tokens| {
        let tokens = tokens.borrow();
        tokens.iter().find_map(|(id, token)| {
            if token.canister_id == canister_id {
                Some(id.clone())
            } else {
                None
            }
        })
    });

    let token_id = match maybe_token_id {
        Some(id) => id,
        None => return Err("❌ Token not found.".to_string()),
    };

    // Step 2: Check if token is used in any pool
    let is_used_in_pool = POOLS.with(|pools| {
        pools.borrow().iter().any(|(_, pool)| {
            pool.token_id_0 == token_id.0 || pool.token_id_1 == token_id.0
        })
    });

    if is_used_in_pool {
        return Err("Cannot delete token it is currently used in a pool.".to_string());
    }

    // Step 3: Delete token if not in use
    TOKENS.with(|tokens| {
        tokens.borrow_mut().remove(&token_id);
    });

    Ok(format!("✅ Token with canister ID {} deleted.", canister_id))
}
