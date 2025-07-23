use candid:: Nat;
use icrc_ledger_types::icrc3::transactions::{GetTransactionsRequest, GetTransactionsResponse};
use crate::StableToken;
use ic_ledger_types::{query_blocks, AccountIdentifier, Block, GetBlocksArgs, Operation, Subaccount, Tokens};
use crate::ic::id::{caller_account_id, caller_id};
use crate::helpers::math_helpers::nat_to_u64;
use crate::stable_mercx_settings::mercx_settings_map;
use crate::ic::general::get_time;

#[cfg(not(feature = "prod"))]
const ICP_CANISTER_ID: &str = "asrmz-lmaaa-aaaaa-qaaeq-cai";  // Testnet ICP Ledger
#[cfg(feature = "prod")]
const ICP_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai"; // Mainnet ICP Ledger

/// Verifies a transfer by checking the ledger.
/// For ICRC3 tokens, it tries ICRC3 methods first, falling back to traditional methods.
/// For non-ICRC3 tokens, it uses the traditional verification methods.
/// checks if the transfer happens correctly 
pub async fn verify_transfer(token: &StableToken, block_id: &Nat, amount: &Nat) -> Result<(), String> {


            let canister_id = *token.canister_id().ok_or("Invalid canister id")?;
          
            let mercx_settings = mercx_settings_map::get();
            let min_valid_timestamp = get_time() - mercx_settings.transfer_expiry_nanosecs;
            let mercx_backend_account = &mercx_settings.mercx_backend;
            let caller_account = caller_id();

            // if ICP ledger, use query_blocks
            if canister_id.to_text() == ICP_CANISTER_ID {
                return verify_trnasfer_with_query_blocks(token, block_id, amount, canister_id, min_valid_timestamp, mercx_backend_account)
                    .await  .map_err(|e| format!("❌ ICP verification failed: {}", e));
            }

            // otherwise, use get_transactions
             verify_transfer_with_get_transactions(
                token,
                block_id,
                amount,
                canister_id,
                min_valid_timestamp,
                mercx_backend_account,
                caller_account,
            )
            .await.map_err(|e| format!("❌ ICRC verification failed: {}", e))
       
        
}

async fn verify_transfer_with_get_transactions(
    token: &StableToken,
    block_id: &Nat,
    amount: &Nat,
    canister_id: candid::Principal,
    min_valid_timestamp: u64,
    mercx_backend_account: &icrc_ledger_types::icrc1::account::Account,
    caller_account: icrc_ledger_types::icrc1::account::Account,
) -> Result<(), String> {
   

        // Standard tokens use get_transactions (with 's')
        let block_args = GetTransactionsRequest {
            start: block_id.clone(),
            length: Nat::from(1_u32),
        };
        match ic_cdk::call::<(GetTransactionsRequest,), (GetTransactionsResponse,)>(canister_id, "get_transactions", (block_args,)).await {
            Ok(get_transactions_response) => {
                let transactions = get_transactions_response.0.transactions;
                for transaction in transactions.into_iter() {
                    if let Some(transfer) = transaction.transfer { //Checks if the transaction is of type transfer //If not, check other types (burn, mint, approve) later.
                        let from = transfer.from;
                        if from != caller_account {
                            Err("Transfer from does not match caller")?
                        }
                        let to = transfer.to;
                        if to != *mercx_backend_account {
                            Err("Transfer to does not match Mercx backend")?
                        }
                        // make sure spender is None, so not an icrc2_transfer_from transaction
                        let spender = transfer.spender;
                        if spender.is_some() {
                            Err("Invalid transfer spender")?
                        }
                        let transfer_amount = transfer.amount;
                        if transfer_amount != *amount {
                            Err(format!("Invalid transfer amount: rec {:?} exp {:?}", transfer_amount, amount))?
                        }
                        let timestamp = transaction.timestamp;
                        if timestamp < min_valid_timestamp {
                            Err("Expired transfer timestamp")?
                        }

                        return Ok(()); // success
                    }  else {
                        Err(format!("Invalid transaction kind: {}", transaction.kind))?
                    }
                }

                Err(format!("Failed to verify {} transfer block id {}", token.symbol(), block_id))?
            }
            Err(e) => Err(e.1)?,
        }
    
}

async fn verify_trnasfer_with_query_blocks(
    token: &StableToken,
    block_id: &Nat,
    amount: &Nat,
    canister_id: candid::Principal,
    min_valid_timestamp: u64,
    mercx_backend_account: &icrc_ledger_types::icrc1::account::Account,
) -> Result<(), String> {
    // if ICP ledger, use query_blocks
    let block_args = GetBlocksArgs {
        start: nat_to_u64(block_id).ok_or_else(|| format!("ICP ledger block id {:?} not found", block_id))?,
        length: 1,
    };
    match query_blocks(canister_id, block_args).await {
        Ok(query_response) => {
            let blocks: Vec<Block> = query_response.blocks;
            let backend_account_id = AccountIdentifier::new(
                &mercx_backend_account.owner,
                &Subaccount(mercx_backend_account.subaccount.unwrap_or([0; 32])),
            );
            let amount = Tokens::from_e8s(nat_to_u64(amount).ok_or("Invalid ICP amount")?);
            for block in blocks.into_iter() {
                match block.transaction.operation {
                    Some(operation) => match operation {
                        Operation::Transfer {
                            from,
                            to,
                            amount: transfer_amount,
                            ..
                        } => {
                            // ICP ledger seems to combine transfer and transfer_from
                            // use account id for ICP
                            if from != caller_account_id() {
                                Err("Transfer from does not match caller")?
                            }
                            if to != backend_account_id {
                                Err("Transfer to does not match Mercx backend")?
                            }
                            if transfer_amount != amount {
                                Err(format!("Invalid transfer amount: rec {:?} exp {:?}", transfer_amount, amount))?
                            }
                            if block.transaction.created_at_time.timestamp_nanos < min_valid_timestamp {
                                Err("Expired transfer timestamp")?
                            }

                            return Ok(()); // success
                        }
                        _ => {} // ⚠️ Ignore other operation types like Mint, Burn, Approve, etc.
                      
                    },
                    None => Err("No transactions in block")?,
                }
            }

            Err(format!("Failed to verify {} transfer block id {}", token.symbol(), block_id))?
        }
        Err(e) => Err(e.1)?,
    }
}