use crate::stable_memory::{POOLS, TOKENS, TRANSFERS, LP_TOKEN_MAP};
 use crate::pool::stable_pool::StablePool;
use crate::pool::stable_pool::StablePoolId;
use crate::token::stable_token::StableTokenId;
use crate::token::stable_token::StableToken;
use ic_stable_structures::StableBTreeMap;
use crate::transfers::stable_transfer::StableTransfer;
use crate::transfers::tx_id::TxId;
use crate::helpers::math_helpers::{nat_add, nat_multiply, nat_divide, nat_to_decimal_precision};
use crate::ic::general::get_time;
use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};
use ic_cdk::{query, update};
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

// Calculate volume for a specific pool and time period
#[query]
/// Calculates the trading volume, fees, and transaction count for a specific pool over a given time period (in hours).
/// Returns a `PoolVolume` struct with detailed breakdowns for 24h and 7d windows, depending on the `hours` argument.
///
/// # Arguments
/// * `pool_id` - The unique identifier of the pool to analyze.
/// * `hours` - The time window (in hours) for which to calculate the statistics. Typically 24 (1 day) or 168 (7 days).
///
/// # Returns
/// * `Ok(PoolVolume)` with the calculated statistics if successful.
/// * `Err(String)` if the pool is not found or another error occurs.
pub fn calculate_pool_volume(pool_id: u32, hours: u64) -> Result<PoolVolume, String> {
    // Get the current time in nanoseconds.
    let current_time = get_time();
    // Calculate the lower bound timestamp for the time window.
    // Only transfers with a timestamp >= time_threshold will be considered.
    let time_threshold = current_time - (hours * 60 * 60 * 1_000_000_000); // hours -> nanoseconds

    // Access the stable memory maps for transfers, tokens, and pools.
    TRANSFERS.with(|transfers| {
        TOKENS.with(|tokens| {
            POOLS.with(|pools| {
                let transfers_map = transfers.borrow();
                let tokens_map = tokens.borrow();
                let pools_map = pools.borrow();

                // Attempt to retrieve the pool by its ID.
                let pool = pools_map.get(&StablePoolId(pool_id))
                    .ok_or("Pool not found")?;

                // Initialize accumulators for volume, fees, and transaction count.
                let mut volume_usd: f64 = 0.0;
                let mut fees_usd: f64 = 0.0;
                let mut transaction_count: u32 = 0;

                // Iterate over all transfers in the system.
                // For each transfer, check if it falls within the time window and is relevant to the pool.
                for (_, transfer) in transfers_map.iter() {
                    // Only consider transfers within the specified time window.
                    if transfer.ts >= time_threshold {
                        // Attempt to get the token metadata for this transfer.
                        if let Some(token) = tokens_map.get(&StableTokenId(transfer.token_id)) {
                            // Check if the transfer's token is one of the pool's tokens.
                            if transfer.token_id == pool.token_id_0 || transfer.token_id == pool.token_id_1 {
                                // Convert the transfer amount to USD using the token's price and decimals.
                                let amount_usd = token_amount_to_usd(&transfer.amount, &token);
                                
                                // For swaps, we count the input amount as volume
                                if transfer.is_send {
                                    // Add the USD value to the total volume.
                                    volume_usd += amount_usd;
                                    
                                    // Calculate fees (using your lp_fee_bps)
                                    let fee_rate = pool.lp_fee_bps as f64 / 10000.0; // Convert basis points to percentage
                                    fees_usd += amount_usd * fee_rate;
                                    
                                    transaction_count += 1;
                                }
                            }
                        }
                    }
                }

                // Get the pool's symbol for reporting.
                let symbol = pool.name();

                // Depending on the requested time window, populate the appropriate fields in PoolVolume.
                // Only one of the 24h or 7d fields will be filled per call; the other is set to zero.
                if hours == 24 {
                    Ok(PoolVolume {
                        pool_id,
                        symbol,
                        volume_24h_usd: volume_usd,
                        volume_7d_usd: 0.0, // 7d volume not calculated in this call
                        fees_24h_usd: fees_usd,
                        fees_7d_usd: 0.0,
                        transactions_24h: transaction_count,
                        transactions_7d: 0,
                    })
                } else if hours == 168 { // 7 days
                    Ok(PoolVolume {
                        pool_id,
                        symbol,
                        volume_24h_usd: 0.0, // Will be calculated separately
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



// #[query]
// pub fn calculate_pool_volume_improved(pool_id: u32, hours: u64) -> Result<PoolVolume, String> {
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

//                 // Group transfers by transaction ID
//                 let mut transfers_by_tx: std::collections::HashMap<TxId, Vec<&StableTransfer>> = std::collections::HashMap::new();
                
//                 // Collect relevant transfers (just push the reference, no clone needed)
//                 for (_, transfer) in transfers_map.iter() {
//                     if transfer.ts >= time_threshold &&
//                        (transfer.token_id == pool.token_id_0 || transfer.token_id == pool.token_id_1) {
//                         transfers_by_tx
//                             .entry(transfer.tx_id.clone())
//                             .or_insert(Vec::new())
//                             .push(&transfer);  // Just push the reference
//                     }
//                 }

//                 // Process each transaction
//                 for (tx_id, transfers_in_tx) in transfers_by_tx {
//                     // Skip single-transfer transactions (not swaps)
//                     if transfers_in_tx.len() != 2 {
//                         continue;
//                     }

//                     // For swaps, we should have exactly 2 transfers: one input and one output
//                     let transfer1 = &transfers_in_tx[0];
//                     let transfer2 = &transfers_in_tx[1];

//                     // Identify which is input and which is output
//                     let (input_transfer, output_transfer) = if !transfer1.is_send && transfer2.is_send {
//                         (transfer1, transfer2)
//                     } else if transfer1.is_send && !transfer2.is_send {
//                         (transfer2, transfer1)
//                     } else {
//                         continue; // Not a valid swap pattern
//                     };

//                     // Make sure it's a cross-token swap
//                     if input_transfer.token_id == output_transfer.token_id {
//                         continue;
//                     }

//                     // Get token objects
//                     if let (Some(input_token), Some(output_token)) = (
//                         tokens_map.get(&StableTokenId(input_transfer.token_id)),
//                         tokens_map.get(&StableTokenId(output_transfer.token_id))
//                     ) {
//                         // Calculate volume using INPUT amount (standard practice)
//                         let input_amount_usd = token_amount_to_usd(&input_transfer.amount, &input_token);
                        
//                         volume_usd += input_amount_usd;
                        
//                         // Calculate fees
//                         let fee_rate = pool.lp_fee_bps as f64 / 10000.0;
//                         fees_usd += input_amount_usd * fee_rate;
                        
//                         transaction_count += 1;
//                     }
//                 }

//                 let symbol = pool.name();

//                 // Return appropriate response based on hours
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
pub fn calculate_pool_volume_simple(pool_id: u32, hours: u64) -> Result<PoolVolume, String> {
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

                // SIMPLE: Just count outgoing transfers
                for (_, transfer) in transfers_map.iter() {
                    // Check time window
                    if transfer.ts < time_threshold {
                        continue;
                    }
                    
                    // Check if it's a pool token
                    if transfer.token_id != pool.token_id_0 && transfer.token_id != pool.token_id_1 {
                        continue;
                    }
                    
                    // Only count outgoing transfers (user sending to pool)
                    if !transfer.is_send {
                        continue;
                    }

                    // Calculate USD value
                    if let Some(token) = tokens_map.get(&StableTokenId(transfer.token_id)) {
                        let amount_usd = token_amount_to_usd(&transfer.amount, &token);
                        volume_usd += amount_usd;
                        
                        // Calculate fees
                        let fee_rate = pool.lp_fee_bps as f64 / 10000.0;
                        fees_usd += amount_usd * fee_rate;
                        
                        transaction_count += 1;
                    }
                }

                let symbol = pool.name();

                // Return response
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


#[query]
pub fn calculate_pool_volume1(pool_id: u32, hours: u64) -> Result<PoolVolume, String> {
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

                // SIMPLE APPROACH: Count outgoing transfers as volume
                for (_, transfer) in transfers_map.iter() {
                    // Check time window and pool relevance
                    if transfer.ts < time_threshold {
                        continue;
                    }
                    
                    if transfer.token_id != pool.token_id_0 && transfer.token_id != pool.token_id_1 {
                        continue;
                    }
                    
                    // Only count outgoing transfers (is_send = true)
                    if !transfer.is_send {
                        continue;
                    }

                    // Get token and calculate USD value
                    if let Some(token) = tokens_map.get(&StableTokenId(transfer.token_id)) {
                        let amount_usd = token_amount_to_usd(&transfer.amount, &token);
                        volume_usd += amount_usd;
                        
                        // Calculate fees
                        let fee_rate = pool.lp_fee_bps as f64 / 10000.0;
                        fees_usd += amount_usd * fee_rate;
                        
                        transaction_count += 1;
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