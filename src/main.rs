use anyhow::Result;
use clap::Parser;
use tracing::{error, info};

mod app;
mod cloud;
mod config;
mod error;
mod logging;
mod ui;

use crate::app::App;
use crate::config::Config;

/// Terminal UI for monitoring Google Cloud Instances
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Google Cloud project ID
    #[arg(short, long)]
    project: Option<String>,

    /// Google Cloud region
    #[arg(short = 'g', long)]
    region: Option<String>,

    /// Auto-refresh interval in seconds
    #[arg(short, long, default_value_t = 5)]
    refresh: u64,

    /// Path to config file
    #[arg(short, long)]
    config: Option<String>,

    /// Log file path (enables file logging)
    #[arg(short, long)]
    log_file: Option<String>,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short = 'L', long, default_value = "info")]
    log_level: Option<String>,

    /// Log format (json or text)
    #[arg(short = 'F', long, default_value = "text")]
    log_format: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    logging::init(
        args.log_file.as_deref(),
        args.log_level.as_deref(),
        args.log_format.as_deref(),
    )?;
    info!("Application logging setup");

    // Load configuration
    let config = match Config::load(args.config.as_deref()) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            Config::default()
        }
    };

    // Override config with command line arguments
    let config = config
        .with_project(args.project)
        .with_region(args.region)
        .with_refresh_interval(args.refresh);

    // Setup terminal
    let mut terminal = ui::setup_terminal()?;

    // Create and run the application
    let result = App::new(config).await?.run(&mut terminal).await;

    // Restore terminal
    ui::restore_terminal()?;

    // Handle application result
    if let Err(err) = result {
        // Log to stderr via tracing
        error!("Application error: {}", err);

        // Also log directly to file if one is configured
        if let Some(log_file) = args.log_file {
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file)
            {
                // Format error message based on log format
                let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                let message = if args.log_format.as_deref() == Some("json") {
                    format!(
                        "{{\"timestamp\":\"{}\",\"level\":\"ERROR\",\"target\":\"g1c\",\"message\":\"Application error: {}\"}}\n",
                        now, err
                    )
                } else {
                    format!("[{} ERROR g1c] Application error: {}\n", now, err)
                };

                // Write directly to the log file
                use std::io::Write;
                let _ = file.write_all(message.as_bytes());
            }
        }

        return Err(err);
    }

    Ok(())
}
