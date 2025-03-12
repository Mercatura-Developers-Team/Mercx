use candid::Principal;
use ic_cdk::api::caller;
use ic_cdk_macros::query;

use crate::errors::general::GeneralError;
use crate::store::{USERS, USERNAMES};
use crate::models::user::{User, UsernameAvailabilityResponse, UserPrincipalInfo};
use crate::validations::user::validate_username;

#[query]
pub fn is_admin() -> Result<(), String> {
    let caller_principal = caller();
    let is_admin = USERS.with(|users| {
        users.borrow()
            .get(&caller_principal)
            .map(|user| user.admin)
            .unwrap_or(false) // Assume not admin if not found
    });

    if is_admin {
        Ok(())
    } else {
        Err("Unauthorized: Only admins can perform this operation.".to_string())
    }
}


#[query]
pub fn whoami() -> Principal {
    caller()
}

#[query]
pub fn get_user(principal: Principal) -> Result<User, String> {
    is_admin()?; // Admin check
    // if principal == Principal::anonymous() {
    //     return Err(GeneralError::AnonymousNotAllowed.to_string())
    // }

    USERS.with(|users| {
        users.borrow()
            .get(&principal)
            .ok_or_else(|| GeneralError::NotFound("User".to_string()).to_string())
    })
}

#[query]
pub fn get_current_user() -> Result<User, String> {
    //is_admin()?; // Admin check
    get_user(caller())
}

#[query]
pub fn check_username_availability(username: String) -> Result<UsernameAvailabilityResponse, String> {
    //is_admin()?; // Admin check
    let username = username.trim().to_lowercase();

    if let Err(error) = validate_username(&username) {
        return Ok(UsernameAvailabilityResponse {
            username: username.clone(),
            available: false,
            message: error.to_string(),
        });
    }

    USERNAMES.with(|usernames| {
        let available = !usernames.borrow().contains_key(&username);
        let message = if available {
            "Username is available".to_string()
        } else {
            "Username is already taken".to_string()
        };

        Ok(UsernameAvailabilityResponse {
            username,
            available,
            message,
        })
    })
}

#[query]
pub fn get_all_users() -> Result<Vec<UserPrincipalInfo>, String> {
    
   // is_admin()?; // Admin check
    USERS.with(|users| {
        users.borrow().iter().map(|(principal, user)| {
            if user.email.is_empty() || user.phone_number.is_empty() || user.full_name.is_empty() {
                Err("User data integrity error: Required fields are missing".to_string())
            } else {
                Ok(UserPrincipalInfo {
                    principal,
                    username: user.username.clone(),
                    email: user.email.clone(),
                    phone_number: user.phone_number.clone(),
                    full_name: user.full_name.clone(),
                    refered_by:user.refered_by.clone(),
                })
            }
        }).collect()
    })
}


#[query]
pub fn check_kyc_status(principal: Principal) -> Result<bool, String> {
   
    USERS.with(|users| {
        users.borrow()
            .get(&principal)
            .map(|user| user.kyc_status)
            .ok_or_else(|| "User not found.".to_string())
    })
}

#[query]
pub fn has_username_for_principal(principal: Principal) -> bool {
    
    USERS.with(|users| {
        users.borrow().contains_key(&principal)
    })
}

#[query]
pub fn get_username_by_principal(principal: Principal) -> Result<String, String> {
    USERS.with(|users| {
        users.borrow()
            .get(&principal)
            .map(|user| user.username.clone())
            .ok_or_else(|| "User not found.".to_string())
    
})
}

#[query]
pub fn get_principal_by_username(username: String) -> Result<Principal, String> {
    USERNAMES.with(|usernames| {
        usernames.borrow()
            .get(&username)
            .map(|principal| principal.clone())
            .ok_or_else(|| "Username not found please try a valid username ".to_string())
    })
}

