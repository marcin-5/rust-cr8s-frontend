pub mod crates;
pub mod rustaceans;
pub mod user;

use once_cell::sync::Lazy;
use std::env;

pub static API_URL: Lazy<String> =
    Lazy::new(|| env::var("API_URL").unwrap_or_else(|_| "http://localhost:8000".to_string()));
