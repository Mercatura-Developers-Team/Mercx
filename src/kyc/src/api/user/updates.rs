use ic_cdk::api::{caller, time};
use ic_cdk_macros::update;
use candid::Principal;

use crate::errors::general::GeneralError;
use crate::errors::user::UserError;
use crate::store::{USERS, USERNAMES};
use crate::models::user::{SignupRequest, UpdateUserRequest, User};
use crate::validations::user::{validate_username, validate_name, validate_avatar_url};
use crate::user_queries::get_user;
use crate::user_queries::is_admin;

#[update]
pub fn signup(request: SignupRequest) -> Result<User, String> {

    let caller = caller();

    if caller == Principal::anonymous() {
        return Err(GeneralError::AnonymousNotAllowed.to_string());
    }

    let username = request.username.trim().to_lowercase();
    

    if let Err(err) = validate_username(&username) {
        return Err(err.to_string());
    }

    // Check if user exists
    USERS.with(|users| {
        if users.borrow().contains_key(&caller) {
            return Err(GeneralError::AlreadyExists("User with this principal".to_string()).to_string());
        }
        Ok::<(), String>(())
    })?;

    // Check if username is taken
    USERNAMES.with(|usernames| {
        if usernames.borrow().contains_key(&username) {
            return Err(UserError::UsernameTaken.to_string());
        }
        Ok::<(), String>(())
    })?;

    let valid_refered_by = if let Some(refered_by_username) = &request.refered_by {
        let refered_by_username = refered_by_username.trim().to_lowercase();

        USERNAMES.with(|usernames| {
            if usernames.borrow().contains_key(&refered_by_username) {
                Ok(Some(refered_by_username))
            } else {
                Err("Referred username does not exist.".to_string())
            }
        })?
    } else {
        None
    };

    let user = User::new(
        caller,
        username.clone(),
        request.full_name.clone(),
        request.email.clone(),
        request.phone_number.clone(),
        request.refered_by.clone(),
    );
    // Insert user data
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        users.insert(caller, user.clone());
        Ok::<(), String>(())
    })?;

    USERNAMES.with(|usernames| {
        let mut usernames = usernames.borrow_mut();
        usernames.insert(username.clone(), caller);
        Ok::<(), String>(())
    })?;

    Ok(user)
}

#[update]
pub fn update_profile(request: UpdateUserRequest) -> Result<User, String> {

    let caller = caller();

    if caller == Principal::anonymous() {
        return Err(GeneralError::AnonymousNotAllowed.to_string());
    }

    USERS.with(|users| {
        let mut users = users.borrow_mut();
        let mut user = users.get(&caller)
            .map(|user| user.clone())
            .ok_or_else(|| GeneralError::NotFound("User".to_string()).to_string())?;

        // Update name if provided
        if let Some(name) = request.name {
            let name = name.trim();
            if let Err(err) = validate_name(name) {
                return Err(err.to_string());
            }
            user.name = name.to_string();
        }

        // Update avatar if provided
        if let Some(avatar) = request.avatar {
            let avatar = avatar.trim();
            if let Err(err) = validate_avatar_url(avatar) {
                return Err(err.to_string());
            }
            user.avatar = avatar.to_string();
        }

        user.updated_at = time();
        users.insert(caller, user.clone());
        Ok(user)
    })
}

#[update]
pub fn upgrade_to_librarian() -> Result<User, String> {
    let caller = caller();

    if caller == Principal::anonymous() {
        return Err(GeneralError::AnonymousNotAllowed.to_string());
    }

    USERS.with(|users| {
        let mut users = users.borrow_mut();
        let mut user = users.get(&caller)
            .map(|user| user.clone())
            .ok_or_else(|| GeneralError::NotFound("User".to_string()).to_string())?;

        user.librarian = true;
        user.updated_at = time();
        users.insert(caller, user.clone());
        
        Ok(user)
    })
}

// ✅ KYC Verification (Admin-Only)
#[update]
pub fn verify_kyc(principal: Principal) -> Result<String, String> {
    is_admin()?; // Admin check
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        if let Some(mut user) = users.get(&principal) {
            user.kyc_status = true;

            // Re-insert the updated user into the map
            users.insert(principal, user);

            Ok("KYC verification successful.".to_string())
        } else {
            Err("User not found.".to_string())
        }
    })
}

#[update]
pub fn delete_user(principal: Principal) -> Result<String, String> {
    is_admin()?; // Admin check
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        
        // Check if the user exists
        if let Some(user) = users.remove(&principal) {
            
            // Remove the username associated with the principal
            USERNAMES.with(|usernames| {
                let mut usernames = usernames.borrow_mut();
                
                // Find the username associated with this principal
                let username_to_remove = usernames.iter()
                    .find_map(|(username, p)| if p == principal { Some(username.clone()) } else { None });
                
                // Remove the username if found
                if let Some(username) = username_to_remove {
                    usernames.remove(&username);
                }
            });

            Ok(format!("User '{}' deleted successfully.", user.username))
        } else {
            Err("User not found.".to_string())
        }
    })
}

#[update]
fn add_admin(principal: Principal) -> Result<(), String> {
    
    let mut user = get_user(principal)?;
    user.admin = true;

    USERS.with(|users| {
        users.borrow_mut().insert(principal, user);
    });

    Ok(())
}





// Optional: Helper function for future use
// fn is_authorized(principal: Principal) -> bool {
//     // Add authorization logic here
//     false
// }