use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Arguments for adding a token.
#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct AddTokenArgs {
    pub canister_id: Principal,
}

