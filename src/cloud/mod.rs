mod auth;
mod instances;

use anyhow::{Context, Result};
use tracing::{debug, info};

use crate::config::Config;

pub use self::auth::get_gcloud_version;
pub use self::instances::Instance;

/// Google Cloud API client
pub struct CloudClient {
    /// Project ID
    project_id: String,
    /// Default region
    region: String,
    /// Whether to format output as JSON
    json_output: bool,
}

impl CloudClient {
    /// Create a new Cloud API client
    pub async fn new(config: &Config) -> Result<Self> {
        // Get project ID from config or gcloud
        let project_id = match &config.project {
            Some(project) => project.clone(),
            None => {
                info!("No project ID specified, trying to detect from gcloud config");
                auth::get_default_project().context("Failed to get default project")?
            }
        };

        // Get region from config or use default
        let region = config.region.clone().unwrap_or_else(|| {
            info!("No region specified, using us-central1");
            "us-central1".to_string()
        });

        debug!(
            "Initialized CloudClient with project={}, region={}",
            project_id, region
        );

        Ok(Self {
            project_id,
            region,
            json_output: true,
        })
    }

    /// List instances in the project
    pub async fn list_instances(&self) -> Result<Vec<Instance>> {
        instances::list_instances(&self.project_id, self.json_output).await
    }

    /// Start an instance
    pub async fn start_instance(&self, instance_id: &str) -> Result<()> {
        instances::start_instance(&self.project_id, instance_id).await
    }

    /// Stop an instance
    pub async fn stop_instance(&self, instance_id: &str) -> Result<()> {
        instances::stop_instance(&self.project_id, instance_id).await
    }

    /// Restart an instance
    pub async fn restart_instance(&self, instance_id: &str) -> Result<()> {
        instances::restart_instance(&self.project_id, instance_id).await
    }

    /// Delete an instance
    pub async fn delete_instance(&self, instance_id: &str) -> Result<()> {
        instances::delete_instance(&self.project_id, instance_id).await
    }

    /// Get detailed information about an instance
    pub async fn get_instance(&self, instance_id: &str) -> Result<Instance> {
        instances::get_instance(&self.project_id, instance_id, self.json_output).await
    }

    /// Get the region for this client
    pub fn get_region(&self) -> &str {
        &self.region
    }

    /// Get the project ID for this client
    pub fn get_project_id(&self) -> &str {
        &self.project_id
    }

    /// Get the gcloud CLI version
    pub fn get_cli_version(&self) -> Result<String> {
        auth::get_gcloud_version()
    }
}
