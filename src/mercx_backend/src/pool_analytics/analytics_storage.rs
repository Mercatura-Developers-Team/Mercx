// analytics_storage.rs
// Simple analytics storage for TVL and Volume data with 30-day retention

use candid::{CandidType};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::cell::RefCell;
use ic_cdk::api::time;

const MAX_DAYS_HISTORY: usize = 30;
const SNAPSHOTS_PER_DAY: usize = 24; // Hourly snapshots
const MAX_SNAPSHOTS: usize = MAX_DAYS_HISTORY * SNAPSHOTS_PER_DAY;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct PoolSnapshot {
    pub timestamp: u64,
    pub pool_id: u32,
    pub tvl_usd: f64,
    pub volume_24h_usd: f64,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, Default)]
pub struct PoolTimeSeries {
    pub pool_id: u32,
    pub snapshots: VecDeque<PoolSnapshot>,
}

impl PoolTimeSeries {
    pub fn new(pool_id: u32) -> Self {
        Self {
            pool_id,
            snapshots: VecDeque::with_capacity(MAX_SNAPSHOTS),
        }
    }

    // pub fn add_snapshot(&mut self, tvl_usd: f64, volume_24h_usd: f64) {
    //     let current_time = time();
        
    //     // Remove old snapshots (older than 30 days)
    //     let cutoff_time = current_time.saturating_sub(30 * 24 * 60 * 60 * 1_000_000_000);
    //     while let Some(front) = self.snapshots.front() {
    //         if front.timestamp < cutoff_time {
    //             self.snapshots.pop_front();
    //         } else {
    //             break;
    //         }
    //     }

    //     // Add new snapshot
    //     let snapshot = PoolSnapshot {
    //         timestamp: current_time,
    //         pool_id: self.pool_id,
    //         tvl_usd,
    //         volume_24h_usd,
    //     };
        
    //     self.snapshots.push_back(snapshot);

    //     // Failsafe: keep only MAX_SNAPSHOTS entries
    //     while self.snapshots.len() > MAX_SNAPSHOTS {
    //         self.snapshots.pop_front();
    //     }
    // }

    pub fn add_snapshot(&mut self, tvl_usd: f64, volume_24h_usd: f64) {
        let current_time = time();
        
        // Add new snapshot first
        let snapshot = PoolSnapshot {
            timestamp: current_time,
            pool_id: self.pool_id,
            tvl_usd,
            volume_24h_usd,
        };
        
        self.snapshots.push_back(snapshot);
        
        // Remove old snapshots ONLY if we exceed max capacity
        // This is safer than time-based removal for now
        while self.snapshots.len() > MAX_SNAPSHOTS {
            self.snapshots.pop_front();
        }
        
        // Alternative: More conservative time-based cleanup (only remove if older than 30 days AND we have many snapshots)
        if self.snapshots.len() > (MAX_SNAPSHOTS / 2) {
            let cutoff_time = current_time.saturating_sub(30 * 24 * 60 * 60 * 1_000_000_000);
            while let Some(front) = self.snapshots.front() {
                if front.timestamp < cutoff_time && self.snapshots.len() > 24 {
                    self.snapshots.pop_front();
                } else {
                    break;
                }
            }
        }
    }

    pub fn get_chart_data(&self, hours: u64) -> Vec<(u64, f64, f64)> {
        let cutoff_time = time().saturating_sub(hours * 60 * 60 * 1_000_000_000);
        self.snapshots
            .iter()
            .filter(|snapshot| snapshot.timestamp >= cutoff_time)
            .map(|s| (s.timestamp, s.tvl_usd, s.volume_24h_usd))
            .collect()
    }
}

// Global in-memory storage
thread_local! {
    static ANALYTICS_DATA: RefCell<HashMap<u32, PoolTimeSeries>> = RefCell::new(HashMap::new());
}

// Public API functions

/// Record current TVL and volume for a pool
#[ic_cdk::update]
pub fn record_pool_snapshot(pool_id: u32, tvl_usd: f64, volume_24h_usd: f64) -> Result<String, String> {
    ANALYTICS_DATA.with(|data| {
        let mut analytics = data.borrow_mut();
        let pool_series = analytics
            .entry(pool_id)
            .or_insert_with(|| PoolTimeSeries::new(pool_id));
        
        pool_series.add_snapshot(tvl_usd, volume_24h_usd);
        
        Ok(format!("Recorded snapshot for pool {}: TVL=${:.2}, Volume=${:.2}", 
                  pool_id, tvl_usd, volume_24h_usd))
    })
}

/// Record snapshots for all pools at once
#[ic_cdk::update] 
pub fn record_all_pools_snapshot() -> Result<String, String> {
    use crate::pool_analytics::analytics::{get_all_pools_tvl, calculate_pool_volume};
    
    let pools_tvl = get_all_pools_tvl();
    let mut recorded_count = 0;

    for pool_tvl in pools_tvl {
        // Get volume for this pool
        let volume_result = calculate_pool_volume(pool_tvl.pool_id, 24);
        let volume_24h_usd = match volume_result {
            Ok(vol) => vol.volume_24h_usd,
            Err(_) => 0.0,
        };

        // Record the snapshot
        let _ = record_pool_snapshot(pool_tvl.pool_id, pool_tvl.tvl_usd, volume_24h_usd);
        recorded_count += 1;
    }

    Ok(format!("Recorded {} pool snapshots", recorded_count))
}

/// Get chart data for a specific pool
#[ic_cdk::query]
pub fn get_pool_chart_data(pool_id: u32, hours: u64) -> Vec<(u64, f64, f64)> {
    ANALYTICS_DATA.with(|data| {
        let analytics = data.borrow();
        match analytics.get(&pool_id) {
            Some(pool_series) => pool_series.get_chart_data(hours),
            None => Vec::new(),
        }
    })
}

/// Get current snapshot count for a pool (for debugging)
#[ic_cdk::query]
pub fn get_pool_snapshot_count(pool_id: u32) -> u32 {
    ANALYTICS_DATA.with(|data| {
        let analytics = data.borrow();
        match analytics.get(&pool_id) {
            Some(pool_series) => pool_series.snapshots.len() as u32,
            None => 0,
        }
    })
}

/// Get the latest snapshot for a pool
#[ic_cdk::query]
pub fn get_latest_pool_snapshot(pool_id: u32) -> Option<PoolSnapshot> {
    ANALYTICS_DATA.with(|data| {
        let analytics = data.borrow();
        analytics.get(&pool_id)
            .and_then(|pool_series| pool_series.snapshots.back().cloned())
    })
}

/// Clean up all analytics data (for testing)
#[ic_cdk::update]
pub fn clear_analytics_data() -> String {
    ANALYTICS_DATA.with(|data| {
        data.borrow_mut().clear();
    });
    "Analytics data cleared".to_string()
}

/// Get analytics summary for all pools 
#[ic_cdk::query]
pub fn get_analytics_summary() -> Vec<(u32, u32, f64, f64)> {
    ANALYTICS_DATA.with(|data| {
        let analytics = data.borrow();
        analytics.iter().map(|(pool_id, series)| {
            let latest = series.snapshots.back();
            let (latest_tvl, latest_volume) = match latest {
                Some(snapshot) => (snapshot.tvl_usd, snapshot.volume_24h_usd),
                None => (0.0, 0.0),
            };
            (*pool_id, series.snapshots.len() as u32, latest_tvl, latest_volume)
        }).collect()
    })
}

// // Timer-based cron job to automatically record snapshots
// #[ic_cdk::update]
// pub fn analytics_cron_job() -> Result<String, String> {
//     record_all_pools_snapshot()
// }


