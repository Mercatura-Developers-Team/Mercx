use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use ic_ledger_types::{AccountIdentifier, Subaccount};


/// Principal ID of the caller.
pub fn caller() -> Principal {
    ic_cdk::api::caller()  //Who is calling me right now (user or other canister)
}

/// Account of the caller.
pub fn caller_id() -> Account {
    Account::from(caller())
}

/// Account ID of the caller.
/// Used for ICP token
pub fn caller_account_id() -> AccountIdentifier {
    let account = caller_id();
    let subaccount = Subaccount(account.subaccount.unwrap_or([0; 32]));
    AccountIdentifier::new(&account.owner, &subaccount)
}

/// Check to make sure Principal Id is not anonymous
pub fn principal_id_is_not_anonymous(principal_id: &str) -> Result<(), String> {
    if principal_id == Principal::anonymous().to_text() {
        return Err("Anonymous user not allowed".to_string());
    }
    Ok(())
}

