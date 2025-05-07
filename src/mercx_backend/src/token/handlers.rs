use crate::token::stable_token::{StableToken, StableTokenId};
use crate::stable_memory::TOKENS;
use candid::Principal;

pub fn get_by_token_id(token_id: u32) -> Option<StableToken> {
    TOKENS.with(|m| m.borrow().get(&StableTokenId(token_id)))
}


// /// return all tokens
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
