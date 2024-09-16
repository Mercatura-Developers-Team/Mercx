use candid::{CandidType, Deserialize, Principal};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens, TransferArg, TransferError};
use serde::Serialize;
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;


#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferArgs {
    amount: NumTokens,
    to_account: Account,
}

#[ic_cdk::update]
async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &args.to_account,
    );

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

    // 1. Asynchronously call another canister function using ic_cdk::call.
    ic_cdk::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
        // 2. Convert a textual representation of a Principal into an actual Principal object. The principal is the one we specified in dfx.json.
        //    expect will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
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


// Function to check the balance
// #[ic_cdk::update]
// async fn check_balance(account: Account) -> Result<NumTokens, String> {
//     Ok(ic_cdk::call::<(Account,), (NumTokens,)>(
//         Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
//             .expect("Could not decode the principal."),
//         "icrc1_balance_of",
//         (account,),
//     )
//     .await
//     .map_err(|e| format!("failed to retrieve balance: {:?}", e))?
//     .0)
// }
#[ic_cdk::update]
async fn check_balance(account: Account) -> NumTokens {
    // Perform the call to icrc1_balance_of canister method
    let (balance_result,): (NumTokens,) = ic_cdk::call::<(Account,), (NumTokens,)>(
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
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
async fn get_token_name() -> String {
    let (token_name,): (String,) = ic_cdk::call(
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
            .expect("Could not decode the principal."),
        "icrc1_name",
        (),
    )
    .await
    .expect("failed to retrieve token name");

    token_name
}

// #[ic_cdk::update]
// async fn get_logo_url() -> Result<String, String> {
//     let (metadata,): (Vec<(String, MetadataValue)>,) = ic_cdk::call(
//         Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
//             .expect("Could not decode the principal."),
//         "icrc1_metadata",
//         (),
//     )
//     .await
//     .map_err(|e| format!("failed to retrieve metadata: {:?}", e))?;

//     // Find the "logo_url" in the metadata
//     for (key, value) in metadata {
//         if key == "logo_url" {
//             if let MetadataValue::Text(url) = value {
//                 return Ok(url);
//             }
//         }
//     }

//     Err("Logo URL not found in metadata".to_string())
// }

#[ic_cdk::update]
async fn get_logo_url() -> String {
    let (metadata,): (Vec<(String, MetadataValue)>,) = ic_cdk::call(
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
            .expect("Could not decode the principal."),
        "icrc1_metadata",
        (),
    )
    .await
    .expect("failed to retrieve metadata");

    // Find the "logo_url" in the metadata
    for (key, value) in metadata {
        if key == "logo_url" {
            if let MetadataValue::Text(url) = value {
                return url;
            }
        }
    }

    // Default return if the logo URL is not found
    "Logo URL not found".to_string()
}



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
async fn get_transactions(start: candid::Nat, length: candid::Nat) -> Result<TransactionResponse, String> {
    let get_transactions_args = TransactionQueryArgs { start, length };

    let (response,): (TransactionResponse,) = ic_cdk::call(
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").expect("Could not decode the principal."),
        "get_transactions",
        (get_transactions_args,),
    )
    .await
    .map_err(|e| format!("failed to retrieve transactions: {:?}", e))?;

    Ok(response)
}


// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();