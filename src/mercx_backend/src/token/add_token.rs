use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::token::stable_token::StableTokenId;
use crate::stable_memory::TOKENS;
use crate::StableToken;

use crate::stable_mercx_settings::mercx_settings_map::inc_token_map_idx;

use crate::token::handlers::exists_by_canister_id;

/// Arguments for adding a token.
#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct AddTokenArgs {
    pub canister_id: Principal,
}

#[ic_cdk::update]
pub async fn add_token(canister_id: Principal) -> Result<StableToken, String> {

     // 🔒 Check if the token already exists by canister ID
     if exists_by_canister_id(&canister_id) {
        return Err(format!("Token with canister_id {} already exists", canister_id));
    }

    let mut token = StableToken::new(canister_id).await?;

    let token_id = inc_token_map_idx();
    token.token_id = token_id;

    TOKENS.with(|tokens| {
        tokens.borrow_mut().insert(StableTokenId(token_id), token.clone());
    });

    Ok(token)
}
