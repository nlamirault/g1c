use std::io;
use thiserror::Error;

/// Custom error types for GCI application
#[derive(Error, Debug)]
pub enum GciError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Google Cloud API error: {0}")]
    CloudApi(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Instance not found: {0}")]
    InstanceNotFound(String),

    #[error("Operation timeout: {0}")]
    Timeout(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<serde_json::Error> for GciError {
    fn from(err: serde_json::Error) -> Self {
        GciError::Config(format!("JSON error: {}", err))
    }
}

/// Result type alias using our custom error
pub type GciResult<T> = Result<T, GciError>;