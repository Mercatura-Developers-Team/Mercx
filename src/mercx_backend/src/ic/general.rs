use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::query;
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens, TransferArg, TransferError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use serde::Serialize;
//use ic_cdk::caller;
use crate::stable_memory::WHITELIST;


// pub const CANISTER_ID_XRC:&str="uf6dk-hyaaa-aaaaq-qaaaq-cai";
// pub const CANISTER_ID_ICRC1_LEDGER_CANISTER :&str="7p6gu-biaaa-aaaap-aknta-cai";
// pub const CANISTER_ID_ICRC1_INDEX_CANISTER :&str="7i7aa-mqaaa-aaaap-akntq-cai";
// pub const CANISTER_ID_ICP_LEDGER_CANISTER:&str ="ryjl3-tyaaa-aaaaa-aaaba-cai";
// pub const CANISTER_ID_ICP_INDEX_CANISTER :&str="qhbym-qaaaa-aaaaa-aaafq-cai";
// pub const CANISTER_ID_MERCX_BACKEND :&str="zoa6c-riaaa-aaaan-qzmta-cai";
// pub const CANISTER_ID_TOMMY_LEDGER_CANISTER: &str = "j47wy-ciaaa-aaaan-qzqyq-cai";
// pub const CANISTER_ID_CKUSDT_LEDGER_CANISTER: &str = "cngnf-vqaaa-aaaar-qag4q-cai";

// pub const CANISTER_ID_XRC: &str = "ahw5u-keaaa-aaaaa-qaaha-cai";
// pub const CANISTER_ID_ICRC1_LEDGER_CANISTER: &str = "be2us-64aaa-aaaaa-qaabq-cai";
// pub const CANISTER_ID_ICRC1_INDEX_CANISTER: &str = "bw4dl-smaaa-aaaaa-qaacq-cai";
// pub const CANISTER_ID_ICP_LEDGER_CANISTER: &str = "asrmz-lmaaa-aaaaa-qaaeq-cai";
// //pub const CANISTER_ID_ICP_INDEX_CANISTER: &str = "qhbym-qaaaa-aaaaa-aaafq-cai";
// pub const CANISTER_ID_MERCX_BACKEND: &str = "aovwi-4maaa-aaaaa-qaagq-cai";
// pub const CANISTER_ID_TOMMY_LEDGER_CANISTER: &str = "by6od-j4aaa-aaaaa-qaadq-cai";
// pub const CANISTER_ID_CKUSDT_LEDGER_CANISTER: &str = "br5f7-7uaaa-aaaaa-qaaca-cai";


//Nour's Id
pub const CANISTER_ID_XRC: &str = "ahw5u-keaaa-aaaaa-qaaha-cai";
pub const CANISTER_ID_ICRC1_LEDGER_CANISTER: &str = "b77ix-eeaaa-aaaaa-qaada-cai";
pub const CANISTER_ID_ICRC1_INDEX_CANISTER: &str = "bw4dl-smaaa-aaaaa-qaacq-cai";
pub const CANISTER_ID_ICP_LEDGER_CANISTER: &str = "br5f7-7uaaa-aaaaa-qaaca-cai";
//pub const CANISTER_ID_ICP_INDEX_CANISTER: &str = "qhbym-qaaaa-aaaaa-aaafq-cai";
pub const CANISTER_ID_MERCX_BACKEND: &str = "ahw5u-keaaa-aaaaa-qaaha-cai";
pub const CANISTER_ID_TOMMY_LEDGER_CANISTER: &str = "by6od-j4aaa-aaaaa-qaadq-cai";
pub const CANISTER_ID_CKUSDT_LEDGER_CANISTER: &str = "bkyz2-fmaaa-aaaaa-qaaaq-cai";



//The function allows you to query the principal ID of the caller of the function
#[query]
fn whoami() -> Principal {
    ic_cdk::caller()
}

pub fn get_time() -> u64 {
    ic_cdk::api::time()
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferArgs {
    amount: NumTokens,
    to_account: Account,
}

#[ic_cdk::update]
fn add_to_whitelist(principal: Principal) {
    WHITELIST.with(|whitelist| {
        whitelist.borrow_mut().insert(principal, true);
    });
    ic_cdk::println!("Added {} to whitelist", principal.to_text());
}

#[ic_cdk::update]
fn remove_from_whitelist(principal: Principal) {
    WHITELIST.with(|whitelist| {
        whitelist.borrow_mut().remove(&principal);
    });
    ic_cdk::println!("Removed {} from whitelist", principal.to_text());
}

#[ic_cdk::update]
fn reset_whitelist() {
    WHITELIST.with(|whitelist| {
        whitelist.borrow_mut().clear_new();
    });
    ic_cdk::println!("✅ Whitelist has been reset.");
}


#[ic_cdk::query]
fn get_whitelisted_principals() -> Vec<String> {
    WHITELIST.with(|whitelist| {
        whitelist.borrow()
            .iter()
            .map(|(principal, _)| principal.to_text()) // Convert Principal to String
            .collect()
    })
}


#[ic_cdk::query]
fn is_whitelisted(principle:Principal) -> bool {

    WHITELIST.with(|whitelist| {
        whitelist.borrow().get(&principle).unwrap_or(false)
})
}


#[ic_cdk::update]
async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &args.to_account,
    );

    let caller_principal = ic_cdk::caller();
    ic_cdk::println!("Caller Principal: {}", caller_principal.to_text());

        // Check if the recipient is whitelisted
        if !is_whitelisted(args.to_account.owner){
            return Err(format!(
                "Transfer failed: recipient {} is not whitelisted.",
                args.to_account.owner
            ));
        }

      

    let transfer_args: TransferArg = TransferArg {
        // can be used to distinguish between transactions
        memo: None,
        // the amount we want to transfer
        amount: args.amount,
        // we want to transfer tokens from the default subaccount of the canister
        from_subaccount: None,
        // if not specified, the default fee for the canister is used
        fee: None,
        // the account we want to transfer tokens to
        to: args.to_account,
        // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
        created_at_time: None,
    };

    // let ledger_principal = env::var("CANISTER_ID_ICRC1_LEDGER_CANISTER")
    // .expect("Ledger Canister Principal ID not set in .env");
    // 1. Asynchronously call another canister function using ic_cdk::call.
    ic_cdk::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
        // 2. Convert a textual representation of a Principal into an actual Principal object. The principal is the one we specified in dfx.json.
        //    expect will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
        Principal::from_text(CANISTER_ID_ICRC1_LEDGER_CANISTER)
            .expect("Could not decode the principal."),
        // 3. Specify the method name on the target canister to be called, in this case, "icrc1_transfer".
        "icrc1_transfer",
        // 4. Provide the arguments for the call in a tuple, here transfer_args is encapsulated as a single-element tuple.
        (transfer_args,),
    )
    .await // 5. Await the completion of the asynchronous call, pausing the execution until the future is resolved.
    // 6. Apply map_err to transform any network or system errors encountered during the call into a more readable string format.
    //    The ? operator is then used to propagate errors: if the result is an Err, it returns from the function with that error,
    //    otherwise, it unwraps the Ok value, allowing the chain to continue.
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    // 7. Access the first element of the tuple, which is the Result<BlockIndex, TransferError>, for further processing.
    .0
    // 8. Use map_err again to transform any specific ledger transfer errors into a readable string format, facilitating error handling and debugging.
    .map_err(|e| format!("ledger transfer error {:?}", e))
}

#[ic_cdk::update]
async fn transfer_from(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &args.to_account,
    );

    let transfer_from_args = TransferFromArgs {
        // the account we want to transfer tokens from (in this case we assume the caller approved the canister to spend funds on their behalf)
        from: Account::from(ic_cdk::caller()), //when changed to api::id the from and spender where the same (MERCX canister)
        // can be used to distinguish between transactions
        memo: None,
        // the amount we want to transfer
        amount: args.amount,
        // the subaccount we want to spend the tokens from (in this case we assume the default subaccount has been approved)
        spender_subaccount: None,
        // if not specified, the default fee for the canister is used
        fee: None,
        // the account we want to transfer tokens to
        to: args.to_account,
        // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
        created_at_time: None,
    };

    // 1. Asynchronously call another canister function using `ic_cdk::call`.
    ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
        // 2. Convert a textual representation of a Principal into an actual `Principal` object. The principal is the one we specified in `dfx.json`.
        //    `expect` will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
        Principal::from_text(CANISTER_ID_ICP_LEDGER_CANISTER)
            .expect("Could not decode the principal."),
        // 3. Specify the method name on the target canister to be called, in this case, "icrc1_transfer".
        "icrc2_transfer_from",
        // 4. Provide the arguments for the call in a tuple, here `transfer_args` is encapsulated as a single-element tuple.
        (transfer_from_args,),
    )
    .await // 5. Await the completion of the asynchronous call, pausing the execution until the future is resolved.
    // 6. Apply `map_err` to transform any network or system errors encountered during the call into a more readable string format.
    //    The `?` operator is then used to propagate errors: if the result is an `Err`, it returns from the function with that error,
    //    otherwise, it unwraps the `Ok` value, allowing the chain to continue.
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    // 7. Access the first element of the tuple, which is the `Result<BlockIndex, TransferError>`, for further processing.
    .0
    // 8. Use `map_err` again to transform any specific ledger transfer errors into a readable string format, facilitating error handling and debugging.
    .map_err(|e| format!("ledger transfer error {:?}", e))
}

#[ic_cdk::update]
async fn transfer_from_general(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &args.to_account,
    );

    let transfer_from_args = TransferFromArgs {
        // the account we want to transfer tokens from (in this case we assume the caller approved the canister to spend funds on their behalf)
        from: Account::from(ic_cdk::caller()), //when changed to api::id the from and spender where the same (MERCX canister)
        // can be used to distinguish between transactions
        memo: None,
        // the amount we want to transfer
        amount: args.amount,
        // the subaccount we want to spend the tokens from (in this case we assume the default subaccount has been approved)
        spender_subaccount: None,
        // if not specified, the default fee for the canister is used
        fee: None,
        // the account we want to transfer tokens to
        to: args.to_account,
        // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
        created_at_time: None,
    };

    // 1. Asynchronously call another canister function using `ic_cdk::call`.
    ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
        // 2. Convert a textual representation of a Principal into an actual `Principal` object. The principal is the one we specified in `dfx.json`.
        //    `expect` will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
        Principal::from_text(CANISTER_ID_ICRC1_LEDGER_CANISTER)
            .expect("Could not decode the principal."),
        // 3. Specify the method name on the target canister to be called, in this case, "icrc1_transfer".
        "icrc2_transfer_from",
        // 4. Provide the arguments for the call in a tuple, here `transfer_args` is encapsulated as a single-element tuple.
        (transfer_from_args,),
    )
    .await // 5. Await the completion of the asynchronous call, pausing the execution until the future is resolved.
    // 6. Apply `map_err` to transform any network or system errors encountered during the call into a more readable string format.
    //    The `?` operator is then used to propagate errors: if the result is an `Err`, it returns from the function with that error,
    //    otherwise, it unwraps the `Ok` value, allowing the chain to continue.
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    // 7. Access the first element of the tuple, which is the `Result<BlockIndex, TransferError>`, for further processing.
    .0
    // 8. Use `map_err` again to transform any specific ledger transfer errors into a readable string format, facilitating error handling and debugging.
    .map_err(|e| format!("ledger transfer error {:?}", e))
}

#[ic_cdk::update]
async fn deposit_icp_in_canister(amount: u64) -> Result<BlockIndex, String> {
    let transfer_from_args = TransferFromArgs {
        // the account we want to transfer tokens from (in this case we assume the caller approved the canister to spend funds on their behalf)
        from: Account::from(ic_cdk::caller()), //when changed to api::id the from and spender where the same (MERCX canister)
        // can be used to distinguish between transactions
        memo: None,
        // the amount we want to transfer
        amount: amount.into(),
        // the subaccount we want to spend the tokens from (in this case we assume the default subaccount has been approved)
        spender_subaccount: None,
        // if not specified, the default fee for the canister is used
        fee: None,
        // the account we want to transfer tokens to
        to: Account::from(ic_cdk::api::id()),
        // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
        created_at_time: None,
    };

    // 1. Asynchronously call another canister function using `ic_cdk::call`.
    ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
        // 2. Convert a textual representation of a Principal into an actual `Principal` object. The principal is the one we specified in `dfx.json`.
        //    `expect` will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
        Principal::from_text(CANISTER_ID_ICP_LEDGER_CANISTER)
            .expect("Could not decode the principal."),
        // 3. Specify the method name on the target canister to be called, in this case, "icrc1_transfer".
        "icrc2_transfer_from",
        // 4. Provide the arguments for the call in a tuple, here `transfer_args` is encapsulated as a single-element tuple.
        (transfer_from_args,),
    )
    .await // 5. Await the completion of the asynchronous call, pausing the execution until the future is resolved.
    // 6. Apply `map_err` to transform any network or system errors encountered during the call into a more readable string format.
    //    The `?` operator is then used to propagate errors: if the result is an `Err`, it returns from the function with that error,
    //    otherwise, it unwraps the `Ok` value, allowing the chain to continue.
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    // 7. Access the first element of the tuple, which is the `Result<BlockIndex, TransferError>`, for further processing.
    .0
    // 8. Use `map_err` again to transform any specific ledger transfer errors into a readable string format, facilitating error handling and debugging.
    .map_err(|e| format!("ledger transfer error {:?}", e))
}

#[ic_cdk::update]
async fn deposit_token(amount: u64, token_info: Principal) -> Result<BlockIndex, String> {
    let transfer_from_args = TransferFromArgs {
        from: Account::from(ic_cdk::caller()), // Assuming the caller has approved the transfer
        memo: None,
        amount: amount.into(),
        spender_subaccount: None,
        fee: None,
        to: Account::from(ic_cdk::api::id()),
        created_at_time: None,
    };

    // Properly handle the output of the call directly as a tuple
    let call_result: Result<(Result<BlockIndex, TransferFromError>,), _> = ic_cdk::api::call::call(token_info, "icrc2_transfer_from", (transfer_from_args,)).await;

    // Flatten the result for easier error handling
    match call_result {
        Ok((Ok(block_index),)) => Ok(block_index),
        Ok((Err(e),)) => Err(format!("Ledger transfer error: {:?}", e)),
        Err(e) => Err(format!("Failed to call ledger: {:?}", e)),
    }
}


#[ic_cdk::update]
async fn check_balance_icp(account: Account) -> NumTokens {
    // Perform the call to icrc1_balance_of canister method
    let (balance_result,): (NumTokens,) = ic_cdk::call::<(Account,), (NumTokens,)>(
        Principal::from_text(CANISTER_ID_ICP_LEDGER_CANISTER)
            .expect("Could not decode the principal."),
        "icrc1_balance_of",
        (account,),
    )
    .await
    .expect("failed to retrieve balance");

    // Return the balance directly
    balance_result
}

#[ic_cdk::update]
async fn check_balance_mercx(account: Account) -> NumTokens {
    // Perform the call to icrc1_balance_of canister method
    let (balance_result,): (NumTokens,) = ic_cdk::call::<(Account,), (NumTokens,)>(
        Principal::from_text(CANISTER_ID_ICRC1_LEDGER_CANISTER)
            .expect("Could not decode the principal."),
        "icrc1_balance_of",
        (account,),
    )
    .await
    .expect("failed to retrieve balance");

    // Return the balance directly
    balance_result
}



#[ic_cdk::update]
async fn check_balance(account: Account, token_info: Principal) -> Result<NumTokens, String> {
    // Perform the call to icrc1_balance_of canister method
    let result: Result<(NumTokens,), _> =
        ic_cdk::api::call::call(token_info, "icrc1_balance_of", (account,)).await;

    match result {
        Ok((balance,)) => Ok(balance),
        Err(e) => Err(format!("Failed to retrieve balance: {:?}", e)),
    }
}

#[ic_cdk::query]
pub async fn get_name(ledger: Principal) -> Result<String, String> {
    ic_cdk::call::<(), (String,)>(ledger, "icrc1_name", ())
        .await
        .map(|(name,)| name)
        .map_err(|e| e.1)
}

pub async fn get_symbol(ledger: Principal) -> Result<String, String> {
    ic_cdk::call::<(), (String,)>(ledger, "icrc1_symbol", ())
        .await
        .map(|(symbol,)| symbol)
        .map_err(|e| e.1)
}

pub async fn get_decimals(ledger: Principal) -> Result<u8, String> {
    ic_cdk::call::<(), (u8,)>(ledger, "icrc1_decimals", ())
        .await
        .map(|(decimals,)| decimals)
        .map_err(|e| e.1)
}

pub async fn get_fee(ledger: Principal) -> Result<Nat, String> {
    ic_cdk::call::<(), (Nat,)>(ledger, "icrc1_fee", ())
        .await
        .map(|(fee,)| fee)
        .map_err(|e| e.1)
}


// #[ic_cdk::update]
// async fn get_logo_url() -> Result<String, String> {
//     let (metadata,): (Vec<(String, MetadataValue)>,) = ic_cdk::call(
//         Principal::from_text(CANISTER_ID_ICRC1_LEDGER_CANISTER)
//             .expect("Could not decode the principal."),
//         "icrc1_metadata",
//         (),
//     )
//     .await
//     .map_err(|e| format!("failed to retrieve metadata: {:?}", e))?;

//   // Find the "logo_url" in the metadata
//     for (key, value) in metadata {
//         if key == "logo_url" {
//             if let MetadataValue::Text(url) = value {
//                 return Ok(url);
//             }
//         }
//     }

//     Err("Logo URL not found in metadata".to_string())
// }



// #[ic_cdk::update]
// async fn get_logo_url() -> String {
//     let (metadata,): (Vec<(String, MetadataValue)>,) = ic_cdk::call(
//         Principal::from_text(CANISTER_ID_CKUSDT_LEDGER_CANISTER)
//             .expect("Could not decode the principal."),
//         "icrc1_metadata",
//         (),
//     )
//     .await
//     .expect("failed to retrieve metadata");

//     // Find the "logo_url" in the metadata
//     for (key, value) in metadata {
//         if key == "logo_url" {
//             if let MetadataValue::Text(url) = value {
//                 return url;
//             }
//         }
//     }

//     // Default return if the logo URL is not found
//     "Logo URL not found".to_string()
// }



//use call result because You’re handling the success or failure of the inter-canister call explicitly.
#[ic_cdk::update]
async fn get_logo_url(ledger: Principal) -> String {
    let result: ic_cdk::api::call::CallResult<(Vec<(String, MetadataValue)>,)> = ic_cdk::call(
        ledger,
        "icrc1_metadata",
        (),
    )
    .await;

    match result {
        Ok((metadata,)) => {
            for (key, value) in metadata {
                if key == "logo_url" || key == "icrc1:logo" {
                    if let MetadataValue::Text(url) = value {
                        return url;
                    }
                }
            }
            "logo_url not found in metadata".to_string()
        }
        Err((_, msg)) => format!("Error calling metadata: {}", msg),
    }
}


#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct StandardRecord {
    pub url: String,
    pub name: String,
}

/// try icrc10_supported_standards first, if it fails, try icrc1_supported_standards
pub async fn get_supported_standards(ledger: Principal) -> Result<Vec<StandardRecord>, String> {
    match ic_cdk::call::<(), (Vec<StandardRecord>,)>(ledger, "icrc10_supported_standards", ())
        .await
        .map(|(standards,)| standards)
    {
        Ok(standards) => Ok(standards),
        Err(_) => ic_cdk::call::<(), (Vec<StandardRecord>,)>(ledger, "icrc1_supported_standards", ())
            .await
            .map(|(standards,)| standards)
            .map_err(|e| e.1),
    }
}
// pub async fn get_logo(ledger: Principal) -> Result<Nat, String> {
//     ic_cdk::call::<(), (Nat,)>(ledger, "icrc1_metadata", ())
//         .await
//         .map(|(fee,)| fee)
//         .map_err(|e| e.1)
// }

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct TransactionQueryArgs {
    start: candid::Nat,
    length: candid::Nat,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct TransactionResponse {
    first_index: candid::Nat,
    log_length: candid::Nat,
    transactions: Vec<Transaction>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct Transaction {
    kind: String,
    timestamp: u64,
    // amount: Option<NumTokens>,
    // from_account: Option<Account>,
    // to_account: Option<Account>,
    burn: Option<Burn>,
    mint: Option<Mint>,
    approve: Option<Approve>,
    transfer: Option<Transfer>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct Burn {
    from: Account,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
    amount: NumTokens,
    spender: Option<Account>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct Mint {
    to: Account,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
    amount: NumTokens,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct Approve {
    fee: Option<NumTokens>,
    from: Account,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
    amount: NumTokens,
    expected_allowance: Option<NumTokens>,
    expires_at: Option<u64>,
    spender: Account,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct Transfer {
    to: Account,
    fee: Option<NumTokens>,
    from: Account,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
    amount: NumTokens,
    spender: Option<Account>,
}

#[ic_cdk::update]
async fn get_transactions(
    start: candid::Nat,
    length: candid::Nat,
) -> Result<TransactionResponse, String> {
    let get_transactions_args = TransactionQueryArgs { start, length };

    let (response,): (TransactionResponse,) = ic_cdk::call(
        Principal::from_text(CANISTER_ID_ICRC1_LEDGER_CANISTER)
            .expect("Could not decode the principal."),
        "get_transactions",
        (get_transactions_args,),
    )
    .await
    .map_err(|e| format!("failed to retrieve transactions: {:?}", e))?;

    Ok(response)
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct GetAccountTransactionsArgs {
    max_results: Nat,
    start: Option<Nat>,
    account: Account,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct GetTransactions {
    balance: Nat,
    transactions: Vec<TransactionWithId>,
    oldest_tx_id: Option<Nat>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct GetTransactionsErr {
    message: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetTransactionsResult {
    #[serde(rename = "Ok")]
    Ok(GetTransactions),
    #[serde(rename = "Err")]
    Err(GetTransactionsErr),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct TransactionWithId {
    id: Nat,
    transaction: Transaction,
}

#[ic_cdk::update]
async fn get_account_transactions(
    account: Account,
    start: Option<Nat>,
    max_results: Nat,
) -> Result<GetTransactions, String> {
    let args = GetAccountTransactionsArgs {
        account,
        start,
        max_results,
    };

    let (result,): (GetTransactionsResult,) = ic_cdk::call(
        Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai").unwrap(),
        "get_account_transactions",
        (args,), // Pass as a single argument
    )
    .await
    .map_err(|e| format!("Failed to call canister: {:?}", e))?;

    match result {
        GetTransactionsResult::Ok(response) => Ok(response),
        GetTransactionsResult::Err(err) => Err(err.message),
    }
}

#[ic_cdk::update]
async fn send_mercx(amount: u64) -> Result<BlockIndex, String> {
    let caller: Principal = ic_cdk::caller();
    let amount = Nat::from(amount);

    let transfer_args: TransferArg = TransferArg {
        // can be used to distinguish between transactions
        // the amount we want to transfer
        amount,
        // we want to transfer tokens from the default subaccount of the canister
        from_subaccount: None,
        // if not specified, the default fee for the canister is used
        fee: None,
        // the account we want to transfer tokens to
        to: caller.into(),
        // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
        created_at_time: None,
        memo: None,
    };

    // let ledger_principal = env::var("CANISTER_ID_ICRC1_LEDGER_CANISTER")
    // .expect("Ledger Canister Principal ID not set in .env");
    // 1. Asynchronously call another canister function using ic_cdk::call.
    ic_cdk::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
        // 2. Convert a textual representation of a Principal into an actual Principal object. The principal is the one we specified in dfx.json.
        //    expect will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
        Principal::from_text(CANISTER_ID_ICRC1_LEDGER_CANISTER)
            .expect("Could not decode the principal."),
        // 3. Specify the method name on the target canister to be called, in this case, "icrc1_transfer".
        "icrc1_transfer",
        // 4. Provide the arguments for the call in a tuple, here transfer_args is encapsulated as a single-element tuple.
        (transfer_args,),
    )
    .await // 5. Await the completion of the asynchronous call, pausing the execution until the future is resolved.
    // 6. Apply map_err to transform any network or system errors encountered during the call into a more readable string format.
    //    The ? operator is then used to propagate errors: if the result is an Err, it returns from the function with that error,
    //    otherwise, it unwraps the Ok value, allowing the chain to continue.
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    // 7. Access the first element of the tuple, which is the Result<BlockIndex, TransferError>, for further processing.
    .0
    // 8. Use map_err again to transform any specific ledger transfer errors into a readable string format, facilitating error handling and debugging.
    .map_err(|e| format!("ledger transfer mercx error {:?}", e))
}

#[ic_cdk::update]
async fn send_token(amount: u64, token_info: Principal) -> Result<BlockIndex, String> {
    let caller: Principal = ic_cdk::caller();
    let amount = Nat::from(amount);

    let transfer_args: TransferArg = TransferArg {
        // can be used to distinguish between transactions
        // the amount we want to transfer
        amount,
        // we want to transfer tokens from the default subaccount of the canister
        from_subaccount: None,
        // if not specified, the default fee for the canister is used
        fee: None,
        // the account we want to transfer tokens to
        to: caller.into(),
        // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
        created_at_time: None,
        memo: None,
    };

    ic_cdk::api::call::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
        // 2. Convert a textual representation of a Principal into an actual Principal object. The principal is the one we specified in dfx.json.
        //    expect will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
        token_info,
        // 3. Specify the method name on the target canister to be called, in this case, "icrc1_transfer".
        "icrc1_transfer",
        // 4. Provide the arguments for the call in a tuple, here transfer_args is encapsulated as a single-element tuple.
        (transfer_args,),
    )
    .await // 5. Await the completion of the asynchronous call, pausing the execution until the future is resolved.
    // 6. Apply map_err to transform any network or system errors encountered during the call into a more readable string format.
    //    The ? operator is then used to propagate errors: if the result is an Err, it returns from the function with that error,
    //    otherwise, it unwraps the Ok value, allowing the chain to continue.
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    // 7. Access the first element of the tuple, which is the Result<BlockIndex, TransferError>, for further processing.
    .0
    // 8. Use map_err again to transform any specific ledger transfer errors into a readable string format, facilitating error handling and debugging.
    .map_err(|e| format!("ledger transfer tommy error {:?}", e))
}




// #[ic_cdk::update]
// async fn send_tommy(amount: u64) -> Result<BlockIndex, String> {
//     let caller: Principal = ic_cdk::caller();
//     let amount = Nat::from(amount);

//     let transfer_args: TransferArg = TransferArg {
//         // can be used to distinguish between transactions
//         // the amount we want to transfer
//         amount,
//         // we want to transfer tokens from the default subaccount of the canister
//         from_subaccount: None,
//         // if not specified, the default fee for the canister is used
//         fee: None,
//         // the account we want to transfer tokens to
//         to: caller.into(),
//         // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
//         created_at_time: None,
//         memo: None,
//     };

//     // let ledger_principal = env::var("CANISTER_ID_ICRC1_LEDGER_CANISTER")
//     // .expect("Ledger Canister Principal ID not set in .env");
//     // 1. Asynchronously call another canister function using ic_cdk::call.
//     ic_cdk::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
//         // 2. Convert a textual representation of a Principal into an actual Principal object. The principal is the one we specified in dfx.json.
//         //    expect will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
//         Principal::from_text(CANISTER_ID_TOMMY_LEDGER_CANISTER)
//             .expect("Could not decode the principal."),
//         // 3. Specify the method name on the target canister to be called, in this case, "icrc1_transfer".
//         "icrc1_transfer",
//         // 4. Provide the arguments for the call in a tuple, here transfer_args is encapsulated as a single-element tuple.
//         (transfer_args,),
//     )
//     .await // 5. Await the completion of the asynchronous call, pausing the execution until the future is resolved.
//     // 6. Apply map_err to transform any network or system errors encountered during the call into a more readable string format.
//     //    The ? operator is then used to propagate errors: if the result is an Err, it returns from the function with that error,
//     //    otherwise, it unwraps the Ok value, allowing the chain to continue.
//     .map_err(|e| format!("failed to call ledger: {:?}", e))?
//     // 7. Access the first element of the tuple, which is the Result<BlockIndex, TransferError>, for further processing.
//     .0
//     // 8. Use map_err again to transform any specific ledger transfer errors into a readable string format, facilitating error handling and debugging.
//     .map_err(|e| format!("ledger transfer tommy error {:?}", e))
// }

#[ic_cdk::update]
pub async fn swap(amount_icp: u64, amount_mercx: u64) -> Result<String, String> {
   
    let caller = ic_cdk::caller();

    let principal = Principal::from_text(CANISTER_ID_MERCX_BACKEND)
        .map_err(|e| format!("Error parsing principal: {:?}", e))?;
    let account = Account::from(principal);
    let mercx_balance = check_balance_mercx(account).await;

    let icp_balance = check_balance_icp(Account::from(caller)).await;
    // Check if ICP balance is sufficient
    if amount_icp > icp_balance {
        return Err("Insufficient ICP balance".to_string());
    }

    // Check if MERCX balance is sufficient
    if amount_mercx as u64 > mercx_balance {
        return Err("Insufficient MERCX balance".to_string());
    }

    deposit_icp_in_canister(amount_icp).await?;

    match send_mercx(amount_mercx).await {
        Ok(block_index) => {
            // Mint was successful
            ic_cdk::println!("Successful, block index: {:?}", block_index);
        }
        Err(e) => {
            // If there was an error, log it in archive trx and return an error result
            return Err(e);
        } // };
    }

    Ok("Swapped Successfully!".to_string())
}

#[ic_cdk::update]
pub async fn sell(
    amount_from: u64,
    from_token: Principal,
    amount_to: u64,
    to_token: Principal,
) -> Result<String, String> {
    let caller = ic_cdk::caller();
    let to_principal = Principal::from_text(CANISTER_ID_MERCX_BACKEND)
        .map_err(|e| format!("Error parsing Mercx canister principal: {:?}", e))?;
    // let from_principal = Principal::from_text(CANISTER_ID_ICRC1_LEDGER_CANISTER)
    //     .map_err(|e| format!("Error parsing ICP ledger principal: {:?}", e))?;

    // Check if the recipient is whitelisted
    if !is_whitelisted(caller){
        return Err(format!(
            "Transfer failed: recipient {} is not whitelisted.",
            caller
        ));
    }

    let account_to = Account::from(to_principal);
    let account_from = Account::from(caller);

    let from_balance = check_balance(account_from, from_token).await?;
    let to_balance = check_balance(account_to, to_token).await?;

    // Check if balances are sufficient
    if amount_from > from_balance {
        return Err("Insufficient Bella balance".to_string());
    }
    
    if amount_to as u64 > to_balance {
        return Err("Insufficient Tommy balance".to_string());
    }


    deposit_token(amount_from,from_token).await?;

    match send_token(amount_to,to_token).await {
        Ok(block_index) => {
            // Mint was successful
            ic_cdk::println!("Successful, block index: {:?}", block_index);
        }
        Err(e) => {
            // If there was an error, log it in archive trx and return an error result
            return Err(e);
        } // };
    }

    Ok("Swapped Successfully!".to_string())
}

