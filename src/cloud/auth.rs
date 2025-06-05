use anyhow::{Context, Result};
use std::process::Command;
use tracing::{debug, info, warn};

/// Get the default project ID from gcloud config
pub fn get_default_project() -> Result<String> {
    // Run gcloud config get-value project command
    let output = Command::new("gcloud")
        .args(["config", "get-value", "project"])
        .output()
        .context("Failed to execute gcloud config get-value project command")?;
    
    // Check if command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to get default project: {}", error));
    }
    
    // Parse output
    let project_id = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();
    
    // Check if project ID is empty
    if project_id.is_empty() {
        return Err(anyhow::anyhow!("No default project found. Please set a default project with 'gcloud config set project PROJECT_ID' or specify a project with --project"));
    }
    
    debug!("Default project: {}", project_id);
    Ok(project_id)
}

/// Check if gcloud CLI is installed and authenticated
pub fn check_gcloud() -> Result<()> {
    // Check if gcloud is installed
    let output = Command::new("gcloud")
        .arg("--version")
        .output()
        .context("Failed to execute gcloud --version command. Is gcloud CLI installed?")?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("gcloud CLI check failed: {}", error));
    }
    
    // Check if user is authenticated
    let output = Command::new("gcloud")
        .args(["auth", "list"])
        .output()
        .context("Failed to execute gcloud auth list command")?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Authentication check failed: {}", error));
    }
    
    let auth_output = String::from_utf8_lossy(&output.stdout);
    if !auth_output.contains("*") {
        warn!("No active gcloud account found. Please run 'gcloud auth login' to authenticate");
        return Err(anyhow::anyhow!("No active gcloud account found"));
    }
    
    info!("gcloud CLI is installed and authenticated");
    Ok(())
}

/// Check if user has compute API access
pub fn check_compute_api(project_id: &str) -> Result<()> {
    // Check if compute API is enabled
    let output = Command::new("gcloud")
        .args([
            "services", "list",
            "--project", project_id,
            "--filter", "name:compute.googleapis.com",
            "--format", "value(state)",
        ])
        .output()
        .context("Failed to check compute API status")?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to check compute API status: {}", error));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let api_state = stdout.trim();
    
    if api_state != "ENABLED" {
        warn!("Compute Engine API is not enabled for project {}", project_id);
        return Err(anyhow::anyhow!("Compute Engine API is not enabled for project {}. Please enable it at https://console.cloud.google.com/apis/library/compute.googleapis.com", project_id));
    }
    
    info!("Compute Engine API is enabled for project {}", project_id);
    Ok(())
}