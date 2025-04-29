use crate::token::stable_token::{StableToken, StableTokenId};
use crate::stable_memory::TOKENS;

pub fn get_by_token_id(token_id: u32) -> Option<StableToken> {
    TOKENS.with(|m| m.borrow().get(&StableTokenId(token_id)))
}


/// return all tokens
pub fn get() -> Vec<StableToken> {
    TOKENS.with(|m| {
        m.borrow()
            .iter()
            .filter_map(|(_, v)| if !v.is_removed() { Some(v) } else { None })
            .collect()
    })
}