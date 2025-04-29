use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::token::stable_token::StableTokenId;
use crate::stable_memory::TOKENS;
use crate::StableToken;

use crate::stable_mercx_settings::increment_ids::inc_token_map_idx;

/// Arguments for adding a token.
#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct AddTokenArgs {
    pub canister_id: Principal,
}

#[ic_cdk::update]
pub async fn add_token(canister_id: Principal) -> Result<StableToken, String> {
    let mut token = StableToken::new(canister_id).await?;

    let token_id = inc_token_map_idx();
    token.token_id = token_id;

    TOKENS.with(|tokens| {
        tokens.borrow_mut().insert(StableTokenId(token_id), token.clone());
    });

    Ok(token)
}
