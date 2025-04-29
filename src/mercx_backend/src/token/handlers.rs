use crate::token::stable_token::{StableToken, StableTokenId};
use crate::stable_memory::TOKENS;

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
