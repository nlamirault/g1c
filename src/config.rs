use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Application configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Google Cloud project ID
    pub project: Option<String>,

    /// Google Cloud region
    pub region: Option<String>,

    /// Refresh interval in seconds
    pub refresh_interval: u64,

    /// UI theme
    pub theme: String,

    /// Whether to use SSH for connecting to instances
    pub use_ssh: bool,

    /// Path to Google Cloud credentials file
    pub credentials_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: None,
            region: None,
            refresh_interval: 5,
            theme: "default".to_string(),
            use_ssh: true,
            credentials_path: None,
        }
    }
}

impl Config {
    /// Load configuration from default locations
    pub fn load(config_path: Option<&str>) -> Result<Self> {
        // If config path is provided, try to load from it
        if let Some(path) = config_path {
            return Self::load_from_file(path).context(format!(
                "Failed to load config from specified path: {}",
                path
            ));
        }

        // Otherwise try default locations
        if let Some(config_dir) = Self::config_dir() {
            let config_file = config_dir.join("config.toml");
            if config_file.exists() {
                return Self::load_from_file(&config_file).context(format!(
                    "Failed to load config from default path: {:?}",
                    config_file
                ));
            }
        }

        // If no config file exists, return default config
        info!("No configuration file found, using defaults");
        Ok(Config::default())
    }

    /// Load configuration from a specific file
    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_str = fs::read_to_string(&path)
            .context(format!("Failed to read config file: {:?}", path.as_ref()))?;

        let config: Config =
            toml::from_str(&config_str).context("Failed to parse config file as TOML")?;

        debug!("Loaded configuration from {:?}", path.as_ref());
        Ok(config)
    }

    /// Get the configuration directory
    pub fn config_dir() -> Option<PathBuf> {
        ProjectDirs::from("com", "g1c", "g1c").map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
    }

    /// Update config with a new project, if provided
    pub fn with_project(mut self, project: Option<String>) -> Self {
        if let Some(project) = project {
            self.project = Some(project);
        }
        self
    }

    /// Update config with a new region, if provided
    pub fn with_region(mut self, region: Option<String>) -> Self {
        if let Some(region) = region {
            self.region = Some(region);
        }
        self
    }

    /// Update config with a new refresh interval
    pub fn with_refresh_interval(mut self, interval: u64) -> Self {
        if interval > 0 {
            self.refresh_interval = interval;
        } else {
            warn!(
                "Invalid refresh interval provided: {}, using default",
                interval
            );
        }
        self
    }
}
