use ic_cdk::call;
use crate::ic::id::caller;
use candid::{CandidType,Principal};
use serde::{Deserialize, Serialize};
use crate::ic::canister_address::KYC_CANISTER_ID;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: u32,
    pub principal: Principal,
    pub username: String,
    pub full_name: String,         // ✅ Added full name
    pub email: String,             // ✅ Added email
    pub phone_number: String,      // ✅ Added phone number
    pub name: String,
    pub avatar: String,
    pub librarian: bool,
    pub admin: bool, // Added admin flag
    pub refered_by:Option<String>,
    pub kyc_status: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

// Replace `your_kyc_canister_id` with actual canister ID or pass it dynamically
//const KYC_CANISTER_ID: &str = "ajuq4-ruaaa-aaaaa-qaaga-cai"; // example
#[ic_cdk::update]
pub async fn get_user_by_caller() -> Result<Option<User>, String> {
    let caller_principal = caller();

    let (result,): (Result<Option<User>, String>,) = call(
        Principal::from_text(KYC_CANISTER_ID).unwrap(),
        "get_user_by_principal",
        (caller_principal,),
    )
    .await
    .map_err(|e| format!("Call failed: {:?}", e))?; // fixed formatting

    result
}

