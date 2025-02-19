use candid::{CandidType, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

/// Represents a user's profile in the system
///
/// # Example
/// ```rust
/// let user = User {
///     principal: caller,
///     username: "bookworm",
///     name: Some("John Doe"),
///     avatar: Some("https://example.com/avatar.jpg"),
///     librarian: false,
///     created_at: 1234567890,
///     updated_at: Some(1234567890),
/// };
/// ```
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub principal: Principal,
    pub username: String,
    pub full_name: String,         // ✅ Added full name
    pub email: String,             // ✅ Added email
    pub phone_number: String,      // ✅ Added phone number
    pub name: String,
    pub avatar: String,
    pub librarian: bool,
    pub admin: bool, // Added admin flag
    pub kyc_status: bool,
    pub referred_by: Option<String>, // New optional field
    pub created_at: u64,
    pub updated_at: u64,
}

impl User {
    pub fn new(principal: Principal, username: String, full_name: String, email: String, phone_number: String,        referred_by: Option<String>, // Add this parameter
    ) -> Self {
        let now = time();
        Self {
            principal,
            username,
            full_name,
            email,
            phone_number,
            name: String::new(),
            avatar: String::new(),
            librarian: false,
             admin: false, // Added admin flag
            kyc_status: false,
            referred_by, // Set the referred_by field
            created_at: now,
            updated_at: now,
        }
    }
}


/// Request payload for user signup
///
/// # Example
/// ```rust
/// let request = SignupRequest {
///     username: "bookworm".to_string(),
/// };
/// ```
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SignupRequest {
    pub username: String,
    pub full_name: String,        // ✅ Full Name
    pub email: String,            // ✅ Email
    pub phone_number: String,     // ✅ Phone Number
    pub referred_by: Option<String>, // New optional field

}

/// Request payload for updating user profile
///
/// # Example
/// ```rust
/// let request = UpdateUserRequest {
///     name: Some("John Doe".to_string()),
///     avatar: Some("https://example.com/avatar.jpg".to_string()),
/// };
/// ```
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub avatar: Option<String>,
   
}

/// Response for username availability check
///
/// # Example
/// ```rust
/// let response = UsernameAvailabilityResponse {
///     username: "bookworm".to_string(),
///     available: true,
///     message: Some("Username is available".to_string()),
/// };
/// ```

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct UsernameAvailabilityResponse {
    pub username: String,
    pub available: bool,
    pub message: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserPrincipalInfo {
    pub principal: Principal,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
    pub referred_by: Option<String>,
}