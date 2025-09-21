use crate::stable_memory::{POOLS, TOKENS, TRANSFERS};
 use crate::pool::stable_pool::StablePool;
use crate::pool::stable_pool::StablePoolId;
use crate::token::stable_token::StableTokenId;
use crate::token::stable_token::StableToken;
use ic_stable_structures::StableBTreeMap;
use crate::transfers::stable_transfer::StableTransfer;
//use crate::transfers::tx_id::TxId;
use crate::pool_analytics::analytics_storage::{record_pool_snapshot};

use crate::helpers::math_helpers::{nat_add};
use crate::ic::general::get_time;
use crate::transfers::stable_transfer::TransferType;
use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};
use ic_cdk::{query};
use num::ToPrimitive;
use ic_stable_structures::Memory; // <-- import the trait
// Analytics response types
#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct PoolTVL {
    pub pool_id: u32,
    pub symbol: String,
    pub token_0_symbol: String,
    pub token_1_symbol: String,
    pub tvl_usd: f64,
    pub balance_0: Nat,
    pub balance_1: Nat,
    pub token_0_value_usd: f64,
    pub token_1_value_usd: f64,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct PoolVolume {
    pub pool_id: u32,
    pub symbol: String,
    pub volume_24h_usd: f64,
    pub volume_7d_usd: f64,
    pub fees_24h_usd: f64,
    pub fees_7d_usd: f64,
    pub transactions_24h: u32,
    pub transactions_7d: u32,
}


#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolStats {
    pub total_tvl_usd: f64,
    pub total_volume_24h_usd: f64,
    pub total_volume_7d_usd: f64,
    pub total_fees_24h_usd: f64,
    pub total_fees_7d_usd: f64,
    pub total_pools: u32,
    pub active_pools: u32,
    pub total_transactions_24h: u32,
    pub total_transactions_7d: u32,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetrics {
    pub pool_id: u32,
    pub symbol: String,
    pub tvl: PoolTVL,
    pub volume: PoolVolume,
    pub apy: f64,
    pub utilization: f64,
}

// Price oracle - you can replace this with your actual XRC integration
fn get_token_price_usd(token: &StableToken) -> f64 {
    // This should integrate with your XRC canister
    // For now, returning mock prices based on symbol
    match token.symbol().as_str() {
        "ICP" => 12.50,
        "BELLA" => 7.89,  // Your EGX30 token
        "TOMMY" => 1.02,  // Your Treasury token
        "USDC" => 1.00,
        "FXMX" => 0.50,
        "ckUSDT" => 1.00,
        _ => 0.0,
    }
}

// Convert token amount to USD value
fn token_amount_to_usd(amount: &Nat, token: &StableToken) -> f64 {
    let price_usd = get_token_price_usd(token); // Get the USD price for the token
    let amount_f64 = amount.0.to_f64().unwrap_or(0.0); // Convert the Nat (BigUint) to f64
    let decimals_factor = 10_f64.powi(token.decimals() as i32); // Calculate 10^decimals for normalization
    
    (amount_f64 / decimals_factor) * price_usd
}

// Calculate TVL for a specific pool
#[query]
pub fn calculate_pool_tvl(pool_id: u32) -> Result<PoolTVL, String> {
    POOLS.with(|pools| {
        TOKENS.with(|tokens| {
            let pools_map = pools.borrow();
            let tokens_map = tokens.borrow();
            
            if let Some(pool) = pools_map.get(&StablePoolId(pool_id)) {
                // Get tokens
                // TODO: Verify if token_id_0 and token_id_1 need type conversion
                let token_0 = tokens_map.get(&StableTokenId(pool.token_id_0))
                .ok_or("Token 0 not found")?;
            let token_1 = tokens_map.get(&StableTokenId(pool.token_id_1))
                .ok_or("Token 1 not found")?;
                
                // Calculate total reserves (balance + fees)
                let total_balance_0 = nat_add(&pool.balance_0, &pool.lp_fee_0);
                let total_balance_1 = nat_add(&pool.balance_1, &pool.lp_fee_1);
                
                // Convert to USD values
                let token_0_value_usd = token_amount_to_usd(&total_balance_0, &token_0);
                let token_1_value_usd = token_amount_to_usd(&total_balance_1, &token_1);
                let tvl_usd = token_0_value_usd + token_1_value_usd;


                
                Ok(PoolTVL {
                    pool_id,
                    symbol: pool.name(),
                    token_0_symbol: token_0.symbol(),
                    token_1_symbol: token_1.symbol(),
                    tvl_usd,
                    balance_0: total_balance_0,
                    balance_1: total_balance_1,
                    token_0_value_usd,
                    token_1_value_usd,
                })
            } else {
                Err("Pool not found".to_string())
            }
        })
    })
}


// Calculate total protocol TVL
#[query]
pub fn calculate_total_tvl() -> f64 {
    POOLS.with(|pools| {
        TOKENS.with(|tokens| {
            let pools_map = pools.borrow();
            let tokens_map = tokens.borrow();
            let mut total_tvl = 0.0;
            
            // This loop iterates over all pools in the pools_map.
            // For each pool, it tries to fetch the two tokens associated with the pool (token_0 and token_1) from the tokens_map.
            // If both tokens are found, it calculates the total balance for each token by adding the pool's balance and its accumulated LP fees.
            // Then, it converts these token balances to their USD value using the token_amount_to_usd function.
            // The sum of the USD values of both tokens is added to the running total_tvl, which represents the total value locked across all pools.
            for (_, pool) in pools_map.iter() {
                if let (Some(token_0), Some(token_1)) = (
                    tokens_map.get(&StableTokenId(pool.token_id_0)),
                    tokens_map.get(&StableTokenId(pool.token_id_1))
                ) {
                    let total_balance_0 = nat_add(&pool.balance_0, &pool.lp_fee_0);
                    let total_balance_1 = nat_add(&pool.balance_1, &pool.lp_fee_1);
                    
                    let token_0_value = token_amount_to_usd(&total_balance_0, &token_0);
                    let token_1_value = token_amount_to_usd(&total_balance_1, &token_1);
                    
                    total_tvl += token_0_value + token_1_value;
                }
            }
            
            total_tvl
        })
    })
}

//helper function for get_all_pools_tvl
fn calculate_pool_tvl_internal<M: Memory>(
    pool: &StablePool,
    tokens_map: &StableBTreeMap<StableTokenId, StableToken, M>,
) -> Result<PoolTVL, String> {
    // tokens_map is keyed by StableTokenId -> wrap the u32 ids
    let token_0 = tokens_map
        .get(&StableTokenId(pool.token_id_0))
        .ok_or("Token 0 not found")?;
    let token_1 = tokens_map
        .get(&StableTokenId(pool.token_id_1))
        .ok_or("Token 1 not found")?;

    // include uncollected LP fees in TVL
    let total_balance_0 = nat_add(&pool.balance_0, &pool.lp_fee_0);
    let total_balance_1 = nat_add(&pool.balance_1, &pool.lp_fee_1);

    // your price + USD math (keep precise internally; f64 only at the end)
    let token_0_value_usd = token_amount_to_usd(&total_balance_0, &token_0);
    let token_1_value_usd = token_amount_to_usd(&total_balance_1, &token_1);
    let tvl_usd = token_0_value_usd + token_1_value_usd;

    Ok(PoolTVL {
        pool_id: pool.pool_id,
        symbol: pool.name(),
        token_0_symbol: token_0.symbol(),
        token_1_symbol: token_1.symbol(),
        tvl_usd,
        balance_0: total_balance_0,
        balance_1: total_balance_1,
        token_0_value_usd,
        token_1_value_usd,
    })
}

//Sort descending by TVL
#[query]
pub fn get_all_pools_tvl() -> Vec<PoolTVL> {
    POOLS.with(|pools| {
        TOKENS.with(|tokens| {
            let pools_map = pools.borrow();
            let tokens_map = tokens.borrow(); // Ref<StableBTreeMap<StableTokenId, StableToken, _>>
            let tokens_view: &StableBTreeMap<StableTokenId, StableToken, _> = &*tokens_map; // <-- deref Ref

            let mut rows = Vec::new();
            for (_id, pool) in pools_map.iter() {
                if let Ok(row) = calculate_pool_tvl_internal(&pool, tokens_view) {
                    rows.push(row);
                }
            }

            rows.sort_by(|a, b| b.tvl_usd.partial_cmp(&a.tvl_usd).unwrap_or(std::cmp::Ordering::Equal));
            rows
        })
    })
}

#[query]
pub fn calculate_pool_volume(pool_id: u32, hours: u64) -> Result<PoolVolume, String> {
    let current_time = get_time();
    let time_threshold = current_time - (hours * 60 * 60 * 1_000_000_000);

    TRANSFERS.with(|transfers| {
        TOKENS.with(|tokens| {
            POOLS.with(|pools| {
                let transfers_map = transfers.borrow();
                let tokens_map = tokens.borrow();
                let pools_map = pools.borrow();

                let pool = pools_map.get(&StablePoolId(pool_id))
                    .ok_or("Pool not found")?;

                let mut volume_usd: f64 = 0.0;
                let mut fees_usd: f64 = 0.0;
                let mut transaction_count: u32 = 0;

                // Collect relevant transfers for this pool
                let mut pool_transfers = Vec::new();
                for (_, transfer) in transfers_map.iter() {
                    if transfer.ts >= time_threshold &&
                       matches!(transfer.transfer_type, TransferType::Swap) &&
                       (transfer.token_id == pool.token_id_0 || transfer.token_id == pool.token_id_1) {
                        pool_transfers.push(transfer);
                    }
                }

                // Sort by timestamp
                pool_transfers.sort_by_key(|t| t.ts);

                // Group transfers by exact timestamp (nanosecond precision)
                let mut timestamp_groups: std::collections::HashMap<u64, Vec<&StableTransfer>> = 
                    std::collections::HashMap::new();

                for transfer in &pool_transfers {
                    timestamp_groups.entry(transfer.ts).or_default().push(transfer);
                }

                // Process each timestamp group to find valid swaps
                for (_timestamp, transfers_at_time) in timestamp_groups {
                    // Look for input/output pairs at the same timestamp
                    let mut processed_transfers = std::collections::HashSet::new();

                    for i in 0..transfers_at_time.len() {
                        if processed_transfers.contains(&i) {
                            continue;
                        }

                        let transfer1 = transfers_at_time[i];
                        
                        // Look for a matching transfer that forms a valid swap
                        for j in (i + 1)..transfers_at_time.len() {
                            if processed_transfers.contains(&j) {
                                continue;
                            }

                            let transfer2 = transfers_at_time[j];

                            // Check if these two transfers form a valid swap for THIS pool
                            let is_valid_pool_swap = 
                                (transfer1.is_send && !transfer2.is_send && 
                                  transfer1.token_id == pool.token_id_0 && 
                                  transfer2.token_id == pool.token_id_1) ||
                                 (transfer1.is_send && !transfer2.is_send && 
                                  transfer1.token_id == pool.token_id_1 && 
                                  transfer2.token_id == pool.token_id_0) ||
                                 (!transfer1.is_send && transfer2.is_send && 
                                  transfer2.token_id == pool.token_id_0 && 
                                  transfer1.token_id == pool.token_id_1) ||
                                 (!transfer1.is_send && transfer2.is_send && 
                                  transfer2.token_id == pool.token_id_1 && 
                                  transfer1.token_id == pool.token_id_0);

                            if is_valid_pool_swap {
                                // Determine which is input and which is output
                                let (input_transfer, _output_transfer) = if transfer1.is_send {
                                    (transfer1, transfer2)
                                } else {
                                    (transfer2, transfer1)
                                };

                                // Calculate volume from input transfer
                                if let Some(input_token) = tokens_map.get(&StableTokenId(input_transfer.token_id)) {
                                    let swap_volume_usd = token_amount_to_usd(&input_transfer.amount, &input_token);
                                    
                                    volume_usd += swap_volume_usd;
                                    
                                    let fee_rate = pool.lp_fee_bps as f64 / 10000.0;
                                    fees_usd += swap_volume_usd * fee_rate;
                                    
                                    transaction_count += 1;
                                }

                                // Mark both transfers as processed
                                processed_transfers.insert(i);
                                processed_transfers.insert(j);
                                break;
                            }
                        }
                    }
                }

                let symbol = pool.name();

                if hours == 24 {
                    Ok(PoolVolume {
                        pool_id,
                        symbol,
                        volume_24h_usd: volume_usd,
                        volume_7d_usd: 0.0,
                        fees_24h_usd: fees_usd,
                        fees_7d_usd: 0.0,
                        transactions_24h: transaction_count,
                        transactions_7d: 0,
                    })
                } else if hours == 168 {
                    Ok(PoolVolume {
                        pool_id,
                        symbol,
                        volume_24h_usd: 0.0,
                        volume_7d_usd: volume_usd,
                        fees_24h_usd: 0.0,
                        fees_7d_usd: fees_usd,
                        transactions_24h: 0,
                        transactions_7d: transaction_count,
                    })
                } else {
                    Ok(PoolVolume {
                        pool_id,
                        symbol,
                        volume_24h_usd: volume_usd,
                        volume_7d_usd: 0.0,
                        fees_24h_usd: fees_usd,
                        fees_7d_usd: 0.0,
                        transactions_24h: transaction_count,
                        transactions_7d: 0,
                    })
                }
            })
        })
    })
}


// Add this debug function to your analytics file
// #[query]
// pub fn debug_pool_volume_zero(pool_id: u32, hours: u64) -> Result<String, String> {
//     let current_time = get_time();
//     let time_threshold = current_time - (hours * 60 * 60 * 1_000_000_000);
    
//     TRANSFERS.with(|transfers| {
//         TOKENS.with(|tokens| {
//             POOLS.with(|pools| {
//                 let transfers_map = transfers.borrow();
//              //   let tokens_map = tokens.borrow();
//                 let pools_map = pools.borrow();

//                 let pool = pools_map.get(&StablePoolId(pool_id))
//                     .ok_or("Pool not found")?;

//                 let mut debug_info = Vec::new();
                
//                 debug_info.push(format!("=== DEBUGGING POOL {} VOLUME ===", pool_id));
//                 debug_info.push(format!("Pool: {} (token_0: {}, token_1: {})", 
//                     pool.name(), pool.token_id_0, pool.token_id_1));
//                 debug_info.push(format!("Time threshold: {} (current: {})", time_threshold, current_time));
//                 debug_info.push(format!("Looking for transfers after: {}", time_threshold));
                
//                 // Step 1: Count all transfers in the time window
//                 let mut total_transfers = 0;
//                 let mut swap_transfers = 0;
//                 let mut pool_related_transfers = 0;
                
//                 for (_, transfer) in transfers_map.iter() {
//                     if transfer.ts >= time_threshold {
//                         total_transfers += 1;
                        
//                         if matches!(transfer.transfer_type, TransferType::Swap) {
//                             swap_transfers += 1;
                            
//                             if transfer.token_id == pool.token_id_0 || transfer.token_id == pool.token_id_1 {
//                                 pool_related_transfers += 1;
                                
//                                 // Show details of first 5 relevant transfers
//                                 if pool_related_transfers <= 5 {
//                                     debug_info.push(format!(
//                                         "Transfer {}: token_id={}, amount={}, is_send={}, tx_id={:?}, ts={}", 
//                                         pool_related_transfers, 
//                                         transfer.token_id, 
//                                         transfer.amount, 
//                                         transfer.is_send,
//                                         transfer.tx_id,
//                                         transfer.ts
//                                     ));
//                                 }
//                             }
//                         }
//                     }
//                 }
                
//                 debug_info.push(format!("Total transfers in time window: {}", total_transfers));
//                 debug_info.push(format!("Swap transfers in time window: {}", swap_transfers));
//                 debug_info.push(format!("Pool-related swap transfers: {}", pool_related_transfers));
                
//                 if pool_related_transfers == 0 {
//                     debug_info.push("❌ NO POOL-RELATED SWAP TRANSFERS FOUND!".to_string());
//                     debug_info.push("Possible issues:".to_string());
//                     debug_info.push("1. No swaps in the time window".to_string());
//                     debug_info.push("2. Transfers not marked as TransferType::Swap".to_string());
//                     debug_info.push("3. Wrong token IDs in transfers".to_string());
                    
//                     return Ok(debug_info.join("\n"));
//                 }
                
//                 // Step 2: Analyze transaction groupings
//                 let mut tx_id_groups: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
                
//                 for (_, transfer) in transfers_map.iter() {
//                     if transfer.ts >= time_threshold &&
//                        matches!(transfer.transfer_type, TransferType::Swap) &&
//                        (transfer.token_id == pool.token_id_0 || transfer.token_id == pool.token_id_1) {
                        
//                         let tx_id_str = format!("{:?}", transfer.tx_id);
//                         let transfer_info = format!("token_{}_{}_{}", 
//                             transfer.token_id, 
//                             if transfer.is_send { "OUT" } else { "IN" },
//                             transfer.amount
//                         );
                        
//                         tx_id_groups.entry(tx_id_str).or_default().push(transfer_info);
//                     }
//                 }
                
//                 debug_info.push(format!("=== TRANSACTION GROUPS ({}) ===", tx_id_groups.len()));
                
//                 let mut valid_swaps = 0;
//                 for (tx_id, transfers_in_tx) in tx_id_groups.iter() {
//                     debug_info.push(format!("TX {}: {} transfers", tx_id, transfers_in_tx.len()));
                    
//                     for transfer_info in transfers_in_tx {
//                         debug_info.push(format!("  - {}", transfer_info));
//                     }
                    
//                     // Check if this could be a valid swap
//                     if transfers_in_tx.len() >= 2 {
//                         let has_input = transfers_in_tx.iter().any(|t| t.contains("_OUT_"));
//                         let has_output = transfers_in_tx.iter().any(|t| t.contains("_IN_"));
//                         let has_both_tokens = transfers_in_tx.iter().any(|t| t.starts_with(&format!("token_{}", pool.token_id_0))) &&
//                                             transfers_in_tx.iter().any(|t| t.starts_with(&format!("token_{}", pool.token_id_1)));
                        
//                         debug_info.push(format!("  Analysis: input={}, output={}, both_tokens={}", 
//                             has_input, has_output, has_both_tokens));
                        
//                         if has_input && has_output && has_both_tokens {
//                             valid_swaps += 1;
//                             debug_info.push("  ✅ VALID SWAP DETECTED".to_string());
//                         } else {
//                             debug_info.push("  ❌ Invalid swap pattern".to_string());
//                         }
//                     }
//                 }
                
//                 debug_info.push(format!("=== SUMMARY ==="));
//                 debug_info.push(format!("Valid swaps found: {}", valid_swaps));
                
//                 if valid_swaps == 0 {
//                     debug_info.push("❌ NO VALID SWAPS FOUND!".to_string());
//                     debug_info.push("Common issues:".to_string());
//                     debug_info.push("1. Input/output transfers have different tx_id values".to_string());
//                     debug_info.push("2. Missing input or output transfer".to_string());
//                     debug_info.push("3. Both transfers are same token (not a cross-token swap)".to_string());
//                     debug_info.push("4. is_send field not set correctly".to_string());
//                 }
                
//                 Ok(debug_info.join("\n"))
//             })
//         })
//     })
// }




// #[query]
// pub fn calculate_pool_volume_simple(pool_id: u32, hours: u64) -> Result<PoolVolume, String> {
//     let current_time = get_time();
//     let time_threshold = current_time - (hours * 60 * 60 * 1_000_000_000);
    
//     TRANSFERS.with(|transfers| {
//         TOKENS.with(|tokens| {
//             POOLS.with(|pools| {
//                 let transfers_map = transfers.borrow();
//                 let tokens_map = tokens.borrow();
//                 let pools_map = pools.borrow();

//                 let pool = pools_map.get(&StablePoolId(pool_id))
//                     .ok_or("Pool not found")?;

//                 let mut volume_usd: f64 = 0.0;
//                 let mut fees_usd: f64 = 0.0;
//                 let mut transaction_count: u32 = 0;

//                 // SIMPLE: Just count outgoing transfers
//                 for (_, transfer) in transfers_map.iter() {
//                     // Check time window
//                     if transfer.ts < time_threshold {
//                         continue;
//                     }
                    
//                     // Check if it's a pool token
//                     if transfer.token_id != pool.token_id_0 && transfer.token_id != pool.token_id_1 {
//                         continue;
//                     }
                    
//                     // Only count outgoing transfers (user sending to pool)
//                     if !transfer.is_send {
//                         continue;
//                     }

//                     // Calculate USD value
//                     if let Some(token) = tokens_map.get(&StableTokenId(transfer.token_id)) {
//                         let amount_usd = token_amount_to_usd(&transfer.amount, &token);
//                         volume_usd += amount_usd;
                        
//                         // Calculate fees
//                         let fee_rate = pool.lp_fee_bps as f64 / 10000.0;
//                         fees_usd += amount_usd * fee_rate;
                        
//                         transaction_count += 1;
//                     }
//                 }

//                 let symbol = pool.name();

//                 // Return response
//                 if hours == 24 {
//                     Ok(PoolVolume {
//                         pool_id,
//                         symbol,
//                         volume_24h_usd: volume_usd,
//                         volume_7d_usd: 0.0,
//                         fees_24h_usd: fees_usd,
//                         fees_7d_usd: 0.0,
//                         transactions_24h: transaction_count,
//                         transactions_7d: 0,
//                     })
//                 } else if hours == 168 {
//                     Ok(PoolVolume {
//                         pool_id,
//                         symbol,
//                         volume_24h_usd: 0.0,
//                         volume_7d_usd: volume_usd,
//                         fees_24h_usd: 0.0,
//                         fees_7d_usd: fees_usd,
//                         transactions_24h: 0,
//                         transactions_7d: transaction_count,
//                     })
//                 } else {
//                     Ok(PoolVolume {
//                         pool_id,
//                         symbol,
//                         volume_24h_usd: volume_usd,
//                         volume_7d_usd: 0.0,
//                         fees_24h_usd: fees_usd,
//                         fees_7d_usd: 0.0,
//                         transactions_24h: transaction_count,
//                         transactions_7d: 0,
//                     })
//                 }
//             })
//         })
//     })
// }

#[query]
pub fn calculate_pool_volume_with_tx_grouping(pool_id: u32, hours: u64) -> Result<PoolVolume, String> {
    let current_time = get_time();
    let time_threshold = current_time - (hours * 60 * 60 * 1_000_000_000);
    
    TRANSFERS.with(|transfers| {
        TOKENS.with(|tokens| {
            POOLS.with(|pools| {
                let transfers_map = transfers.borrow();
                let tokens_map = tokens.borrow();
                let pools_map = pools.borrow();

                let pool = pools_map.get(&StablePoolId(pool_id))
                    .ok_or("Pool not found")?;

                let mut volume_usd: f64 = 0.0;
                let mut fees_usd: f64 = 0.0;
                let mut transaction_count: u32 = 0;

                // First pass: collect transaction IDs that involve this pool
                let mut relevant_tx_ids = std::collections::HashSet::new();
                
                for (_, transfer) in transfers_map.iter() {
                    if transfer.ts >= time_threshold &&
                       (transfer.token_id == pool.token_id_0 || transfer.token_id == pool.token_id_1) {
                        relevant_tx_ids.insert(transfer.tx_id.clone());
                    }
                }

                // Second pass: process each relevant transaction
                for tx_id in relevant_tx_ids {
                    let mut transfers_in_tx = Vec::new();
                    
                    // Collect all transfers for this transaction
                    for (_, transfer) in transfers_map.iter() {
                        if transfer.tx_id == tx_id {
                            transfers_in_tx.push(transfer);
                        }
                    }

                    // Look for swap pattern (input + output)
                    if transfers_in_tx.len() == 2 {
                        let transfer1 = &transfers_in_tx[0];
                        let transfer2 = &transfers_in_tx[1];
                        
                        // Check if this is a valid swap (one input, one output)
                        if transfer1.is_send != transfer2.is_send {
                            let (input_transfer, output_transfer) = if !transfer1.is_send {
                                (transfer1, transfer2)
                            } else {
                                (transfer2, transfer1)
                            };

                            // CRITICAL FIX: Check if it's a cross-token swap
                            if input_transfer.token_id != output_transfer.token_id {
                                
                                // DOUBLE CHECK: Both tokens should belong to this pool
                                let valid_swap = 
                                    (input_transfer.token_id == pool.token_id_0 && output_transfer.token_id == pool.token_id_1) ||
                                    (input_transfer.token_id == pool.token_id_1 && output_transfer.token_id == pool.token_id_0);
                                
                                if valid_swap {
                                    // Get tokens and calculate volume
                                    if let Some(input_token) = tokens_map.get(&StableTokenId(input_transfer.token_id)) {
                                        let input_amount_usd = token_amount_to_usd(&input_transfer.amount, &input_token);
                                        volume_usd += input_amount_usd;
                                        
                                        let fee_rate = pool.lp_fee_bps as f64 / 10000.0;
                                        fees_usd += input_amount_usd * fee_rate;
                                        
                                        transaction_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }

                let symbol = pool.name();

                // Return appropriate response
                if hours == 24 {
                    Ok(PoolVolume {
                        pool_id,
                        symbol,
                        volume_24h_usd: volume_usd,
                        volume_7d_usd: 0.0,
                        fees_24h_usd: fees_usd,
                        fees_7d_usd: 0.0,
                        transactions_24h: transaction_count,
                        transactions_7d: 0,
                    })
                } else if hours == 168 {
                    Ok(PoolVolume {
                        pool_id,
                        symbol,
                        volume_24h_usd: 0.0,
                        volume_7d_usd: volume_usd,
                        fees_24h_usd: 0.0,
                        fees_7d_usd: fees_usd,
                        transactions_24h: 0,
                        transactions_7d: transaction_count,
                    })
                } else {
                    Ok(PoolVolume {
                        pool_id,
                        symbol,
                        volume_24h_usd: volume_usd,
                        volume_7d_usd: 0.0,
                        fees_24h_usd: fees_usd,
                        fees_7d_usd: 0.0,
                        transactions_24h: transaction_count,
                        transactions_7d: 0,
                    })
                }
            })
        })
    })
}


// Get comprehensive pool metrics
#[query]
pub fn get_pool_metrics(pool_id: u32) -> Result<PoolMetrics, String> {
    let tvl = calculate_pool_tvl(pool_id)?;
    let volume_24h = calculate_pool_volume(pool_id, 24)?;
    let volume_7d = calculate_pool_volume(pool_id, 168)?;
    
    // Combine volume data
    let volume = PoolVolume {
        pool_id,
        symbol: volume_24h.symbol.clone(),
        volume_24h_usd: volume_24h.volume_24h_usd,
        volume_7d_usd: volume_7d.volume_7d_usd,
        fees_24h_usd: volume_24h.fees_24h_usd,
        fees_7d_usd: volume_7d.fees_7d_usd,
        transactions_24h: volume_24h.transactions_24h,
        transactions_7d: volume_7d.transactions_7d,
    };
    
    // Calculate APY: (daily_fees / tvl) * 365 * 100
    let apy = if tvl.tvl_usd > 0.0 && volume.fees_24h_usd > 0.0 {
        (volume.fees_24h_usd / tvl.tvl_usd) * 365.0 * 100.0
    } else {
        0.0
    };
    
    // Calculate utilization: volume / tvl ratio
    let utilization = if tvl.tvl_usd > 0.0 {
        (volume.volume_24h_usd / tvl.tvl_usd) * 100.0
    } else {
        0.0
    };
    
    Ok(PoolMetrics {
        pool_id,
        symbol: tvl.symbol.clone(),
        tvl,
        volume,
        apy,
        utilization,
    })
}


// Get all pools metrics
#[query]
pub fn get_all_pools_metrics() -> Vec<PoolMetrics> {
    POOLS.with(|pools| {
        let pools_map = pools.borrow();
        let mut pool_metrics = Vec::new();
        
        for (pool_id, _) in pools_map.iter() {
            if let Ok(metrics) = get_pool_metrics(pool_id.0) {
                pool_metrics.push(metrics);
            }
        }
        
        // Sort by TVL descending
        pool_metrics.sort_by(|a, b| b.tvl.tvl_usd.partial_cmp(&a.tvl.tvl_usd).unwrap());
        pool_metrics
    })
}

// Get protocol-wide statistics
#[query]
pub fn get_protocol_stats() -> ProtocolStats {
    let all_metrics = get_all_pools_metrics();
    
    let total_tvl_usd = all_metrics.iter().map(|m| m.tvl.tvl_usd).sum();
    let total_volume_24h_usd = all_metrics.iter().map(|m| m.volume.volume_24h_usd).sum();
    let total_volume_7d_usd = all_metrics.iter().map(|m| m.volume.volume_7d_usd).sum();
    let total_fees_24h_usd = all_metrics.iter().map(|m| m.volume.fees_24h_usd).sum();
    let total_fees_7d_usd = all_metrics.iter().map(|m| m.volume.fees_7d_usd).sum();
    let total_pools = all_metrics.len() as u32;
    let active_pools = all_metrics.iter().filter(|m| m.tvl.tvl_usd > 0.0).count() as u32;
    let total_transactions_24h = all_metrics.iter().map(|m| m.volume.transactions_24h).sum();
    let total_transactions_7d = all_metrics.iter().map(|m| m.volume.transactions_7d).sum();
    
    ProtocolStats {
        total_tvl_usd,
        total_volume_24h_usd,
        total_volume_7d_usd,
        total_fees_24h_usd,
        total_fees_7d_usd,
        total_pools,
        active_pools,
        total_transactions_24h,
        total_transactions_7d,
    }
}

// Get historical TVL data (you might want to store snapshots for this)
//doesnt really store tvl
// #[query]
// pub fn get_historical_tvl(days: u32) -> Vec<(u64, f64)> {
//     // For now, return current TVL. In production, you'd want to store daily snapshots
//     let current_tvl = calculate_total_tvl();
//     let current_time = get_time();
    
//     let mut historical_data = Vec::new();
//     for i in 0..days {
//         let timestamp = current_time - (i as u64 * 24 * 60 * 60 * 1_000_000_000);
//         // In production, you'd fetch actual historical data here
//         historical_data.push((timestamp, current_tvl));
//     }
    
//     historical_data.reverse();
//     historical_data
// }

// Add this function to your existing analytics.rs file

/// Integration function to record current analytics snapshot
#[ic_cdk::update]
pub fn record_current_analytics() -> Result<String, String> {
    
    let pools_tvl = get_all_pools_tvl();
    let mut recorded_count = 0;

    for pool_tvl in pools_tvl {
        // Get volume for this pool
        let volume_24h_usd = match calculate_pool_volume(pool_tvl.pool_id, 24) {
            Ok(vol) => vol.volume_24h_usd,
            Err(_) => 0.0,
        };

        // Record the snapshot using the analytics storage
        match record_pool_snapshot(pool_tvl.pool_id, pool_tvl.tvl_usd, volume_24h_usd) {
            Ok(_) => recorded_count += 1,
            Err(_) => continue,
        }
    }

    Ok(format!("Recorded {} pool analytics snapshots", recorded_count))
}