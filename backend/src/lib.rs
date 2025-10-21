
// backend/src/lib.rs
pub mod config;
mod middleware;
pub mod routes;
mod handlers;
mod services;
mod conversions;
mod error;

pub use config::server::run_server;
pub use config::AppState;