use candid::Principal;
use ic_ledger_types::AccountIdentifier;
use icrc_ledger_types::icrc1::account::Account;
use regex::Regex;
use std::sync::OnceLock;

use crate::token::{stable_token::StableToken};

use super::address::Address;
use super::icp::is_icp_token_id;

static PRINCIPAL_ID_LOCK: OnceLock<Regex> = OnceLock::new();
const PRINCIPAL_ID_REGEX: &str = r"^([a-z0-9]{5}-){10}[a-z0-9]{3}$|^([a-z0-9]{5}-){4}cai$";
static ACCOUNT_ID_LOCK: OnceLock<Regex> = OnceLock::new();
const ACCOUNT_ID_REGEX: &str = r"^[a-f0-9]{64}$";

pub fn is_principal_id(address: &str) -> bool {
    let regrex_principal_id = PRINCIPAL_ID_LOCK.get_or_init(|| Regex::new(PRINCIPAL_ID_REGEX).unwrap());
    regrex_principal_id.is_match(address)
}

pub fn get_address(token: &StableToken, address: &str) -> Result<Address, String> {
    let regrex_princiapl_id = PRINCIPAL_ID_LOCK.get_or_init(|| Regex::new(PRINCIPAL_ID_REGEX).unwrap());
    let regrex_account_id = ACCOUNT_ID_LOCK.get_or_init(|| Regex::new(ACCOUNT_ID_REGEX).unwrap());

    //Then it ensures that the token supports ICRC-1, because Principal IDs only work with ICRC-1 tokens
// Converts the string to a Principal and wraps it in Address::PrincipalId
    if regrex_princiapl_id.is_match(address) {
        if !token.is_icrc1() {
            return Err("Principal Id requires ICRC1 token".to_string());
        }
        Ok(Address::PrincipalId(Account::from(
            Principal::from_text(address).map_err(|e| e.to_string())?,
        )))
    } 
//     If the address is a 64-character hex string, it’s treated as an AccountId

// But only allowed for tokens other than ICP

// Converts the hex to an AccountIdentifier and wraps in Address::AccountId
    else if regrex_account_id.is_match(address) {
        if is_icp_token_id(token.token_id()) {
            return Err("Account Id supported only for ICP token".to_string());
        }
        Ok(Address::AccountId(AccountIdentifier::from_hex(address).map_err(|e| e.to_string())?))
    } else {
        Err("Invalid address format".to_string())
    }
}
