use std::path::Path;
use anyhow::Result;
use tracing_subscriber::{self, prelude::*, filter::LevelFilter, Layer};

/// Log format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    /// Text format (human-readable)
    Text,
    /// JSON format (machine-readable)
    Json,
}

/// Initialize logging for the application
///
/// This function sets up tracing with different configurations:
/// - Console output limited to ERROR level to avoid interfering with the TUI
/// - Optional file logging with configurable level for debugging
/// - Selectable format (text or JSON) for log output
pub fn init(log_file: Option<&str>, log_level: Option<&str>, log_format: Option<&str>) -> Result<()> {
    // Parse the log level
    let level_str = log_level.unwrap_or("info");
    let level_filter = match level_str.to_lowercase().as_str() {
        "trace" => LevelFilter::TRACE,
        "debug" => LevelFilter::DEBUG, 
        "info" => LevelFilter::INFO,
        "warn" => LevelFilter::WARN,
        "error" => LevelFilter::ERROR,
        _ => LevelFilter::INFO,
    };

    // Determine if we should use JSON format
    let is_json = matches!(log_format, Some(format) if format.to_lowercase() == "json");

    // Initialize logging based on whether a log file was provided
    if let Some(log_path) = log_file {
        // Create parent directory if needed
        if let Some(parent) = Path::new(log_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Set up file logging
        let file = std::fs::File::create(log_path)?;
        
        if is_json {
            // Initialize JSON logging
            init_json_logging(file, level_filter);
        } else {
            // Initialize text logging
            init_text_logging(file, level_filter);
        }
    } else {
        // Console-only logging with ERROR level to avoid disrupting TUI
        init_console_logging();
    }

    // Log initialization message
    tracing::info!("Logging initialized with level {}", level_str);
    
    Ok(())
}

/// Initialize console-only logging with ERROR level
fn init_console_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(
                    tracing_subscriber::filter::EnvFilter::builder()
                        .with_default_directive(LevelFilter::ERROR.into())
                        .from_env_lossy()
                )
        )
        .init();
}

/// Initialize text format logging to file with console error logs
fn init_text_logging(file: std::fs::File, level: LevelFilter) {
    // Console layer with ERROR level only
    let console_layer = tracing_subscriber::fmt::layer()
        .with_filter(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(LevelFilter::ERROR.into())
                .from_env_lossy()
        );
    
    // File layer with user-specified level
    let file_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_ansi(false)
        .with_writer(file)
        .with_filter(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(level.into())
                .from_env_lossy()
        );
    
    // Register both layers
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();
}

/// Initialize JSON format logging to file with console error logs
fn init_json_logging(file: std::fs::File, level: LevelFilter) {
    // Console layer with ERROR level only
    let console_layer = tracing_subscriber::fmt::layer()
        .with_filter(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(LevelFilter::ERROR.into())
                .from_env_lossy()
        );
    
    // File layer with user-specified level in JSON format
    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_ansi(false)
        .with_writer(file)
        .with_filter(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(level.into())
                .from_env_lossy()
        );
    
    // Register both layers
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();
}