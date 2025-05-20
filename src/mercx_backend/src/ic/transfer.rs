use candid::Nat;
//use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens, TransferArg, TransferError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use icrc_ledger_types::icrc1::account::Account;


use crate::helpers::math_helpers::nat_is_zero;
use crate::StableToken;


// icrc2_transfer_from using principal id's where from_principal_id has issued an icrc2_approve
pub async fn icrc2_transfer_from(
    token: &StableToken,
    amount: &Nat,
    from_principal_id: &Account,
    to_principal_id: &Account,
) -> Result<Nat, String> {
    if !token.is_icrc2() {
        return Err("Token does not support ICRC2".to_string());
    }
    if nat_is_zero(amount) {
        return Err("Transfer_from amount is zero".to_string());
    }
    let id = *token.canister_id().ok_or("Invalid principal id")?;

    let transfer_from_args = TransferFromArgs {
        spender_subaccount: None,
        from: *from_principal_id,
        to: *to_principal_id,
        amount: amount.clone(),
        fee: None,
        memo: None,
        created_at_time: None,
    };

    let block_id =
        match ic_cdk::call::<(TransferFromArgs,), (Result<Nat, TransferFromError>,)>(id, "icrc2_transfer_from", (transfer_from_args,))
            .await
            .map_err(|e| e.1)?
            .0
        {
            Ok(block_id) => block_id,
            Err(e) => Err(e.to_string())?,
        };
    Ok(block_id)
}