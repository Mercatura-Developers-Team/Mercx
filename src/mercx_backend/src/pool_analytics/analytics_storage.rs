use crate::stable_memory::ANALYTICS_DATA;
use crate::pool_analytics::stable_analytics::{StablePoolTimeSeries, PoolAnalyticsId};
use crate::PoolSnapshot;
use crate::stable_mercx_settings::mercx_settings_map;
use crate::stable_mercx_settings::mercx_settings_map::reset_analytics_map_idx;
use crate::pool_analytics::analytics::get_pool_metrics;
use crate::pool::handlers;
/// Record TVL and volume snapshot for a specific pool
#[ic_cdk::update]
pub fn record_pool_snapshot(pool_id: u32, tvl_usd: f64, volume_24h_usd: f64) -> Result<String, String> {
    ANALYTICS_DATA.with(|data| {
        let mut analytics = data.borrow_mut();
        
        // Get existing or create new with proper ID
        let mut pool_series = match analytics.get(&PoolAnalyticsId(pool_id)) {
            Some(existing) => existing,
            None => {
                // Only increment counter when creating new series
                //warning
                let _new_id = mercx_settings_map::inc_analytics_map_idx();
                StablePoolTimeSeries::new(pool_id)
            }
        };
        
        pool_series.add_snapshot(tvl_usd, volume_24h_usd);
        analytics.insert(PoolAnalyticsId(pool_id), pool_series);
        
        Ok(format!(
            "Recorded snapshot for pool {}: TVL=${:.2}, Volume=${:.2}",
            pool_id, tvl_usd, volume_24h_usd
        ))
    })
}

/// Record snapshots for all pools at once (call this daily via timer)
#[ic_cdk::update]
pub async fn record_all_pools_snapshot() -> Result<String, String> {
    use crate::pool_analytics::analytics::{get_all_pools_tvl, calculate_pool_volume};
    
    let pools_tvl = get_all_pools_tvl();
    let mut recorded_count = 0;

    for pool_tvl in pools_tvl {
        let volume_result = calculate_pool_volume(pool_tvl.pool_id, 24);
        let volume_24h_usd = match volume_result {
            Ok(vol) => vol.volume_24h_usd,
            Err(_) => 0.0,
        };

        let _ = record_pool_snapshot(pool_tvl.pool_id, pool_tvl.tvl_usd, volume_24h_usd);
        recorded_count += 1;
    }

    Ok(format!("Recorded {} pool snapshots", recorded_count))
}

#[ic_cdk::update]
pub async fn record_pool_snapshot2(pool_id: u32) -> Result<String, String> {
    // Get the pool
    let _pool = handlers::get_by_pool_id(pool_id)
        .ok_or("Pool not found")?;
    
    // Calculate TVL from pool metrics
    let metrics = get_pool_metrics(pool_id)?;
    let tvl_usd = metrics.tvl.tvl_usd;
    let volume_24h_usd = metrics.volume.volume_24h_usd;
    
    ANALYTICS_DATA.with(|data| {
        let mut analytics = data.borrow_mut();
        
        let mut pool_series = match analytics.get(&PoolAnalyticsId(pool_id)) {
            Some(existing) => existing,
            None => {
                let _new_id = mercx_settings_map::inc_analytics_map_idx();
                StablePoolTimeSeries::new(pool_id)
            }
        };
        
        pool_series.add_snapshot(tvl_usd, volume_24h_usd);
        analytics.insert(PoolAnalyticsId(pool_id), pool_series);
        
        Ok(format!(
            "Recorded snapshot for pool {}: TVL=${:.2}, Volume=${:.2}",
            pool_id, tvl_usd, volume_24h_usd
        ))
    })
}


/// Get chart data for a specific pool (time-based, in hours)
#[ic_cdk::query]
pub fn get_pool_chart_data(pool_id: u32, hours: u64) -> Vec<(u64, f64, f64)> {
    ANALYTICS_DATA.with(|data| {
        let analytics = data.borrow();
        match analytics.get(&PoolAnalyticsId(pool_id)) {
            Some(pool_series) => pool_series.get_chart_data(hours),
            None => Vec::new(),
        }
    })
}

/// Get daily chart data for a specific pool (day-based, better for daily charts)
/// Returns: Vec<(timestamp, day_number, tvl_usd, volume_24h_usd)>
#[ic_cdk::query]
pub fn get_pool_daily_chart(pool_id: u32, days: u64) -> Vec<(u64, u64, f64, f64)> {
    ANALYTICS_DATA.with(|data| {
        let analytics = data.borrow();
        match analytics.get(&PoolAnalyticsId(pool_id)) {
            Some(pool_series) => pool_series.get_daily_chart_data(days),
            None => Vec::new(),
        }
    })
}

/// Get all snapshots for a pool (all 30 days)
#[ic_cdk::query]
pub fn get_all_pool_snapshots(pool_id: u32) -> Vec<PoolSnapshot> {
    ANALYTICS_DATA.with(|data| {
        let analytics = data.borrow();
        match analytics.get(&PoolAnalyticsId(pool_id)) {
            Some(pool_series) => pool_series.get_all_snapshots(),
            None => Vec::new(),
        }
    })
}

/// Get snapshot count for a pool
#[ic_cdk::query]
pub fn get_pool_snapshot_count(pool_id: u32) -> u32 {
    ANALYTICS_DATA.with(|data| {
        let analytics = data.borrow();
        match analytics.get(&PoolAnalyticsId(pool_id)) {
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
        analytics
            .get(&PoolAnalyticsId(pool_id))
            .and_then(|pool_series| pool_series.snapshots.last().cloned())
    })
}

/// Get analytics summary for all pools
/// Returns: Vec<(pool_id, snapshot_count, latest_tvl, latest_volume)>
#[ic_cdk::query]
pub fn get_analytics_summary() -> Vec<(u32, u32, f64, f64)> {
    ANALYTICS_DATA.with(|data| {
        let analytics = data.borrow();
        analytics
            .iter()
            .map(|(pool_id, series)| {
                let latest = series.snapshots.last();
                let (latest_tvl, latest_volume) = match latest {
                    Some(snapshot) => (snapshot.tvl_usd, snapshot.volume_24h_usd),
                    None => (0.0, 0.0),
                };
                (pool_id.0, series.snapshots.len() as u32, latest_tvl, latest_volume)
            })
            .collect()
    })
}

/// Debug: Get detailed snapshot information with day numbers
/// Returns: Vec<(timestamp, day_number, tvl_usd, volume_24h_usd)>
#[ic_cdk::query]
pub fn debug_pool_snapshots(pool_id: u32) -> Vec<(u64, u64, f64, f64)> {
    ANALYTICS_DATA.with(|data| {
        let analytics = data.borrow();
        match analytics.get(&PoolAnalyticsId(pool_id)) {
            Some(pool_series) => {
                pool_series
                    .snapshots
                    .iter()
                    .map(|s| (s.timestamp, s.day_number, s.tvl_usd, s.volume_24h_usd))
                    .collect()
            }
            None => Vec::new(),
        }
    })
}

//// Verify stable storage is working and show stats
#[ic_cdk::query]
pub fn verify_analytics_persistence() -> String {
    ANALYTICS_DATA.with(|data| {
        let analytics = data.borrow();
        let pool_count = analytics.iter().count();
        let total_snapshots: usize = analytics.iter()
            .map(|(_, series)| series.snapshots.len())
            .sum();
        
        let oldest_snapshot = analytics.iter()
            .filter_map(|(_, series)| series.snapshots.first().map(|s| s.day_number))
            .min();
        
        let newest_snapshot = analytics.iter()
            .filter_map(|(_, series)| series.snapshots.last().map(|s| s.day_number))
            .max();
        
        format!(
            "Stable storage active: {} pools, {} total snapshots. Day range: {:?} to {:?}",
            pool_count, total_snapshots, oldest_snapshot, newest_snapshot
        )
    })
}

/// Clear all analytics data (use with caution!)
#[ic_cdk::update]
pub fn clear_analytics_data() -> String {
    ANALYTICS_DATA.with(|data| {
        let mut analytics = data.borrow_mut();
        let keys: Vec<PoolAnalyticsId> = analytics.iter().map(|(k, _)| k).collect();
        for key in keys {
            analytics.remove(&key);
        }
    });
    "Analytics data cleared".to_string()
}

//cargo build --release --features prod
#[cfg(not(feature = "prod"))]
#[ic_cdk::update]
fn reset_analytics() -> Result<String, String> {
    ANALYTICS_DATA.with(|analytics| {
        analytics.borrow_mut().clear_new(); // `clear_new()` btmsh kolo remove law hanmsh haga specific
    });

    reset_analytics_map_idx();

    Ok("✅ Analytics data cleared".to_string())
}