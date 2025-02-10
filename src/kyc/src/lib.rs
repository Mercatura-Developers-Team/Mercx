pub mod store;
pub mod models;
pub mod errors;
pub mod validations;
pub mod api;

use candid::Principal;

pub use models::user::*;

pub use validations::user::*;

pub use errors::general::*;
pub use errors::user::UserError;

pub use api::user::queries as user_queries;
pub use api::user::updates as user_updates;

ic_cdk::export_candid!();