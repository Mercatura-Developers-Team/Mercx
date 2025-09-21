pub mod analytics;
pub mod analytics_storage;
// use ic_cdk::export::candid::{CandidType, Deserialize};
// use serde::Serialize;
// use std::collections::HashMap;

// #[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
// pub struct PoolAnalytics {
//     pub total_volume: HashMap<String, f64>,
//     pub total_value_locked: HashMap<String, f64>,
// }

// impl PoolAnalytics {
//     pub fn new() -> Self {
//         Self {
//             total_volume: HashMap::new(),
//             total_value_locked: HashMap::new(),
//         }
//     }

//     pub fn update_volume(&mut self, pool_id: String, volume: f64) {
//         *self.total_volume.entry(pool_id).or_insert(0.0) += volume;
//     }

//     pub fn update_tvl(&mut self, pool_id: String, tvl: f64) {
//         self.total_value_locked.insert(pool_id, tvl);
//     }

//     pub fn get_volume(&self, pool_id: &str) -> f64 {
//         self.total_volume.get(pool_id).cloned().unwrap_or(0.0)
//     }

//     pub fn get_tvl(&self, pool_id: &str) -> f64 {
//         self.total_value_locked.get(pool_id).cloned().unwrap_or(0.0)
//     }
// }
