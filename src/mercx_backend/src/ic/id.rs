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