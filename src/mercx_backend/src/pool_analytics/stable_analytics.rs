use candid::{CandidType};
use serde::{Deserialize, Serialize};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;
use ic_cdk::api::time; //icp time

const MAX_DAYS_HISTORY: usize = 30;
const NANOSECONDS_PER_DAY: u64 = 24 * 60 * 60 * 1_000_000_000;
const CAIRO_TIMEZONE_OFFSET_NANOS: u64 = 3 * 60 * 60 * 1_000_000_000; // +3 hours for Cairo (UTC+3)


#[derive(CandidType, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PoolAnalyticsId(pub u32);

impl Storable for PoolAnalyticsId {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct PoolSnapshot {
    pub timestamp: u64,
    pub day_number: u64,
    pub pool_id: u32,
    pub tvl_usd: f64,
    pub volume_24h_usd: f64,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct StablePoolTimeSeries {
    pub pool_id: u32,
    pub snapshots: Vec<PoolSnapshot>,
    pub last_updated: u64,
}

impl Storable for StablePoolTimeSeries {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl StablePoolTimeSeries {
    pub fn new(pool_id: u32) -> Self {
        Self {
            pool_id,
            snapshots: Vec::new(),
            last_updated: 0,
        }
    }

    // Convert UTC timestamp to Cairo timezone day number
    fn get_day_number(timestamp: u64) -> u64 {
        let cairo_time = timestamp.saturating_add(CAIRO_TIMEZONE_OFFSET_NANOS);
        cairo_time / NANOSECONDS_PER_DAY
    }

    pub fn add_snapshot(&mut self, tvl_usd: f64, volume_24h_usd: f64) {
        let current_time = time();
        let current_day = Self::get_day_number(current_time);
        
        // Remove snapshots older than 30 days (Cairo timezone)
        let cutoff_day = current_day.saturating_sub(MAX_DAYS_HISTORY as u64);
        self.snapshots.retain(|s| s.day_number >= cutoff_day);
        
        // Check if we already have a snapshot for today (Cairo timezone)
        if let Some(today_snapshot) = self.snapshots.iter_mut().find(|s| s.day_number == current_day) {
            // Update existing snapshot for today
            today_snapshot.tvl_usd = tvl_usd;
            today_snapshot.volume_24h_usd = volume_24h_usd;
            today_snapshot.timestamp = current_time;
        } else {
            // Create new snapshot for a new day
            let snapshot = PoolSnapshot {
                timestamp: current_time,
                day_number: current_day,
                pool_id: self.pool_id,
                tvl_usd,
                volume_24h_usd,
            };
            self.snapshots.push(snapshot);
            
            // Keep sorted by day_number
            self.snapshots.sort_by_key(|s| s.day_number);
        }
        
        self.last_updated = current_time;
    }

    pub fn get_chart_data(&self, hours: u64) -> Vec<(u64, f64, f64)> {
        let cutoff_time = time().saturating_sub(hours * 60 * 60 * 1_000_000_000);
        self.snapshots
            .iter()
            .filter(|snapshot| snapshot.timestamp >= cutoff_time)
            .map(|s| (s.timestamp, s.tvl_usd, s.volume_24h_usd))
            .collect()
    }

    pub fn get_daily_chart_data(&self, days: u64) -> Vec<(u64, u64, f64, f64)> {
        let current_day = Self::get_day_number(time());
        let cutoff_day = current_day.saturating_sub(days);
        
        self.snapshots
            .iter()
            .filter(|s| s.day_number >= cutoff_day)
            .map(|s| (s.timestamp, s.day_number, s.tvl_usd, s.volume_24h_usd))
            .collect()
    }

    pub fn get_all_snapshots(&self) -> Vec<PoolSnapshot> {
        self.snapshots.clone()
    }
}