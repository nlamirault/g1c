use std::io;
use thiserror::Error;

/// Custom error types for GCI application
#[derive(Error, Debug)]
pub enum GciError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}

impl From<serde_json::Error> for GciError {
    fn from(err: serde_json::Error) -> Self {
        GciError::Config(format!("JSON error: {}", err))
    }
}

