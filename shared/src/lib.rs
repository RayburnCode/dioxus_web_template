// shared/src/lib.rs
use serde::{Deserialize, Serialize};


pub mod models;

// API Error response for consistent error handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub details: Option<String>, // Additional error details
}

// API Response wrapper for consistent response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub message: Option<String>, // Optional success message
}

impl From<&str> for ApiError {
    fn from(error: &str) -> Self {
        ApiError {
            error: error.to_string(),
            details: None,
        }
    }
}

impl From<String> for ApiError {
    fn from(error: String) -> Self {
        ApiError {
            error,
            details: None,
        }
    }
}